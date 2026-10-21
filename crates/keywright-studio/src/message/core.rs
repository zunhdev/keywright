//! The `core` subsystem (`core.proto`): device info, the lock state, and resetting settings.

use super::proto::core as wire;
use super::{DecodeError, WireEnum, decode_enum, encode_enum, required};
use wire::notification::NotificationType;
use wire::request::RequestType;
use wire::response::ResponseType;

/// A request to the `core` subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum CoreRequest {
    /// The keyboard's name and serial number. ZMK answers it while locked.
    GetDeviceInfo,
    /// Whether the keyboard is locked. ZMK answers it while locked.
    GetLockState,
    /// Locks the keyboard. ZMK v0.3 doesn't handle it, and answers
    /// [`ErrorCondition::RpcNotFound`](super::ErrorCondition::RpcNotFound).
    Lock,
    /// Resets the keyboard's saved settings, the keymap included, to the firmware's
    /// defaults (CHG-5).
    ResetSettings,
}

/// The answer to a [`CoreRequest`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum CoreResponse {
    /// The answer to [`CoreRequest::GetDeviceInfo`].
    GetDeviceInfo(DeviceInfo),
    /// The answer to [`CoreRequest::GetLockState`].
    GetLockState(LockState),
    /// The answer to [`CoreRequest::ResetSettings`].
    ResetSettings {
        /// Whether every subsystem reset its settings.
        succeeded: bool,
    },
}

/// A change in the `core` subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum CoreNotification {
    /// The keyboard was locked or unlocked.
    LockStateChanged(LockState),
}

/// What the keyboard says about itself.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct DeviceInfo {
    /// The keyboard's name, as its firmware was built with it.
    pub name: String,
    /// The microcontroller's hardware ID, or nothing if the firmware can't read it. Its hex
    /// is the USB serial number (tech spec §6.1).
    pub serial_number: Vec<u8>,
}

/// Whether the keyboard accepts requests that read or change the keymap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum LockState {
    /// Only device info, the lock state and the list of behaviors are available. The user
    /// unlocks the keyboard by pressing its unlock key.
    Locked,
    /// Every request is available.
    Unlocked,
}

impl WireEnum for LockState {
    type Wire = wire::LockState;

    fn from_wire(wire: Self::Wire) -> Option<Self> {
        Some(match wire {
            Self::Wire::ZmkStudioCoreLockStateLocked => Self::Locked,
            Self::Wire::ZmkStudioCoreLockStateUnlocked => Self::Unlocked,
        })
    }

    fn to_wire(self) -> Self::Wire {
        match self {
            Self::Locked => Self::Wire::ZmkStudioCoreLockStateLocked,
            Self::Unlocked => Self::Wire::ZmkStudioCoreLockStateUnlocked,
        }
    }
}

impl From<&CoreRequest> for wire::Request {
    fn from(request: &CoreRequest) -> Self {
        let request_type = match request {
            CoreRequest::GetDeviceInfo => RequestType::GetDeviceInfo(true),
            CoreRequest::GetLockState => RequestType::GetLockState(true),
            CoreRequest::Lock => RequestType::Lock(true),
            CoreRequest::ResetSettings => RequestType::ResetSettings(true),
        };
        Self {
            request_type: Some(request_type),
        }
    }
}

impl TryFrom<wire::Request> for CoreRequest {
    type Error = DecodeError;

    fn try_from(request: wire::Request) -> Result<Self, DecodeError> {
        let request_type = required("core.Request.request_type", request.request_type)?;
        Ok(match request_type {
            RequestType::GetDeviceInfo(_) => Self::GetDeviceInfo,
            RequestType::GetLockState(_) => Self::GetLockState,
            RequestType::Lock(_) => Self::Lock,
            RequestType::ResetSettings(_) => Self::ResetSettings,
        })
    }
}

impl From<&CoreResponse> for wire::Response {
    fn from(response: &CoreResponse) -> Self {
        let response_type = match response {
            CoreResponse::GetDeviceInfo(info) => {
                ResponseType::GetDeviceInfo(wire::GetDeviceInfoResponse {
                    name: info.name.clone(),
                    serial_number: info.serial_number.clone(),
                })
            }
            CoreResponse::GetLockState(state) => ResponseType::GetLockState(encode_enum(*state)),
            CoreResponse::ResetSettings { succeeded } => ResponseType::ResetSettings(*succeeded),
        };
        Self {
            response_type: Some(response_type),
        }
    }
}

impl TryFrom<wire::Response> for CoreResponse {
    type Error = DecodeError;

    fn try_from(response: wire::Response) -> Result<Self, DecodeError> {
        let response_type = required("core.Response.response_type", response.response_type)?;
        Ok(match response_type {
            ResponseType::GetDeviceInfo(info) => Self::GetDeviceInfo(DeviceInfo {
                name: info.name,
                serial_number: info.serial_number,
            }),
            ResponseType::GetLockState(state) => {
                Self::GetLockState(decode_enum("core.Response.get_lock_state", state)?)
            }
            ResponseType::ResetSettings(succeeded) => Self::ResetSettings { succeeded },
        })
    }
}

impl From<&CoreNotification> for wire::Notification {
    fn from(notification: &CoreNotification) -> Self {
        let notification_type = match notification {
            CoreNotification::LockStateChanged(state) => {
                NotificationType::LockStateChanged(encode_enum(*state))
            }
        };
        Self {
            notification_type: Some(notification_type),
        }
    }
}

impl TryFrom<wire::Notification> for CoreNotification {
    type Error = DecodeError;

    fn try_from(notification: wire::Notification) -> Result<Self, DecodeError> {
        let notification_type = required(
            "core.Notification.notification_type",
            notification.notification_type,
        )?;
        Ok(match notification_type {
            NotificationType::LockStateChanged(state) => {
                let field = "core.Notification.lock_state_changed";
                Self::LockStateChanged(decode_enum(field, state)?)
            }
        })
    }
}
