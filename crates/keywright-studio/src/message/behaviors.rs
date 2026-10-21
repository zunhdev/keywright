//! The `behaviors` subsystem (`behaviors.proto`): the behaviors a keyboard has, and the
//! parameters each one takes.

use super::proto::behaviors as wire;
use super::{DecodeError, required};
use wire::behavior_parameter_value_description::ValueType;
use wire::request::RequestType;
use wire::response::ResponseType;

/// A behavior's ID on one keyboard.
///
/// IDs stay the same while the keyboard keeps its settings, but differ between keyboards
/// (tech spec §8.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct BehaviorId(pub u32);

/// A request to the `behaviors` subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum BehaviorsRequest {
    /// The IDs of all the keyboard's behaviors. ZMK answers it while locked.
    ListAllBehaviors,
    /// A behavior's name and the parameters it takes.
    GetBehaviorDetails {
        /// The behavior.
        behavior_id: BehaviorId,
    },
}

/// The answer to a [`BehaviorsRequest`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum BehaviorsResponse {
    /// The answer to [`BehaviorsRequest::ListAllBehaviors`].
    ListAllBehaviors(Vec<BehaviorId>),
    /// The answer to [`BehaviorsRequest::GetBehaviorDetails`].
    GetBehaviorDetails(BehaviorDetails),
}

/// A behavior and the parameters it takes.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct BehaviorDetails {
    /// The behavior's ID.
    pub id: BehaviorId,
    /// The behavior's name in the firmware, such as `Key Press`.
    pub display_name: String,
    /// The combinations of parameters the behavior accepts. A binding is valid when its
    /// two parameters match one set. With no sets, both parameters must be 0.
    #[cfg_attr(test, proptest(strategy = "super::tests::short_vec()"))]
    pub metadata: Vec<ParameterSet>,
}

/// Values that a behavior's two parameters accept together.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct ParameterSet {
    /// The values `param1` accepts: any one of them matches. If there are none, it must
    /// be 0.
    #[cfg_attr(test, proptest(strategy = "super::tests::short_vec()"))]
    pub param1: Vec<ParameterValue>,
    /// The values `param2` accepts, as for `param1`.
    #[cfg_attr(test, proptest(strategy = "super::tests::short_vec()"))]
    pub param2: Vec<ParameterValue>,
}

/// A value, or a kind of value, that a parameter accepts
/// (`behaviors.BehaviorParameterValueDescription`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct ParameterValue {
    /// What to call it, such as `Next Profile`. It can be empty.
    pub name: String,
    /// The values it covers.
    pub kind: ValueKind,
}

/// The values a [`ParameterValue`] covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum ValueKind {
    /// Only 0: the parameter is unused.
    Nil,
    /// Exactly this value.
    Constant(u32),
    /// Any value from `min` to `max`, both included.
    Range {
        /// The smallest value.
        min: i32,
        /// The largest value.
        max: i32,
    },
    /// A HID usage: a key, or a consumer control such as a media key, encoded as tech spec
    /// §8.1 describes.
    HidUsage {
        /// The largest keyboard usage ID (page 7) the firmware supports.
        keyboard_max: u32,
        /// The largest consumer usage ID (page 12) the firmware supports.
        consumer_max: u32,
    },
    /// A layer ID.
    LayerId,
}

impl From<&BehaviorsRequest> for wire::Request {
    fn from(request: &BehaviorsRequest) -> Self {
        let request_type = match request {
            BehaviorsRequest::ListAllBehaviors => RequestType::ListAllBehaviors(true),
            BehaviorsRequest::GetBehaviorDetails { behavior_id } => {
                RequestType::GetBehaviorDetails(wire::GetBehaviorDetailsRequest {
                    behavior_id: behavior_id.0,
                })
            }
        };
        Self {
            request_type: Some(request_type),
        }
    }
}

impl TryFrom<wire::Request> for BehaviorsRequest {
    type Error = DecodeError;

    fn try_from(request: wire::Request) -> Result<Self, DecodeError> {
        let request_type = required("behaviors.Request.request_type", request.request_type)?;
        Ok(match request_type {
            RequestType::ListAllBehaviors(_) => Self::ListAllBehaviors,
            RequestType::GetBehaviorDetails(request) => Self::GetBehaviorDetails {
                behavior_id: BehaviorId(request.behavior_id),
            },
        })
    }
}

impl From<&BehaviorsResponse> for wire::Response {
    fn from(response: &BehaviorsResponse) -> Self {
        let response_type = match response {
            BehaviorsResponse::ListAllBehaviors(ids) => {
                ResponseType::ListAllBehaviors(wire::ListAllBehaviorsResponse {
                    behaviors: ids.iter().map(|id| id.0).collect(),
                })
            }
            BehaviorsResponse::GetBehaviorDetails(details) => {
                ResponseType::GetBehaviorDetails(details.into())
            }
        };
        Self {
            response_type: Some(response_type),
        }
    }
}

impl TryFrom<wire::Response> for BehaviorsResponse {
    type Error = DecodeError;

    fn try_from(response: wire::Response) -> Result<Self, DecodeError> {
        let response_type = required("behaviors.Response.response_type", response.response_type)?;
        Ok(match response_type {
            ResponseType::ListAllBehaviors(list) => {
                Self::ListAllBehaviors(list.behaviors.into_iter().map(BehaviorId).collect())
            }
            ResponseType::GetBehaviorDetails(details) => {
                Self::GetBehaviorDetails(details.try_into()?)
            }
        })
    }
}

impl From<&BehaviorDetails> for wire::GetBehaviorDetailsResponse {
    fn from(details: &BehaviorDetails) -> Self {
        Self {
            id: details.id.0,
            display_name: details.display_name.clone(),
            metadata: details.metadata.iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<wire::GetBehaviorDetailsResponse> for BehaviorDetails {
    type Error = DecodeError;

    fn try_from(details: wire::GetBehaviorDetailsResponse) -> Result<Self, DecodeError> {
        Ok(Self {
            id: BehaviorId(details.id),
            display_name: details.display_name,
            metadata: details
                .metadata
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<&ParameterSet> for wire::BehaviorBindingParametersSet {
    fn from(set: &ParameterSet) -> Self {
        Self {
            param1: set.param1.iter().map(Into::into).collect(),
            param2: set.param2.iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<wire::BehaviorBindingParametersSet> for ParameterSet {
    type Error = DecodeError;

    fn try_from(set: wire::BehaviorBindingParametersSet) -> Result<Self, DecodeError> {
        let values = |values: Vec<wire::BehaviorParameterValueDescription>| {
            values
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()
        };
        Ok(Self {
            param1: values(set.param1)?,
            param2: values(set.param2)?,
        })
    }
}

impl From<&ParameterValue> for wire::BehaviorParameterValueDescription {
    fn from(value: &ParameterValue) -> Self {
        let value_type = match value.kind {
            ValueKind::Nil => ValueType::Nil(wire::BehaviorParameterNil {}),
            ValueKind::Constant(constant) => ValueType::Constant(constant),
            ValueKind::Range { min, max } => {
                ValueType::Range(wire::BehaviorParameterValueDescriptionRange { min, max })
            }
            ValueKind::HidUsage {
                keyboard_max,
                consumer_max,
            } => ValueType::HidUsage(wire::BehaviorParameterHidUsage {
                keyboard_max,
                consumer_max,
            }),
            ValueKind::LayerId => ValueType::LayerId(wire::BehaviorParameterLayerId {}),
        };
        Self {
            name: value.name.clone(),
            value_type: Some(value_type),
        }
    }
}

impl TryFrom<wire::BehaviorParameterValueDescription> for ParameterValue {
    type Error = DecodeError;

    fn try_from(value: wire::BehaviorParameterValueDescription) -> Result<Self, DecodeError> {
        let value_type = required(
            "behaviors.BehaviorParameterValueDescription.value_type",
            value.value_type,
        )?;
        let kind = match value_type {
            ValueType::Nil(_) => ValueKind::Nil,
            ValueType::Constant(constant) => ValueKind::Constant(constant),
            ValueType::Range(range) => ValueKind::Range {
                min: range.min,
                max: range.max,
            },
            ValueType::HidUsage(usage) => ValueKind::HidUsage {
                keyboard_max: usage.keyboard_max,
                consumer_max: usage.consumer_max,
            },
            ValueType::LayerId(_) => ValueKind::LayerId,
        };
        Ok(Self {
            name: value.name,
            kind,
        })
    }
}
