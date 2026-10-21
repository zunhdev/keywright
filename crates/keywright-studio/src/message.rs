//! Keywright's own types for the ZMK Studio messages, and their protobuf encoding.
//!
//! The client sends a [`ClientMessage`]. The keyboard answers with a [`DeviceMessage`]:
//! either a response carrying the same request ID, or a notification. Each one encodes to
//! and decodes from the bytes inside one frame. Both directions are here, because
//! `keywright-sim` plays the keyboard's side.
//!
//! The types mirror the vendored `.proto` files, with one module per file and the same
//! field names, but they read like Rust:
//!
//! - a choice between fields (a protobuf `oneof`) is an enum;
//! - a reply that is either `ok` or an error code is a [`Result`];
//! - layer and behavior IDs have types of their own, so an ID can't be passed where an
//!   index is meant.
//!
//! The types generated from the `.proto` files stay private (tech spec §5.1), so changes to
//! the protocol stay inside this module.
//!
//! Decoding never panics: any bytes give a message or a [`DecodeError`]. Two details of the
//! wire format don't show in the types:
//!
//! - Several requests and replies are `bool` fields that only mark a choice, such as
//!   `keymap.Request.get_keymap` or `keymap.SaveChangesResponse.ok`. ZMK sets them to
//!   `true` and only checks which field is present. This module does the same.
//! - Behavior IDs are `uint32`, except in `keymap.BehaviorBinding`, where they are
//!   `sint32`. ZMK's IDs are 16-bit, so [`BehaviorId`] holds a `u32` and reinterprets the
//!   bits of the `sint32`, which maps every value one-to-one.

mod behaviors;
mod core;
mod keymap;
mod meta;
mod proto;
#[cfg(test)]
mod tests;

use prost::Message as _;

pub use self::behaviors::{
    BehaviorDetails, BehaviorId, BehaviorsRequest, BehaviorsResponse, ParameterSet, ParameterValue,
    ValueKind,
};
pub use self::core::{CoreNotification, CoreRequest, CoreResponse, DeviceInfo, LockState};
pub use self::keymap::{
    AddLayerError, AddedLayer, Binding, KeyPhysicalAttrs, Keymap, KeymapNotification,
    KeymapRequest, KeymapResponse, Layer, LayerId, MoveLayerError, PhysicalLayout, PhysicalLayouts,
    RemoveLayerError, RestoreLayerError, SaveChangesError, SetActivePhysicalLayoutError,
    SetLayerBindingError, SetLayerPropsError,
};
pub use self::meta::ErrorCondition;
use self::proto::studio as wire;

/// A request on its way to the keyboard, with the ID its response will carry.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct ClientMessage {
    /// Chosen by the client. The keyboard copies it into the response.
    pub request_id: u32,
    /// What the client asks for.
    pub request: Request,
}

impl ClientMessage {
    /// Encodes the message as the protobuf bytes for one frame.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        wire::Request::from(self).encode_to_vec()
    }

    /// Decodes a message from the protobuf bytes inside one frame.
    ///
    /// # Errors
    ///
    /// Fails if the bytes aren't a request this protocol version defines.
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        wire::Request::decode(bytes)?.try_into()
    }
}

/// A request to the keyboard, by subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum Request {
    /// Device info, the lock state and settings (`core.Request`).
    Core(CoreRequest),
    /// The behaviors the keyboard has (`behaviors.Request`).
    Behaviors(BehaviorsRequest),
    /// The keymap and the physical layouts (`keymap.Request`).
    Keymap(KeymapRequest),
}

/// A message from the keyboard.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum DeviceMessage {
    /// The response to the request with ID `request_id`.
    Response {
        /// The ID of the request this answers.
        request_id: u32,
        /// The answer.
        response: Response,
    },
    /// Something changed on the keyboard, unprompted.
    Notification(Notification),
}

impl DeviceMessage {
    /// Encodes the message as the protobuf bytes for one frame.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        wire::Response::from(self).encode_to_vec()
    }

    /// Decodes a message from the protobuf bytes inside one frame.
    ///
    /// # Errors
    ///
    /// Fails if the bytes aren't a response or notification this protocol version defines.
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        wire::Response::decode(bytes)?.try_into()
    }
}

/// The keyboard's answer to a request.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum Response {
    /// The request succeeded, and there is nothing to report (`meta.no_response`).
    Empty,
    /// The keyboard didn't carry out the request (`meta.simple_error`).
    Error(ErrorCondition),
    /// The answer to a [`CoreRequest`].
    Core(CoreResponse),
    /// The answer to a [`BehaviorsRequest`].
    Behaviors(BehaviorsResponse),
    /// The answer to a [`KeymapRequest`].
    Keymap(KeymapResponse),
}

/// A change the keyboard reports without being asked, by subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum Notification {
    /// A change to the lock state (`core.Notification`).
    Core(CoreNotification),
    /// A change to the keymap's saved state (`keymap.Notification`).
    Keymap(KeymapNotification),
}

/// Why bytes couldn't be decoded as a ZMK Studio message.
///
/// Fields are named as in the `.proto` files, with the package first, such as
/// `keymap.Layer.bindings`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DecodeError {
    /// The bytes aren't a valid protobuf encoding of the message.
    #[error("not a valid protobuf message")]
    Protobuf(#[from] prost::DecodeError),
    /// A field that is always set is missing, or none of the fields in a `oneof` is set.
    /// Protobuf skips fields it doesn't know, so a message from a newer protocol version
    /// can look like this.
    #[error("`{0}` is missing")]
    MissingField(&'static str),
    /// An enum field holds a value the protocol doesn't define, or one that makes no sense
    /// in its place, such as an error code meaning "no error".
    #[error("`{field}` has the invalid value {value}")]
    InvalidEnumValue {
        /// The field.
        field: &'static str,
        /// The value it holds.
        value: i32,
    },
}

/// Returns a field that the protocol always sets, or an error naming it.
fn required<T>(field: &'static str, value: Option<T>) -> Result<T, DecodeError> {
    value.ok_or(DecodeError::MissingField(field))
}

/// A protocol enum that this module represents with a type of its own.
trait WireEnum: Sized {
    /// The generated enum. Messages hold its values as `i32`.
    type Wire: TryFrom<i32> + Into<i32>;

    /// Converts from the generated enum, or returns `None` for a value that makes no
    /// sense here.
    fn from_wire(wire: Self::Wire) -> Option<Self>;

    /// Converts to the generated enum.
    fn to_wire(self) -> Self::Wire;
}

/// Decodes the value of an enum field.
fn decode_enum<T: WireEnum>(field: &'static str, value: i32) -> Result<T, DecodeError> {
    T::Wire::try_from(value)
        .ok()
        .and_then(T::from_wire)
        .ok_or(DecodeError::InvalidEnumValue { field, value })
}

/// Encodes the value of an enum field.
fn encode_enum<T: WireEnum>(value: T) -> i32 {
    value.to_wire().into()
}

impl From<&ClientMessage> for wire::Request {
    fn from(message: &ClientMessage) -> Self {
        use wire::request::Subsystem;

        let subsystem = match &message.request {
            Request::Core(request) => Subsystem::Core(request.into()),
            Request::Behaviors(request) => Subsystem::Behaviors(request.into()),
            Request::Keymap(request) => Subsystem::Keymap(request.into()),
        };
        Self {
            request_id: message.request_id,
            subsystem: Some(subsystem),
        }
    }
}

impl TryFrom<wire::Request> for ClientMessage {
    type Error = DecodeError;

    fn try_from(message: wire::Request) -> Result<Self, DecodeError> {
        use wire::request::Subsystem;

        let subsystem = required("studio.Request.subsystem", message.subsystem)?;
        let request = match subsystem {
            Subsystem::Core(request) => Request::Core(request.try_into()?),
            Subsystem::Behaviors(request) => Request::Behaviors(request.try_into()?),
            Subsystem::Keymap(request) => Request::Keymap(request.try_into()?),
        };
        Ok(Self {
            request_id: message.request_id,
            request,
        })
    }
}

impl From<&DeviceMessage> for wire::Response {
    fn from(message: &DeviceMessage) -> Self {
        use wire::response::Type;

        let r#type = match message {
            DeviceMessage::Response {
                request_id,
                response,
            } => Type::RequestResponse(wire::RequestResponse {
                request_id: *request_id,
                subsystem: Some(response.into()),
            }),
            DeviceMessage::Notification(notification) => Type::Notification(notification.into()),
        };
        Self {
            r#type: Some(r#type),
        }
    }
}

impl TryFrom<wire::Response> for DeviceMessage {
    type Error = DecodeError;

    fn try_from(message: wire::Response) -> Result<Self, DecodeError> {
        use wire::response::Type;

        let message_type = required("studio.Response.type", message.r#type)?;
        Ok(match message_type {
            Type::RequestResponse(response) => {
                let subsystem = required("studio.RequestResponse.subsystem", response.subsystem)?;
                Self::Response {
                    request_id: response.request_id,
                    response: subsystem.try_into()?,
                }
            }
            Type::Notification(notification) => Self::Notification(notification.try_into()?),
        })
    }
}

impl From<&Response> for wire::request_response::Subsystem {
    fn from(response: &Response) -> Self {
        use proto::meta::response::ResponseType as Meta;

        let meta = |response_type| {
            Self::Meta(proto::meta::Response {
                response_type: Some(response_type),
            })
        };
        match response {
            Response::Empty => meta(Meta::NoResponse(true)),
            Response::Error(condition) => meta(Meta::SimpleError(encode_enum(*condition))),
            Response::Core(response) => Self::Core(response.into()),
            Response::Behaviors(response) => Self::Behaviors(response.into()),
            Response::Keymap(response) => Self::Keymap(response.into()),
        }
    }
}

impl TryFrom<wire::request_response::Subsystem> for Response {
    type Error = DecodeError;

    fn try_from(subsystem: wire::request_response::Subsystem) -> Result<Self, DecodeError> {
        use proto::meta::response::ResponseType as Meta;
        use wire::request_response::Subsystem;

        Ok(match subsystem {
            Subsystem::Meta(meta) => {
                let response_type = required("meta.Response.response_type", meta.response_type)?;
                match response_type {
                    Meta::NoResponse(_) => Self::Empty,
                    Meta::SimpleError(condition) => {
                        Self::Error(decode_enum("meta.Response.simple_error", condition)?)
                    }
                }
            }
            Subsystem::Core(response) => Self::Core(response.try_into()?),
            Subsystem::Behaviors(response) => Self::Behaviors(response.try_into()?),
            Subsystem::Keymap(response) => Self::Keymap(response.try_into()?),
        })
    }
}

impl From<&Notification> for wire::Notification {
    fn from(notification: &Notification) -> Self {
        use wire::notification::Subsystem;

        let subsystem = match notification {
            Notification::Core(notification) => Subsystem::Core(notification.into()),
            Notification::Keymap(notification) => Subsystem::Keymap(notification.into()),
        };
        Self {
            subsystem: Some(subsystem),
        }
    }
}

impl TryFrom<wire::Notification> for Notification {
    type Error = DecodeError;

    fn try_from(notification: wire::Notification) -> Result<Self, DecodeError> {
        use wire::notification::Subsystem;

        let subsystem = required("studio.Notification.subsystem", notification.subsystem)?;
        Ok(match subsystem {
            Subsystem::Core(notification) => Self::Core(notification.try_into()?),
            Subsystem::Keymap(notification) => Self::Keymap(notification.try_into()?),
        })
    }
}
