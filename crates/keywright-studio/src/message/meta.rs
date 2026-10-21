//! The `meta` messages (`meta.proto`): replies that any request can get.

use super::WireEnum;
use super::proto::meta as wire;

/// Why the keyboard didn't carry out a request (`meta.ErrorConditions`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum ErrorCondition {
    /// Something else went wrong, such as asking for a behavior the keyboard doesn't have.
    #[error("the keyboard reported an error")]
    Generic,
    /// The request needs the keyboard unlocked, and it is locked.
    #[error("the keyboard is locked")]
    UnlockRequired,
    /// The firmware doesn't handle this request.
    #[error("the keyboard doesn't support this request")]
    RpcNotFound,
    /// The keyboard couldn't decode the request. ZMK v0.3 sends no response at all in this
    /// case (tech spec §5.3).
    #[error("the keyboard couldn't decode the request")]
    DecodeFailed,
    /// The keyboard couldn't encode its response.
    #[error("the keyboard couldn't encode its response")]
    EncodeFailed,
}

impl WireEnum for ErrorCondition {
    type Wire = wire::ErrorConditions;

    fn from_wire(wire: Self::Wire) -> Option<Self> {
        Some(match wire {
            Self::Wire::Generic => Self::Generic,
            Self::Wire::UnlockRequired => Self::UnlockRequired,
            Self::Wire::RpcNotFound => Self::RpcNotFound,
            Self::Wire::MsgDecodeFailed => Self::DecodeFailed,
            Self::Wire::MsgEncodeFailed => Self::EncodeFailed,
        })
    }

    fn to_wire(self) -> Self::Wire {
        match self {
            Self::Generic => Self::Wire::Generic,
            Self::UnlockRequired => Self::Wire::UnlockRequired,
            Self::RpcNotFound => Self::Wire::RpcNotFound,
            Self::DecodeFailed => Self::Wire::MsgDecodeFailed,
            Self::EncodeFailed => Self::Wire::MsgEncodeFailed,
        }
    }
}
