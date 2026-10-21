//! The `keymap` subsystem (`keymap.proto`): layers and their bindings, saving and
//! discarding changes, and physical layouts.

use super::proto::keymap as wire;
use super::{BehaviorId, DecodeError, WireEnum, decode_enum, encode_enum, required};
use wire::notification::NotificationType;
use wire::request::RequestType;
use wire::response::ResponseType;

/// A layer's ID.
///
/// A layer keeps its ID when layers move, while its index changes. Layer keys refer to
/// layers by ID, so moving layers doesn't break them (tech spec §8.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct LayerId(pub u32);

/// A request to the `keymap` subsystem. They all need the keyboard unlocked.
///
/// Changes are unsaved until [`SaveChanges`](Self::SaveChanges): the keyboard forgets them
/// when it restarts.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum KeymapRequest {
    /// Every layer and its bindings.
    GetKeymap,
    /// Assigns a binding to a key on a layer.
    SetLayerBinding {
        /// The layer.
        layer_id: LayerId,
        /// The key: its index in [`Layer::bindings`].
        key_position: i32,
        /// What the key does.
        binding: Binding,
    },
    /// Whether the keymap or the physical layout has unsaved changes.
    CheckUnsavedChanges,
    /// Saves the changes to the keyboard's flash.
    SaveChanges,
    /// Reverts the changes made since the last save.
    DiscardChanges,
    /// The physical layouts the keyboard supports, and which one is active.
    GetPhysicalLayouts,
    /// Makes a physical layout active.
    SetActivePhysicalLayout {
        /// The layout's index in [`PhysicalLayouts::layouts`].
        index: u32,
    },
    /// Moves a layer to another index.
    MoveLayer {
        /// The layer's index now.
        start_index: u32,
        /// The index it moves to.
        dest_index: u32,
    },
    /// Adds a layer at the end, in a free slot.
    AddLayer,
    /// Removes a layer. Its slot keeps the bindings, so
    /// [`RestoreLayer`](Self::RestoreLayer) can bring it back.
    RemoveLayer {
        /// The layer's index.
        layer_index: u32,
    },
    /// Brings back a removed layer.
    RestoreLayer {
        /// The removed layer.
        layer_id: LayerId,
        /// The index to put it at.
        at_index: u32,
    },
    /// Renames a layer.
    ///
    /// The keyboard can't decode a name as long as [`Keymap::max_layer_name_length`] or
    /// longer, and then doesn't respond at all (tech spec §5.3).
    SetLayerProps {
        /// The layer.
        layer_id: LayerId,
        /// Its new name.
        name: String,
    },
}

/// The answer to a [`KeymapRequest`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum KeymapResponse {
    /// The answer to [`KeymapRequest::GetKeymap`].
    GetKeymap(Keymap),
    /// The answer to [`KeymapRequest::SetLayerBinding`].
    SetLayerBinding(Result<(), SetLayerBindingError>),
    /// The answer to [`KeymapRequest::CheckUnsavedChanges`].
    CheckUnsavedChanges {
        /// Whether there are unsaved changes.
        unsaved_changes: bool,
    },
    /// The answer to [`KeymapRequest::SaveChanges`].
    SaveChanges(Result<(), SaveChangesError>),
    /// The answer to [`KeymapRequest::DiscardChanges`] when it succeeds. When it fails, the
    /// keyboard answers with [`Response::Error`](super::Response::Error) instead.
    DiscardChanges,
    /// The answer to [`KeymapRequest::GetPhysicalLayouts`].
    GetPhysicalLayouts(PhysicalLayouts),
    /// The answer to [`KeymapRequest::SetActivePhysicalLayout`]: the keymap for the new
    /// layout.
    SetActivePhysicalLayout(Result<Keymap, SetActivePhysicalLayoutError>),
    /// The answer to [`KeymapRequest::MoveLayer`]: the keymap with its layers in their new
    /// order.
    MoveLayer(Result<Keymap, MoveLayerError>),
    /// The answer to [`KeymapRequest::AddLayer`].
    AddLayer(Result<AddedLayer, AddLayerError>),
    /// The answer to [`KeymapRequest::RemoveLayer`].
    RemoveLayer(Result<(), RemoveLayerError>),
    /// The answer to [`KeymapRequest::RestoreLayer`]: the restored layer.
    RestoreLayer(Result<Layer, RestoreLayerError>),
    /// The answer to [`KeymapRequest::SetLayerProps`].
    SetLayerProps(Result<(), SetLayerPropsError>),
}

/// A change in the `keymap` subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum KeymapNotification {
    /// Changes were made, saved or discarded.
    UnsavedChangesStatusChanged {
        /// Whether there are unsaved changes now.
        unsaved_changes: bool,
    },
}

/// The keyboard's layers.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Keymap {
    /// The layers in order: index 0 is the base layer, and a higher layer takes priority.
    #[cfg_attr(test, proptest(strategy = "super::tests::short_vec()"))]
    pub layers: Vec<Layer>,
    /// How many more layers fit: the number of free slots.
    pub available_layers: u32,
    /// The size of the firmware's buffer for a layer name. It has room for C's terminating
    /// NUL, so names can be at most one byte shorter.
    pub max_layer_name_length: u32,
}

/// One layer of the keymap.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Layer {
    /// The layer's ID.
    pub id: LayerId,
    /// The layer's name. It can be empty.
    pub name: String,
    /// What each key does, by key position.
    pub bindings: Vec<Binding>,
}

/// What a key does: a behavior and its two parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Binding {
    /// The behavior, such as key press or layer tap.
    pub behavior_id: BehaviorId,
    /// The first parameter, such as the key to press.
    pub param1: u32,
    /// The second parameter, such as the key to tap in a mod-tap.
    pub param2: u32,
}

/// A layer that [`KeymapRequest::AddLayer`] added.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct AddedLayer {
    /// Its index.
    pub index: u32,
    /// The layer.
    pub layer: Layer,
}

/// The physical layouts a keyboard supports.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct PhysicalLayouts {
    /// The index of the active layout in `layouts`.
    pub active_layout_index: u32,
    /// The layouts.
    #[cfg_attr(test, proptest(strategy = "super::tests::short_vec()"))]
    pub layouts: Vec<PhysicalLayout>,
}

/// Where the keys are on the keyboard.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct PhysicalLayout {
    /// The layout's name. It can be empty.
    pub name: String,
    /// Each key's place and size, by key position.
    pub keys: Vec<KeyPhysicalAttrs>,
}

/// A key's place, size and rotation.
///
/// Lengths are in hundredths of a key unit (`100` is one standard key wide), and angles in
/// hundredths of a degree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct KeyPhysicalAttrs {
    /// The key's width.
    pub width: i32,
    /// The key's height.
    pub height: i32,
    /// The left edge.
    pub x: i32,
    /// The top edge.
    pub y: i32,
    /// The rotation, clockwise.
    pub r: i32,
    /// The x coordinate of the point the key rotates around.
    pub rx: i32,
    /// The y coordinate of the point the key rotates around.
    pub ry: i32,
}

/// Why the keyboard didn't set a binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum SetLayerBindingError {
    /// There is no such layer, or no such key position.
    #[error("there is no such layer or key position")]
    InvalidLocation,
    /// The keyboard has no behavior with this ID.
    #[error("the keyboard has no such behavior")]
    InvalidBehavior,
    /// The behavior doesn't accept these parameters.
    #[error("the behavior doesn't accept these parameters")]
    InvalidParameters,
}

/// Why the keyboard didn't save the changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum SaveChangesError {
    /// Something else went wrong.
    #[error("the keyboard couldn't save the changes")]
    Generic,
    /// The firmware can't save settings.
    #[error("the keyboard can't save settings")]
    NotSupported,
    /// The keyboard's settings storage is full.
    #[error("the keyboard has no space left to save the changes")]
    NoSpace,
}

/// Why the keyboard didn't switch physical layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum SetActivePhysicalLayoutError {
    /// Something else went wrong.
    #[error("the keyboard couldn't switch physical layouts")]
    Generic,
    /// There is no layout at this index.
    #[error("there is no physical layout at this index")]
    InvalidLayoutIndex,
}

/// Why the keyboard didn't move a layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum MoveLayerError {
    /// Something else went wrong.
    #[error("the keyboard couldn't move the layer")]
    Generic,
    /// There is no layer at the start index.
    #[error("there is no layer at this index")]
    InvalidLayer,
    /// The destination index is out of range.
    #[error("a layer can't move to this index")]
    InvalidDestination,
}

/// Why the keyboard didn't add a layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum AddLayerError {
    /// Something else went wrong.
    #[error("the keyboard couldn't add a layer")]
    Generic,
    /// Every layer slot is in use.
    #[error("the keyboard has no free layer slots")]
    NoSpace,
}

/// Why the keyboard didn't remove a layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum RemoveLayerError {
    /// Something else went wrong.
    #[error("the keyboard couldn't remove the layer")]
    Generic,
    /// There is no layer at this index.
    #[error("there is no layer at this index")]
    InvalidIndex,
}

/// Why the keyboard didn't restore a layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum RestoreLayerError {
    /// Something else went wrong.
    #[error("the keyboard couldn't restore the layer")]
    Generic,
    /// There is no removed layer with this ID.
    #[error("there is no removed layer with this ID")]
    InvalidId,
    /// A layer can't be restored at this index.
    #[error("a layer can't be restored at this index")]
    InvalidIndex,
}

/// Why the keyboard didn't rename a layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum SetLayerPropsError {
    /// Something else went wrong.
    #[error("the keyboard couldn't rename the layer")]
    Generic,
    /// There is no layer with this ID.
    #[error("there is no layer with this ID")]
    InvalidId,
}

impl From<&KeymapRequest> for wire::Request {
    fn from(request: &KeymapRequest) -> Self {
        let request_type = match request {
            KeymapRequest::GetKeymap => RequestType::GetKeymap(true),
            KeymapRequest::SetLayerBinding {
                layer_id,
                key_position,
                binding,
            } => RequestType::SetLayerBinding(wire::SetLayerBindingRequest {
                layer_id: layer_id.0,
                key_position: *key_position,
                binding: Some(binding.into()),
            }),
            KeymapRequest::CheckUnsavedChanges => RequestType::CheckUnsavedChanges(true),
            KeymapRequest::SaveChanges => RequestType::SaveChanges(true),
            KeymapRequest::DiscardChanges => RequestType::DiscardChanges(true),
            KeymapRequest::GetPhysicalLayouts => RequestType::GetPhysicalLayouts(true),
            KeymapRequest::SetActivePhysicalLayout { index } => {
                RequestType::SetActivePhysicalLayout(*index)
            }
            KeymapRequest::MoveLayer {
                start_index,
                dest_index,
            } => RequestType::MoveLayer(wire::MoveLayerRequest {
                start_index: *start_index,
                dest_index: *dest_index,
            }),
            KeymapRequest::AddLayer => RequestType::AddLayer(wire::AddLayerRequest {}),
            KeymapRequest::RemoveLayer { layer_index } => {
                RequestType::RemoveLayer(wire::RemoveLayerRequest {
                    layer_index: *layer_index,
                })
            }
            KeymapRequest::RestoreLayer { layer_id, at_index } => {
                RequestType::RestoreLayer(wire::RestoreLayerRequest {
                    layer_id: layer_id.0,
                    at_index: *at_index,
                })
            }
            KeymapRequest::SetLayerProps { layer_id, name } => {
                RequestType::SetLayerProps(wire::SetLayerPropsRequest {
                    layer_id: layer_id.0,
                    name: name.clone(),
                })
            }
        };
        Self {
            request_type: Some(request_type),
        }
    }
}

impl TryFrom<wire::Request> for KeymapRequest {
    type Error = DecodeError;

    fn try_from(request: wire::Request) -> Result<Self, DecodeError> {
        let request_type = required("keymap.Request.request_type", request.request_type)?;
        Ok(match request_type {
            RequestType::GetKeymap(_) => Self::GetKeymap,
            RequestType::SetLayerBinding(request) => {
                let binding = required("keymap.SetLayerBindingRequest.binding", request.binding)?;
                Self::SetLayerBinding {
                    layer_id: LayerId(request.layer_id),
                    key_position: request.key_position,
                    binding: binding.into(),
                }
            }
            RequestType::CheckUnsavedChanges(_) => Self::CheckUnsavedChanges,
            RequestType::SaveChanges(_) => Self::SaveChanges,
            RequestType::DiscardChanges(_) => Self::DiscardChanges,
            RequestType::GetPhysicalLayouts(_) => Self::GetPhysicalLayouts,
            RequestType::SetActivePhysicalLayout(index) => Self::SetActivePhysicalLayout { index },
            RequestType::MoveLayer(request) => Self::MoveLayer {
                start_index: request.start_index,
                dest_index: request.dest_index,
            },
            RequestType::AddLayer(_) => Self::AddLayer,
            RequestType::RemoveLayer(request) => Self::RemoveLayer {
                layer_index: request.layer_index,
            },
            RequestType::RestoreLayer(request) => Self::RestoreLayer {
                layer_id: LayerId(request.layer_id),
                at_index: request.at_index,
            },
            RequestType::SetLayerProps(request) => Self::SetLayerProps {
                layer_id: LayerId(request.layer_id),
                name: request.name,
            },
        })
    }
}

impl From<&KeymapResponse> for wire::Response {
    fn from(response: &KeymapResponse) -> Self {
        use wire::{
            add_layer_response, move_layer_response, remove_layer_response, restore_layer_response,
            save_changes_response, set_active_physical_layout_response,
        };

        let response_type = match response {
            KeymapResponse::GetKeymap(keymap) => ResponseType::GetKeymap(keymap.into()),
            KeymapResponse::SetLayerBinding(result) => {
                ResponseType::SetLayerBinding(encode_enum(*result))
            }
            KeymapResponse::CheckUnsavedChanges { unsaved_changes } => {
                ResponseType::CheckUnsavedChanges(*unsaved_changes)
            }
            KeymapResponse::SaveChanges(result) => {
                use save_changes_response::Result as Wire;
                ResponseType::SaveChanges(wire::SaveChangesResponse {
                    result: Some(match result {
                        Ok(()) => Wire::Ok(true),
                        Err(error) => Wire::Err(encode_enum(*error)),
                    }),
                })
            }
            KeymapResponse::DiscardChanges => ResponseType::DiscardChanges(true),
            KeymapResponse::GetPhysicalLayouts(layouts) => {
                ResponseType::GetPhysicalLayouts(layouts.into())
            }
            KeymapResponse::SetActivePhysicalLayout(result) => {
                use set_active_physical_layout_response::Result as Wire;
                ResponseType::SetActivePhysicalLayout(wire::SetActivePhysicalLayoutResponse {
                    result: Some(match result {
                        Ok(keymap) => Wire::Ok(keymap.into()),
                        Err(error) => Wire::Err(encode_enum(*error)),
                    }),
                })
            }
            KeymapResponse::MoveLayer(result) => {
                use move_layer_response::Result as Wire;
                ResponseType::MoveLayer(wire::MoveLayerResponse {
                    result: Some(match result {
                        Ok(keymap) => Wire::Ok(keymap.into()),
                        Err(error) => Wire::Err(encode_enum(*error)),
                    }),
                })
            }
            KeymapResponse::AddLayer(result) => {
                use add_layer_response::Result as Wire;
                ResponseType::AddLayer(wire::AddLayerResponse {
                    result: Some(match result {
                        Ok(added) => Wire::Ok(wire::AddLayerResponseDetails {
                            index: added.index,
                            layer: Some((&added.layer).into()),
                        }),
                        Err(error) => Wire::Err(encode_enum(*error)),
                    }),
                })
            }
            KeymapResponse::RemoveLayer(result) => {
                use remove_layer_response::Result as Wire;
                ResponseType::RemoveLayer(wire::RemoveLayerResponse {
                    result: Some(match result {
                        Ok(()) => Wire::Ok(wire::RemoveLayerOk {}),
                        Err(error) => Wire::Err(encode_enum(*error)),
                    }),
                })
            }
            KeymapResponse::RestoreLayer(result) => {
                use restore_layer_response::Result as Wire;
                ResponseType::RestoreLayer(wire::RestoreLayerResponse {
                    result: Some(match result {
                        Ok(layer) => Wire::Ok(layer.into()),
                        Err(error) => Wire::Err(encode_enum(*error)),
                    }),
                })
            }
            KeymapResponse::SetLayerProps(result) => {
                ResponseType::SetLayerProps(encode_enum(*result))
            }
        };
        Self {
            response_type: Some(response_type),
        }
    }
}

impl TryFrom<wire::Response> for KeymapResponse {
    type Error = DecodeError;

    fn try_from(response: wire::Response) -> Result<Self, DecodeError> {
        use wire::{
            add_layer_response, move_layer_response, remove_layer_response, restore_layer_response,
            save_changes_response, set_active_physical_layout_response,
        };

        let response_type = required("keymap.Response.response_type", response.response_type)?;
        Ok(match response_type {
            ResponseType::GetKeymap(keymap) => Self::GetKeymap(keymap.into()),
            ResponseType::SetLayerBinding(code) => {
                Self::SetLayerBinding(decode_enum("keymap.Response.set_layer_binding", code)?)
            }
            ResponseType::CheckUnsavedChanges(unsaved_changes) => {
                Self::CheckUnsavedChanges { unsaved_changes }
            }
            ResponseType::SaveChanges(response) => {
                use save_changes_response::Result as Wire;
                let result = required("keymap.SaveChangesResponse.result", response.result)?;
                Self::SaveChanges(match result {
                    Wire::Ok(_) => Ok(()),
                    Wire::Err(code) => Err(decode_enum("keymap.SaveChangesResponse.err", code)?),
                })
            }
            ResponseType::DiscardChanges(_) => Self::DiscardChanges,
            ResponseType::GetPhysicalLayouts(layouts) => Self::GetPhysicalLayouts(layouts.into()),
            ResponseType::SetActivePhysicalLayout(response) => {
                use set_active_physical_layout_response::Result as Wire;
                let result = required(
                    "keymap.SetActivePhysicalLayoutResponse.result",
                    response.result,
                )?;
                Self::SetActivePhysicalLayout(match result {
                    Wire::Ok(keymap) => Ok(keymap.into()),
                    Wire::Err(code) => Err(decode_enum(
                        "keymap.SetActivePhysicalLayoutResponse.err",
                        code,
                    )?),
                })
            }
            ResponseType::MoveLayer(response) => {
                use move_layer_response::Result as Wire;
                let result = required("keymap.MoveLayerResponse.result", response.result)?;
                Self::MoveLayer(match result {
                    Wire::Ok(keymap) => Ok(keymap.into()),
                    Wire::Err(code) => Err(decode_enum("keymap.MoveLayerResponse.err", code)?),
                })
            }
            ResponseType::AddLayer(response) => {
                use add_layer_response::Result as Wire;
                let result = required("keymap.AddLayerResponse.result", response.result)?;
                Self::AddLayer(match result {
                    Wire::Ok(added) => {
                        let layer = required("keymap.AddLayerResponseDetails.layer", added.layer)?;
                        Ok(AddedLayer {
                            index: added.index,
                            layer: layer.into(),
                        })
                    }
                    Wire::Err(code) => Err(decode_enum("keymap.AddLayerResponse.err", code)?),
                })
            }
            ResponseType::RemoveLayer(response) => {
                use remove_layer_response::Result as Wire;
                let result = required("keymap.RemoveLayerResponse.result", response.result)?;
                Self::RemoveLayer(match result {
                    Wire::Ok(_) => Ok(()),
                    Wire::Err(code) => Err(decode_enum("keymap.RemoveLayerResponse.err", code)?),
                })
            }
            ResponseType::RestoreLayer(response) => {
                use restore_layer_response::Result as Wire;
                let result = required("keymap.RestoreLayerResponse.result", response.result)?;
                Self::RestoreLayer(match result {
                    Wire::Ok(layer) => Ok(layer.into()),
                    Wire::Err(code) => Err(decode_enum("keymap.RestoreLayerResponse.err", code)?),
                })
            }
            ResponseType::SetLayerProps(code) => {
                Self::SetLayerProps(decode_enum("keymap.Response.set_layer_props", code)?)
            }
        })
    }
}

impl From<&KeymapNotification> for wire::Notification {
    fn from(notification: &KeymapNotification) -> Self {
        let notification_type = match notification {
            KeymapNotification::UnsavedChangesStatusChanged { unsaved_changes } => {
                NotificationType::UnsavedChangesStatusChanged(*unsaved_changes)
            }
        };
        Self {
            notification_type: Some(notification_type),
        }
    }
}

impl TryFrom<wire::Notification> for KeymapNotification {
    type Error = DecodeError;

    fn try_from(notification: wire::Notification) -> Result<Self, DecodeError> {
        let notification_type = required(
            "keymap.Notification.notification_type",
            notification.notification_type,
        )?;
        Ok(match notification_type {
            NotificationType::UnsavedChangesStatusChanged(unsaved_changes) => {
                Self::UnsavedChangesStatusChanged { unsaved_changes }
            }
        })
    }
}

impl From<&Keymap> for wire::Keymap {
    fn from(keymap: &Keymap) -> Self {
        Self {
            layers: keymap.layers.iter().map(Into::into).collect(),
            available_layers: keymap.available_layers,
            max_layer_name_length: keymap.max_layer_name_length,
        }
    }
}

impl From<wire::Keymap> for Keymap {
    fn from(keymap: wire::Keymap) -> Self {
        Self {
            layers: keymap.layers.into_iter().map(Into::into).collect(),
            available_layers: keymap.available_layers,
            max_layer_name_length: keymap.max_layer_name_length,
        }
    }
}

impl From<&Layer> for wire::Layer {
    fn from(layer: &Layer) -> Self {
        Self {
            id: layer.id.0,
            name: layer.name.clone(),
            bindings: layer.bindings.iter().map(Into::into).collect(),
        }
    }
}

impl From<wire::Layer> for Layer {
    fn from(layer: wire::Layer) -> Self {
        Self {
            id: LayerId(layer.id),
            name: layer.name,
            bindings: layer.bindings.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<&Binding> for wire::BehaviorBinding {
    fn from(binding: &Binding) -> Self {
        Self {
            // A `sint32` on the wire; see the module docs of `message`.
            behavior_id: binding.behavior_id.0.cast_signed(),
            param1: binding.param1,
            param2: binding.param2,
        }
    }
}

impl From<wire::BehaviorBinding> for Binding {
    fn from(binding: wire::BehaviorBinding) -> Self {
        Self {
            behavior_id: BehaviorId(binding.behavior_id.cast_unsigned()),
            param1: binding.param1,
            param2: binding.param2,
        }
    }
}

impl From<&PhysicalLayouts> for wire::PhysicalLayouts {
    fn from(layouts: &PhysicalLayouts) -> Self {
        Self {
            active_layout_index: layouts.active_layout_index,
            layouts: layouts.layouts.iter().map(Into::into).collect(),
        }
    }
}

impl From<wire::PhysicalLayouts> for PhysicalLayouts {
    fn from(layouts: wire::PhysicalLayouts) -> Self {
        Self {
            active_layout_index: layouts.active_layout_index,
            layouts: layouts.layouts.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<&PhysicalLayout> for wire::PhysicalLayout {
    fn from(layout: &PhysicalLayout) -> Self {
        Self {
            name: layout.name.clone(),
            keys: layout.keys.iter().map(Into::into).collect(),
        }
    }
}

impl From<wire::PhysicalLayout> for PhysicalLayout {
    fn from(layout: wire::PhysicalLayout) -> Self {
        Self {
            name: layout.name,
            keys: layout.keys.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<&KeyPhysicalAttrs> for wire::KeyPhysicalAttrs {
    fn from(key: &KeyPhysicalAttrs) -> Self {
        Self {
            width: key.width,
            height: key.height,
            x: key.x,
            y: key.y,
            r: key.r,
            rx: key.rx,
            ry: key.ry,
        }
    }
}

impl From<wire::KeyPhysicalAttrs> for KeyPhysicalAttrs {
    fn from(key: wire::KeyPhysicalAttrs) -> Self {
        Self {
            width: key.width,
            height: key.height,
            x: key.x,
            y: key.y,
            r: key.r,
            rx: key.rx,
            ry: key.ry,
        }
    }
}

// The error codes. Each `…_OK` code means "no error": it decodes as `Ok(())` where the code
// is the whole answer, and is invalid inside an `err` field.

impl WireEnum for Result<(), SetLayerBindingError> {
    type Wire = wire::SetLayerBindingResponse;

    fn from_wire(wire: Self::Wire) -> Option<Self> {
        Some(match wire {
            Self::Wire::SetLayerBindingRespOk => Ok(()),
            Self::Wire::SetLayerBindingRespInvalidLocation => {
                Err(SetLayerBindingError::InvalidLocation)
            }
            Self::Wire::SetLayerBindingRespInvalidBehavior => {
                Err(SetLayerBindingError::InvalidBehavior)
            }
            Self::Wire::SetLayerBindingRespInvalidParameters => {
                Err(SetLayerBindingError::InvalidParameters)
            }
        })
    }

    fn to_wire(self) -> Self::Wire {
        match self {
            Ok(()) => Self::Wire::SetLayerBindingRespOk,
            Err(SetLayerBindingError::InvalidLocation) => {
                Self::Wire::SetLayerBindingRespInvalidLocation
            }
            Err(SetLayerBindingError::InvalidBehavior) => {
                Self::Wire::SetLayerBindingRespInvalidBehavior
            }
            Err(SetLayerBindingError::InvalidParameters) => {
                Self::Wire::SetLayerBindingRespInvalidParameters
            }
        }
    }
}

impl WireEnum for SaveChangesError {
    type Wire = wire::SaveChangesErrorCode;

    fn from_wire(wire: Self::Wire) -> Option<Self> {
        match wire {
            Self::Wire::SaveChangesErrOk => None,
            Self::Wire::SaveChangesErrGeneric => Some(Self::Generic),
            Self::Wire::SaveChangesErrNotSupported => Some(Self::NotSupported),
            Self::Wire::SaveChangesErrNoSpace => Some(Self::NoSpace),
        }
    }

    fn to_wire(self) -> Self::Wire {
        match self {
            Self::Generic => Self::Wire::SaveChangesErrGeneric,
            Self::NotSupported => Self::Wire::SaveChangesErrNotSupported,
            Self::NoSpace => Self::Wire::SaveChangesErrNoSpace,
        }
    }
}

impl WireEnum for SetActivePhysicalLayoutError {
    type Wire = wire::SetActivePhysicalLayoutErrorCode;

    fn from_wire(wire: Self::Wire) -> Option<Self> {
        match wire {
            Self::Wire::SetActivePhysicalLayoutErrOk => None,
            Self::Wire::SetActivePhysicalLayoutErrGeneric => Some(Self::Generic),
            Self::Wire::SetActivePhysicalLayoutErrInvalidLayoutIndex => {
                Some(Self::InvalidLayoutIndex)
            }
        }
    }

    fn to_wire(self) -> Self::Wire {
        match self {
            Self::Generic => Self::Wire::SetActivePhysicalLayoutErrGeneric,
            Self::InvalidLayoutIndex => Self::Wire::SetActivePhysicalLayoutErrInvalidLayoutIndex,
        }
    }
}

impl WireEnum for MoveLayerError {
    type Wire = wire::MoveLayerErrorCode;

    fn from_wire(wire: Self::Wire) -> Option<Self> {
        match wire {
            Self::Wire::MoveLayerErrOk => None,
            Self::Wire::MoveLayerErrGeneric => Some(Self::Generic),
            Self::Wire::MoveLayerErrInvalidLayer => Some(Self::InvalidLayer),
            Self::Wire::MoveLayerErrInvalidDestination => Some(Self::InvalidDestination),
        }
    }

    fn to_wire(self) -> Self::Wire {
        match self {
            Self::Generic => Self::Wire::MoveLayerErrGeneric,
            Self::InvalidLayer => Self::Wire::MoveLayerErrInvalidLayer,
            Self::InvalidDestination => Self::Wire::MoveLayerErrInvalidDestination,
        }
    }
}

impl WireEnum for AddLayerError {
    type Wire = wire::AddLayerErrorCode;

    fn from_wire(wire: Self::Wire) -> Option<Self> {
        match wire {
            Self::Wire::AddLayerErrOk => None,
            Self::Wire::AddLayerErrGeneric => Some(Self::Generic),
            Self::Wire::AddLayerErrNoSpace => Some(Self::NoSpace),
        }
    }

    fn to_wire(self) -> Self::Wire {
        match self {
            Self::Generic => Self::Wire::AddLayerErrGeneric,
            Self::NoSpace => Self::Wire::AddLayerErrNoSpace,
        }
    }
}

impl WireEnum for RemoveLayerError {
    type Wire = wire::RemoveLayerErrorCode;

    fn from_wire(wire: Self::Wire) -> Option<Self> {
        match wire {
            Self::Wire::RemoveLayerErrOk => None,
            Self::Wire::RemoveLayerErrGeneric => Some(Self::Generic),
            Self::Wire::RemoveLayerErrInvalidIndex => Some(Self::InvalidIndex),
        }
    }

    fn to_wire(self) -> Self::Wire {
        match self {
            Self::Generic => Self::Wire::RemoveLayerErrGeneric,
            Self::InvalidIndex => Self::Wire::RemoveLayerErrInvalidIndex,
        }
    }
}

impl WireEnum for RestoreLayerError {
    type Wire = wire::RestoreLayerErrorCode;

    fn from_wire(wire: Self::Wire) -> Option<Self> {
        match wire {
            Self::Wire::RestoreLayerErrOk => None,
            Self::Wire::RestoreLayerErrGeneric => Some(Self::Generic),
            Self::Wire::RestoreLayerErrInvalidId => Some(Self::InvalidId),
            Self::Wire::RestoreLayerErrInvalidIndex => Some(Self::InvalidIndex),
        }
    }

    fn to_wire(self) -> Self::Wire {
        match self {
            Self::Generic => Self::Wire::RestoreLayerErrGeneric,
            Self::InvalidId => Self::Wire::RestoreLayerErrInvalidId,
            Self::InvalidIndex => Self::Wire::RestoreLayerErrInvalidIndex,
        }
    }
}

impl WireEnum for Result<(), SetLayerPropsError> {
    type Wire = wire::SetLayerPropsResponse;

    fn from_wire(wire: Self::Wire) -> Option<Self> {
        Some(match wire {
            Self::Wire::SetLayerPropsRespOk => Ok(()),
            Self::Wire::SetLayerPropsRespErrGeneric => Err(SetLayerPropsError::Generic),
            Self::Wire::SetLayerPropsRespErrInvalidId => Err(SetLayerPropsError::InvalidId),
        })
    }

    fn to_wire(self) -> Self::Wire {
        match self {
            Ok(()) => Self::Wire::SetLayerPropsRespOk,
            Err(SetLayerPropsError::Generic) => Self::Wire::SetLayerPropsRespErrGeneric,
            Err(SetLayerPropsError::InvalidId) => Self::Wire::SetLayerPropsRespErrInvalidId,
        }
    }
}
