//! The keyboard's side of the protocol: ZMK v0.3's Studio request handlers, as a state
//! machine with no I/O and no clock.
//!
//! [`Keyboard::handle`] answers one request the way ZMK v0.3 does (`app/src/studio/`), with
//! the notifications it sends first. It follows the firmware, quirks included, so code
//! tested against it copes with real keyboards:
//!
//! - **Locking.** The keyboard starts locked. Device info, the lock state and the behavior
//!   list are unsecured; every other request gets `UNLOCK_REQUIRED` while locked. Locking
//!   by request isn't implemented, so `lock` gets `RPC_NOT_FOUND`. Unlocking is a key
//!   press ([`Keyboard::unlock`]), and locking happens on its own ([`Keyboard::lock`]).
//! - **Layer IDs and indexes.** A layer's ID is its slot. Removing a layer only takes it
//!   out of the order, so its slot keeps its name and bindings, and restoring brings them
//!   back. Adding takes the free slot with the lowest ID. ZMK checks little: restoring a
//!   layer twice puts it in the order twice, and keys can be set on any slot.
//! - **Unsaved changes.** Every change sends "unsaved changes: yes", even one that changes
//!   nothing, and saving or discarding sends "no". [`KeymapRequest::CheckUnsavedChanges`]
//!   ignores renamed layers, and discarding doesn't undo a rename the settings don't hold.
//! - **Settings reset** goes back to the stock bindings and order, but keeps the names, and
//!   what counts as changed, until the keyboard restarts.
//! - **Request fields** are narrowed as ZMK's C code does: layer IDs, indexes and key
//!   positions to a byte, behavior IDs to 16 bits. A layer name of 20 bytes or more can't
//!   be decoded, so the keyboard doesn't answer at all.
//! - **Binding parameters** must match the behavior's metadata. A layer parameter is
//!   checked against the number of keys, not of layers, as in v0.3, and a behavior without
//!   metadata, such as External Power, takes no binding at all.
//!
//! The [`Simulator`](crate::Simulator) adds what needs a clock or a connection: the idle
//! lock, latency and other faults.

mod keymap;

#[cfg(test)]
mod tests;

use std::sync::Arc;
use std::time::Duration;

use keywright_studio::message::{
    AddLayerError, AddedLayer, BehaviorsRequest, BehaviorsResponse, Binding, ClientMessage,
    CoreNotification, CoreRequest, CoreResponse, DeviceMessage, ErrorCondition, Keymap,
    KeymapNotification, KeymapRequest, KeymapResponse, LockState, MoveLayerError, Notification,
    ParameterValue, PhysicalLayouts, RemoveLayerError, Request, Response, RestoreLayerError,
    SetActivePhysicalLayoutError, SetLayerBindingError, SetLayerPropsError, ValueKind,
};

use self::keymap::Errno;
use crate::firmware::{Firmware, FirmwareBehavior};

/// How long the keyboard stays unlocked without a request: ZMK's default
/// `CONFIG_ZMK_STUDIO_LOCK_IDLE_TIMEOUT_SEC`.
pub const IDLE_LOCK_TIMEOUT: Duration = Duration::from_secs(600);

/// The longest layer name ZMK v0.3 stores, in bytes, counting the C string's final NUL:
/// `CONFIG_ZMK_KEYMAP_LAYER_NAME_MAX_LEN`. The keyboard reports it as
/// [`Keymap::max_layer_name_length`].
pub const MAX_LAYER_NAME_LEN: usize = 20;

/// A simulated ZMK v0.3 keyboard.
#[derive(Debug, Clone)]
pub struct Keyboard {
    firmware: Arc<Firmware>,
    lock_state: LockState,
    keymap: keymap::Keymap,
}

impl Keyboard {
    /// A keyboard with this firmware and empty settings, as it leaves the factory. It
    /// starts locked.
    #[must_use]
    pub fn new(firmware: Firmware) -> Self {
        let keymap = keymap::Keymap::new(&firmware);
        Self {
            firmware: Arc::new(firmware),
            lock_state: LockState::Locked,
            keymap,
        }
    }

    /// The firmware the keyboard runs.
    #[must_use]
    pub fn firmware(&self) -> &Firmware {
        &self.firmware
    }

    /// Whether the keyboard is locked.
    #[must_use]
    pub fn lock_state(&self) -> LockState {
        self.lock_state
    }

    /// The keymap, as the keyboard would report it.
    #[must_use]
    pub fn keymap(&self) -> Keymap {
        Keymap {
            layers: self.keymap.layers(),
            available_layers: self.keymap.available_layers(),
            max_layer_name_length: u32::try_from(MAX_LAYER_NAME_LEN).unwrap_or(u32::MAX),
        }
    }

    /// Whether the keyboard would report unsaved changes.
    #[must_use]
    pub fn has_unsaved_changes(&self) -> bool {
        self.keymap.has_unsaved_changes()
    }

    /// Someone presses the unlock key (`&studio_unlock`). Returns the notification the
    /// keyboard sends, if it was locked.
    pub fn unlock(&mut self) -> Option<DeviceMessage> {
        self.set_lock_state(LockState::Unlocked)
    }

    /// The keyboard locks itself: after being idle, or when its output moves to another
    /// transport. Returns the notification it sends, if it was unlocked.
    pub fn lock(&mut self) -> Option<DeviceMessage> {
        self.set_lock_state(LockState::Locked)
    }

    /// The keyboard restarts: it forgets unsaved changes, and locks. It sends nothing, as
    /// the connection drops.
    pub fn restart(&mut self) {
        self.keymap.restart();
        self.lock_state = LockState::Locked;
    }

    /// Answers a request: the notifications the keyboard sends, then the response. Returns
    /// `None` if ZMK couldn't decode the request, and so wouldn't answer at all.
    pub fn handle(&mut self, message: &ClientMessage) -> Option<Vec<DeviceMessage>> {
        if let Request::Keymap(KeymapRequest::SetLayerProps { name, .. }) = &message.request
            && name.len() >= MAX_LAYER_NAME_LEN
        {
            return None;
        }
        let mut notifications = Vec::new();
        let response = if secured(&message.request) && self.lock_state == LockState::Locked {
            Response::Error(ErrorCondition::UnlockRequired)
        } else {
            match &message.request {
                Request::Core(request) => self.core(*request),
                Request::Behaviors(request) => self.behaviors(*request),
                Request::Keymap(request) => {
                    let (response, unsaved) = self.keymap_request(request);
                    notifications.extend(unsaved.map(|unsaved_changes| {
                        DeviceMessage::Notification(Notification::Keymap(
                            KeymapNotification::UnsavedChangesStatusChanged { unsaved_changes },
                        ))
                    }));
                    Response::Keymap(response)
                }
            }
        };
        notifications.push(DeviceMessage::Response {
            request_id: message.request_id,
            response,
        });
        Some(notifications)
    }

    fn set_lock_state(&mut self, state: LockState) -> Option<DeviceMessage> {
        if self.lock_state == state {
            return None;
        }
        self.lock_state = state;
        Some(DeviceMessage::Notification(Notification::Core(
            CoreNotification::LockStateChanged(state),
        )))
    }

    fn core(&mut self, request: CoreRequest) -> Response {
        match request {
            CoreRequest::GetDeviceInfo => Response::Core(CoreResponse::GetDeviceInfo(
                self.firmware.device_info.clone(),
            )),
            CoreRequest::GetLockState => {
                Response::Core(CoreResponse::GetLockState(self.lock_state))
            }
            CoreRequest::ResetSettings => {
                self.keymap.reset_settings();
                Response::Core(CoreResponse::ResetSettings { succeeded: true })
            }
            // ZMK v0.3 has no handler for it.
            CoreRequest::Lock => Response::Error(ErrorCondition::RpcNotFound),
        }
    }

    fn behaviors(&self, request: BehaviorsRequest) -> Response {
        match request {
            BehaviorsRequest::ListAllBehaviors => {
                Response::Behaviors(BehaviorsResponse::ListAllBehaviors(
                    self.firmware
                        .behaviors
                        .iter()
                        .map(|behavior| behavior.details.id)
                        .collect(),
                ))
            }
            BehaviorsRequest::GetBehaviorDetails { behavior_id } => {
                match self.behavior(behavior_id.0) {
                    Some(behavior) => Response::Behaviors(BehaviorsResponse::GetBehaviorDetails(
                        behavior.details.clone(),
                    )),
                    None => Response::Error(ErrorCondition::Generic),
                }
            }
        }
    }

    /// Answers a keymap request, and says what to send about unsaved changes, if anything.
    fn keymap_request(&mut self, request: &KeymapRequest) -> (KeymapResponse, Option<bool>) {
        match request {
            KeymapRequest::GetKeymap => (KeymapResponse::GetKeymap(self.keymap()), None),
            KeymapRequest::SetLayerBinding {
                layer_id,
                key_position,
                binding,
            } => changed(
                self.set_binding(layer_id.0, *key_position, *binding),
                KeymapResponse::SetLayerBinding,
            ),
            KeymapRequest::CheckUnsavedChanges => {
                let unsaved_changes = self.keymap.has_unsaved_changes();
                (
                    KeymapResponse::CheckUnsavedChanges { unsaved_changes },
                    None,
                )
            }
            KeymapRequest::SaveChanges => {
                self.keymap.save();
                (KeymapResponse::SaveChanges(Ok(())), Some(false))
            }
            KeymapRequest::DiscardChanges => {
                self.keymap.discard();
                (KeymapResponse::DiscardChanges, Some(false))
            }
            KeymapRequest::GetPhysicalLayouts => {
                let layouts = PhysicalLayouts {
                    active_layout_index: 0,
                    layouts: vec![self.firmware.layout.clone()],
                };
                (KeymapResponse::GetPhysicalLayouts(layouts), None)
            }
            KeymapRequest::SetActivePhysicalLayout { index } => {
                // The keyboard has one layout, which is active. Selecting another fails,
                // yet still reports unsaved changes, as ZMK v0.3 does.
                if byte(*index) == 0 {
                    let keymap = Ok(self.keymap());
                    (KeymapResponse::SetActivePhysicalLayout(keymap), None)
                } else {
                    let error = Err(SetActivePhysicalLayoutError::Generic);
                    (KeymapResponse::SetActivePhysicalLayout(error), Some(true))
                }
            }
            KeymapRequest::MoveLayer {
                start_index,
                dest_index,
            } => {
                let moved = self
                    .keymap
                    .move_layer(byte(*start_index), byte(*dest_index));
                let result = moved
                    .map(|()| self.keymap())
                    .map_err(|_| MoveLayerError::Generic);
                changed(result, KeymapResponse::MoveLayer)
            }
            KeymapRequest::AddLayer => {
                let added = self.keymap.add_layer();
                let result = added
                    .map(|(index, id)| AddedLayer {
                        index: u32::from(index),
                        layer: self.keymap.layer(id),
                    })
                    .map_err(|_| AddLayerError::NoSpace);
                changed(result, KeymapResponse::AddLayer)
            }
            KeymapRequest::RemoveLayer { layer_index } => {
                let result = self.keymap.remove_layer(byte(*layer_index));
                let result = result.map_err(|_| RemoveLayerError::InvalidIndex);
                changed(result, KeymapResponse::RemoveLayer)
            }
            KeymapRequest::RestoreLayer { layer_id, at_index } => {
                let id = byte(layer_id.0);
                let restored = self.keymap.restore_layer(id, byte(*at_index));
                let result =
                    restored
                        .map(|()| self.keymap.layer(id))
                        .map_err(|errno| match errno {
                            Errno::Invalid => RestoreLayerError::InvalidIndex,
                            _ => RestoreLayerError::Generic,
                        });
                changed(result, KeymapResponse::RestoreLayer)
            }
            // An empty name changes nothing, and succeeds.
            KeymapRequest::SetLayerProps { name, .. } if name.is_empty() => {
                (KeymapResponse::SetLayerProps(Ok(())), None)
            }
            KeymapRequest::SetLayerProps { layer_id, name } => {
                let result = self.keymap.rename(byte(layer_id.0), name);
                let result = result.map_err(|_| SetLayerPropsError::InvalidId);
                changed(result, KeymapResponse::SetLayerProps)
            }
        }
    }

    fn set_binding(
        &mut self,
        layer_id: u32,
        key_position: i32,
        binding: Binding,
    ) -> Result<(), SetLayerBindingError> {
        let behavior = self
            .behavior(binding.behavior_id.0)
            .ok_or(SetLayerBindingError::InvalidBehavior)?;
        let id = behavior.details.id;
        if !accepts(
            behavior,
            binding.param1,
            binding.param2,
            self.firmware.key_count(),
        ) {
            return Err(SetLayerBindingError::InvalidParameters);
        }
        let binding = Binding {
            behavior_id: id,
            ..binding
        };
        let [position, ..] = key_position.to_le_bytes();
        self.keymap
            .set_binding(byte(layer_id), position, binding)
            .map_err(|_| SetLayerBindingError::InvalidLocation)
    }

    /// The behavior with this ID, which ZMK narrows to 16 bits.
    fn behavior(&self, id: u32) -> Option<&FirmwareBehavior> {
        let [low, high, ..] = id.to_le_bytes();
        let id = u32::from(u16::from_le_bytes([low, high]));
        self.firmware
            .behaviors
            .iter()
            .find(|behavior| behavior.details.id.0 == id)
    }
}

/// The response to a change, and "unsaved changes: yes" if it was made.
fn changed<T, E>(
    result: Result<T, E>,
    response: fn(Result<T, E>) -> KeymapResponse,
) -> (KeymapResponse, Option<bool>) {
    let made = result.is_ok();
    (response(result), made.then_some(true))
}

/// Whether a request needs the keyboard unlocked.
fn secured(request: &Request) -> bool {
    match request {
        Request::Core(request) => *request == CoreRequest::ResetSettings,
        Request::Behaviors(request) => *request != BehaviorsRequest::ListAllBehaviors,
        Request::Keymap(_) => true,
    }
}

/// A request field narrowed to a byte, as ZMK's `uint8_t` layer IDs and indexes are.
fn byte(value: u32) -> u8 {
    value.to_le_bytes()[0]
}

/// Whether a behavior takes these parameters (`zmk_behavior_validate_binding`).
fn accepts(behavior: &FirmwareBehavior, param1: u32, param2: u32, key_count: usize) -> bool {
    if !behavior.has_metadata {
        return false;
    }
    let sets = &behavior.details.metadata;
    if sets.is_empty() {
        return param1 == 0 && param2 == 0;
    }
    let matches = |values: &[ParameterValue], param: u32| {
        // A parameter with no values must be 0.
        (values.is_empty() && param == 0)
            || values
                .iter()
                .any(|value| value_matches(value.kind, param, key_count))
    };
    sets.iter()
        .any(|set| matches(&set.param1, param1) && matches(&set.param2, param2))
}

fn value_matches(kind: ValueKind, param: u32, key_count: usize) -> bool {
    match kind {
        ValueKind::Nil => param == 0,
        ValueKind::Constant(value) => param == value,
        // C compares the signed limits as unsigned.
        ValueKind::Range { min, max } => {
            (min.cast_unsigned()..=max.cast_unsigned()).contains(&param)
        }
        ValueKind::HidUsage { .. } => {
            let [id_low, id_high, page, _mods] = param.to_le_bytes();
            let id = u16::from_le_bytes([id_low, id_high]);
            // ZMK v0.3 refuses only usage 0 on the keyboard page, and consumer usages past
            // 0xFFF.
            match page {
                0x07 => id != 0,
                0x0C => id <= 0xFFF,
                _ => false,
            }
        }
        // v0.3 checks a layer against the number of keys, not of layers.
        ValueKind::LayerId => usize::try_from(param).is_ok_and(|layer| layer < key_count),
    }
}
