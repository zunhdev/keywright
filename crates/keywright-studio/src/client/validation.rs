//! Checks requests before they're sent (tech spec §5.3).
//!
//! The keyboard answers most bad requests with an error, but not all. A layer name too
//! long for the firmware's buffer fails to decode, and ZMK v0.3 then sends no response at
//! all, so the request would only time out. The client refuses such requests up front, and
//! out-of-range layers and keys with them.
//!
//! [`KeymapShape`] learns the keymap's shape from the responses the client relays: each
//! layer's ID and number of keys, the name limit, and the number of physical layouts. It
//! follows the changes the client's own requests make. When it can't, as after discarding
//! changes, it forgets the keymap, and checks only what needs no keymap until the next one
//! arrives.

use crate::message::{
    CoreResponse, Keymap, KeymapRequest, KeymapResponse, Layer, LayerId, Request, Response,
};

/// A request the client refused to send, because the keyboard would reject it or choke
/// on it.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InvalidRequest {
    /// The layer name doesn't fit the firmware's buffer. Counted in bytes of UTF-8.
    #[error("a layer name can be at most {max} bytes long, and this one has {length}")]
    LayerNameTooLong {
        /// The name's length.
        length: usize,
        /// The most the firmware takes.
        max: usize,
    },
    /// No layer in the keymap has this ID.
    #[error("the keymap has no layer with ID {}", .0.0)]
    UnknownLayer(LayerId),
    /// The key position is negative, or past the layer's last key.
    #[error("key position {0} isn't on the layer")]
    KeyPositionOutOfRange(i32),
    /// The layer index is past the keymap's layers.
    #[error("layer index {index} is past the keymap's {layers} layers")]
    LayerIndexOutOfRange {
        /// The index.
        index: u32,
        /// How many layers the keymap has.
        layers: usize,
    },
    /// The physical layout index is past the keyboard's layouts.
    #[error("physical layout index {index} is past the keyboard's {layouts} layouts")]
    LayoutIndexOutOfRange {
        /// The index.
        index: u32,
        /// How many physical layouts the keyboard has.
        layouts: usize,
    },
}

/// What the client knows about the keymap's shape.
#[derive(Debug, Default)]
pub(super) struct KeymapShape {
    /// The layers, if the client knows them.
    keymap: Option<Layers>,
    /// How many physical layouts the keyboard has, if the client has seen them.
    layouts: Option<usize>,
}

/// The layers, in order, and the name limit.
#[derive(Debug)]
struct Layers {
    /// Each layer's ID and number of keys.
    layers: Vec<(LayerId, usize)>,
    /// The size of the firmware's buffer for a layer name, with room for C's terminating
    /// NUL. Zero if the firmware doesn't say.
    name_buffer: usize,
}

impl KeymapShape {
    /// Checks a request against what the client knows.
    pub(super) fn check(&self, request: &Request) -> Result<(), InvalidRequest> {
        let Request::Keymap(request) = request else {
            return Ok(());
        };
        match request {
            KeymapRequest::SetLayerBinding {
                layer_id,
                key_position,
                ..
            } => {
                let position = usize::try_from(*key_position)
                    .map_err(|_| InvalidRequest::KeyPositionOutOfRange(*key_position))?;
                if let Some(keymap) = &self.keymap
                    && position >= keymap.keys(*layer_id)?
                {
                    return Err(InvalidRequest::KeyPositionOutOfRange(*key_position));
                }
            }
            KeymapRequest::SetLayerProps { layer_id, name } => {
                if let Some(keymap) = &self.keymap {
                    keymap.keys(*layer_id)?;
                    keymap.check_name(name)?;
                }
            }
            KeymapRequest::MoveLayer {
                start_index,
                dest_index,
            } => {
                self.check_layer_index(*start_index)?;
                self.check_layer_index(*dest_index)?;
            }
            KeymapRequest::RemoveLayer { layer_index } => self.check_layer_index(*layer_index)?,
            KeymapRequest::RestoreLayer { at_index, .. } => {
                // A restored layer can also go after the last one.
                if let Some(keymap) = &self.keymap
                    && index(*at_index) > keymap.layers.len()
                {
                    return Err(InvalidRequest::LayerIndexOutOfRange {
                        index: *at_index,
                        layers: keymap.layers.len(),
                    });
                }
            }
            KeymapRequest::SetActivePhysicalLayout { index: layout } => {
                if let Some(layouts) = self.layouts
                    && index(*layout) >= layouts
                {
                    return Err(InvalidRequest::LayoutIndexOutOfRange {
                        index: *layout,
                        layouts,
                    });
                }
            }
            KeymapRequest::GetKeymap
            | KeymapRequest::CheckUnsavedChanges
            | KeymapRequest::SaveChanges
            | KeymapRequest::DiscardChanges
            | KeymapRequest::GetPhysicalLayouts
            | KeymapRequest::AddLayer => {}
        }
        Ok(())
    }

    /// Learns from the keyboard's response to a request.
    pub(super) fn update(&mut self, request: &Request, response: &Response) {
        let Response::Keymap(response) = response else {
            if let Response::Core(CoreResponse::ResetSettings { .. }) = response {
                // The keymap is back to the firmware's defaults, which the client hasn't seen.
                self.keymap = None;
            }
            return;
        };
        match (request, response) {
            (
                _,
                KeymapResponse::GetKeymap(keymap)
                | KeymapResponse::SetActivePhysicalLayout(Ok(keymap))
                | KeymapResponse::MoveLayer(Ok(keymap)),
            ) => self.keymap = Some(Layers::of(keymap)),
            (_, KeymapResponse::GetPhysicalLayouts(layouts)) => {
                self.layouts = Some(layouts.layouts.len());
            }
            (_, KeymapResponse::AddLayer(Ok(added))) => self.insert(added.index, &added.layer),
            (
                Request::Keymap(KeymapRequest::RestoreLayer { at_index, .. }),
                KeymapResponse::RestoreLayer(Ok(layer)),
            ) => self.insert(*at_index, layer),
            (
                Request::Keymap(KeymapRequest::RemoveLayer { layer_index }),
                KeymapResponse::RemoveLayer(Ok(())),
            ) => self.remove(*layer_index),
            // The keymap is back to its saved state, which the client hasn't seen.
            (_, KeymapResponse::DiscardChanges) => self.keymap = None,
            _ => {}
        }
    }

    fn check_layer_index(&self, layer: u32) -> Result<(), InvalidRequest> {
        match &self.keymap {
            Some(keymap) if index(layer) >= keymap.layers.len() => {
                Err(InvalidRequest::LayerIndexOutOfRange {
                    index: layer,
                    layers: keymap.layers.len(),
                })
            }
            _ => Ok(()),
        }
    }

    /// Adds a layer at `at`, or forgets the keymap if that can't be right.
    fn insert(&mut self, at: u32, layer: &Layer) {
        if let Some(keymap) = &mut self.keymap {
            let at = index(at);
            if at <= keymap.layers.len() {
                keymap.layers.insert(at, (layer.id, layer.bindings.len()));
            } else {
                self.keymap = None;
            }
        }
    }

    /// Removes the layer at `at`, or forgets the keymap if that can't be right.
    fn remove(&mut self, at: u32) {
        if let Some(keymap) = &mut self.keymap {
            let at = index(at);
            if at < keymap.layers.len() {
                keymap.layers.remove(at);
            } else {
                self.keymap = None;
            }
        }
    }
}

impl Layers {
    fn of(keymap: &Keymap) -> Self {
        Self {
            layers: keymap
                .layers
                .iter()
                .map(|layer| (layer.id, layer.bindings.len()))
                .collect(),
            name_buffer: index(keymap.max_layer_name_length),
        }
    }

    /// The number of keys on a layer.
    fn keys(&self, id: LayerId) -> Result<usize, InvalidRequest> {
        self.layers
            .iter()
            .find(|(candidate, _)| *candidate == id)
            .map(|(_, keys)| *keys)
            .ok_or(InvalidRequest::UnknownLayer(id))
    }

    fn check_name(&self, name: &str) -> Result<(), InvalidRequest> {
        if self.name_buffer > 0 && name.len() >= self.name_buffer {
            return Err(InvalidRequest::LayerNameTooLong {
                length: name.len(),
                max: self.name_buffer - 1,
            });
        }
        Ok(())
    }
}

/// An index or count from the protocol, as a `usize`. On a 16-bit target, one too large
/// for a `usize` becomes `usize::MAX`, which is out of range everywhere.
fn index(value: u32) -> usize {
    usize::try_from(value).unwrap_or(usize::MAX)
}
