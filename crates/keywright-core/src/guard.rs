//! Lockout guards: which layers a keymap can reach, and what a change would break (tech
//! spec §8.4, SAF-2, LAY-4).
//!
//! A change is checked before it's applied: [`guard`] compares the keymap now with the one
//! the change would make, and reports what gets worse. The last way to unlock the keyboard
//! could go, a layer could become impossible to reach, or a layer could trap the user, with
//! no way back to the base layer.
//!
//! [`Reachability`] works this out the way ZMK v0.3 handles layers (`app/src/keymap.c`):
//!
//! - The layer state is a set of active layers. The default layer, ID 0, is always active.
//! - A key does what its binding on the highest active layer does, from the top of the
//!   order down to the default layer. "Same as layer below" goes on to the next active one.
//!   Layers below the default layer in the order are never consulted.
//! - Holding a key that activates a layer (momentary, layer-tap, sticky layer) adds the
//!   layer until it's released. Toggling flips a layer on or off. Switching to a layer
//!   turns every layer off except the default, then turns that one on.
//! - A conditional layer, such as Adjust when Lower and Raise are both active, is on
//!   exactly when all its conditions are.
//!
//! The analysis explores what the layers that stay on (latched) can be, and from each, what
//! holding keys adds. The base layer alone is where it starts.

#[cfg(test)]
mod tests;

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use crate::action::Action;
use crate::behavior::BehaviorTable;
use crate::keymap::{Binding, Keymap, LayerId};

/// The default layer: always active, and where the keyboard starts.
pub const BASE: LayerId = LayerId(0);

/// A layer the firmware turns on by itself when other layers are on, such as Adjust when
/// Lower and Raise are both held. Support packs declare them, because the keyboard doesn't
/// report them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalLayer {
    /// The layers that must all be active.
    pub if_layers: Vec<LayerId>,
    /// The layer that is then active.
    pub then_layer: LayerId,
}

/// Something a change would break.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GuardFinding {
    /// No reachable key unlocks the keyboard any more, so Keywright couldn't make further
    /// changes until the keyboard is reset.
    LastUnlockKeyRemoved,
    /// The layer can no longer be reached from the base layer.
    LayerUnreachable(LayerId),
    /// The layer can be switched to or toggled on, but then there's no way back to the base
    /// layer.
    NoWayBack(LayerId),
}

/// How a key turns a layer on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Activation {
    /// While held (`&mo`).
    Momentary,
    /// While held, and the key does something else when tapped (`&lt`).
    LayerTap,
    /// For the next key press (`&sl`).
    Sticky,
    /// On or off (`&tog`).
    Toggle,
    /// Instead of every other layer (`&to`).
    ToLayer,
}

/// A key that turns a layer on, for showing how each layer is reached (LAY-4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayerKey {
    /// The layer the key is on.
    pub layer: LayerId,
    /// The key.
    pub position: usize,
    /// The layer it turns on.
    pub target: LayerId,
    /// How.
    pub activation: Activation,
}

/// Every key that turns a layer on, layer by layer in order, then by position.
#[must_use]
pub fn layer_keys(keymap: &Keymap, behaviors: &BehaviorTable) -> Vec<LayerKey> {
    keymap
        .layers
        .iter()
        .flat_map(|layer| {
            layer
                .bindings
                .iter()
                .enumerate()
                .filter_map(move |(position, binding)| {
                    let (target, activation) = activation(*binding, behaviors)?;
                    Some(LayerKey {
                        layer: layer.id,
                        position,
                        target,
                        activation,
                    })
                })
        })
        .collect()
}

/// What changes `proposed` would make worse than `current`: the findings it has that
/// `current` doesn't. A keymap that already strands a layer doesn't warn about it again.
#[must_use]
pub fn guard(
    current: &Keymap,
    proposed: &Keymap,
    behaviors: &BehaviorTable,
    conditional_layers: &[ConditionalLayer],
) -> Vec<GuardFinding> {
    let before = Reachability::of(current, behaviors, conditional_layers).findings(current);
    Reachability::of(proposed, behaviors, conditional_layers)
        .findings(proposed)
        .into_iter()
        .filter(|finding| !before.contains(finding))
        .collect()
}

/// The most layer states the analysis explores. A real keymap reaches a few dozen; a
/// pathological one with dozens of toggles could reach billions. Past the limit, the
/// analysis stops, which can only make it report more problems, never fewer.
const MAX_STATES: usize = 1 << 16;

/// What a keymap lets the user reach, starting from the base layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reachability {
    /// The layers that can be active.
    reachable: BTreeSet<LayerId>,
    /// Whether some reachable key unlocks the keyboard.
    unlock_reachable: bool,
    /// Layers that can stay on with no way back to the base layer alone.
    trapped: BTreeSet<LayerId>,
}

impl Reachability {
    /// Analyses a keymap.
    #[must_use]
    pub fn of(
        keymap: &Keymap,
        behaviors: &BehaviorTable,
        conditional_layers: &[ConditionalLayer],
    ) -> Self {
        let Some(model) = Model::new(keymap, behaviors, conditional_layers) else {
            // Without the default layer, ZMK has nowhere to start.
            return Self {
                reachable: BTreeSet::new(),
                unlock_reachable: false,
                trapped: BTreeSet::new(),
            };
        };
        model.explore()
    }

    /// Whether the layer can be active.
    #[must_use]
    pub fn is_reachable(&self, layer: LayerId) -> bool {
        self.reachable.contains(&layer)
    }

    /// Whether some reachable key unlocks the keyboard.
    #[must_use]
    pub fn unlock_reachable(&self) -> bool {
        self.unlock_reachable
    }

    /// Whether the layer can stay on with no way back to the base layer.
    #[must_use]
    pub fn is_trapped(&self, layer: LayerId) -> bool {
        self.trapped.contains(&layer)
    }

    /// Everything wrong with the keymap this analysis is of.
    #[must_use]
    pub fn findings(&self, keymap: &Keymap) -> Vec<GuardFinding> {
        let mut findings = BTreeSet::new();
        if !self.unlock_reachable {
            findings.insert(GuardFinding::LastUnlockKeyRemoved);
        }
        for layer in &keymap.layers {
            if !self.is_reachable(layer.id) {
                findings.insert(GuardFinding::LayerUnreachable(layer.id));
            }
        }
        findings.extend(self.trapped.iter().map(|id| GuardFinding::NoWayBack(*id)));
        findings.into_iter().collect()
    }
}

/// A set of layers, by ID: bit `n` is layer `n`. ZMK keeps its layer state the same way,
/// with at most 32 layers.
type Layers = u32;

fn bit(id: LayerId) -> Option<Layers> {
    1_u32.checked_shl(id.0)
}

/// The keymap, read as ZMK does.
struct Model<'a> {
    /// The layers in order, from the default layer up: lower layers are never consulted.
    layers: Vec<(LayerId, &'a [Binding])>,
    behaviors: &'a BehaviorTable,
    conditional_layers: Vec<(Layers, Layers)>,
    keys: usize,
}

/// What a key does, once the active layers decide its binding.
enum KeyEffect {
    Hold(Layers),
    Toggle(Layers),
    To(Layers),
    Unlock,
}

impl<'a> Model<'a> {
    fn new(
        keymap: &'a Keymap,
        behaviors: &'a BehaviorTable,
        conditional_layers: &[ConditionalLayer],
    ) -> Option<Self> {
        let base = keymap.layers.iter().position(|layer| layer.id == BASE)?;
        let layers: Vec<_> = keymap.layers[base..]
            .iter()
            .filter(|layer| bit(layer.id).is_some())
            .map(|layer| (layer.id, layer.bindings.as_slice()))
            .collect();
        let keys = layers
            .iter()
            .map(|(_, bindings)| bindings.len())
            .max()
            .unwrap_or(0);
        let conditional_layers = conditional_layers
            .iter()
            .filter_map(|conditional| {
                let mut mask = 0;
                for id in &conditional.if_layers {
                    mask |= bit(*id)?;
                }
                Some((mask, bit(conditional.then_layer)?))
            })
            .collect();
        Some(Self {
            layers,
            behaviors,
            conditional_layers,
            keys,
        })
    }

    /// The active layers once the conditional layers have followed `active`: each one on
    /// exactly when all its conditions are. Repeated, because one can be the condition of
    /// another.
    fn settle(&self, mut active: Layers) -> Layers {
        loop {
            let mut next = active;
            for (conditions, then_layer) in &self.conditional_layers {
                if next & conditions == *conditions {
                    next |= then_layer;
                } else {
                    next &= !then_layer;
                }
            }
            // The default layer can't be turned off.
            next |= 1;
            if next == active {
                return active;
            }
            active = next;
        }
    }

    /// What the key at `position` does with these layers active.
    fn effect(&self, active: Layers, position: usize) -> Option<KeyEffect> {
        let binding = self
            .layers
            .iter()
            .rev()
            .filter(|(id, _)| bit(*id).is_some_and(|bit| active & bit != 0))
            .filter_map(|(_, bindings)| bindings.get(position))
            .find(|binding| {
                Action::from_binding(**binding, self.behaviors) != Action::Transparent
            })?;
        match Action::from_binding(*binding, self.behaviors) {
            Action::Unlock => Some(KeyEffect::Unlock),
            action => {
                let (target, activation) = layer_action(action)?;
                let target = bit(target)?;
                Some(match activation {
                    Activation::Momentary | Activation::LayerTap | Activation::Sticky => {
                        KeyEffect::Hold(target)
                    }
                    Activation::Toggle => KeyEffect::Toggle(target),
                    Activation::ToLayer => KeyEffect::To(target),
                })
            }
        }
    }

    /// Explores every latched state, and every set of layers that holding keys reaches from
    /// each.
    fn explore(&self) -> Reachability {
        let mut reachable: Layers = 0;
        let mut unlock_reachable = false;
        let mut explored = 0;

        // Latched states: the layers that stay on once every key is released. The base
        // layer alone is where the keyboard starts.
        let mut latched: Vec<Layers> = vec![self.settle(0)];
        let mut index_of = HashMap::from([(latched[0], 0)]);
        let mut edges: Vec<Vec<usize>> = Vec::new();
        let mut next = 0;

        while next < latched.len() && explored < MAX_STATES {
            let state = latched[next];
            let mut targets = BTreeSet::new();

            // Every set of layers that holding keys reaches from this state.
            let mut held = HashSet::from([state]);
            let mut queue = VecDeque::from([state]);
            while let Some(active) = queue.pop_front() {
                explored += 1;
                reachable |= active;
                for position in 0..self.keys {
                    let Some(effect) = self.effect(active, position) else {
                        continue;
                    };
                    let after = match effect {
                        KeyEffect::Unlock => {
                            unlock_reachable = true;
                            continue;
                        }
                        KeyEffect::Hold(layer) => {
                            let more = self.settle(active | layer);
                            if explored < MAX_STATES && held.insert(more) {
                                queue.push_back(more);
                            }
                            continue;
                        }
                        // Toggling a layer that's on turns it off; if it was only held, the
                        // release would have turned it off anyway.
                        KeyEffect::Toggle(layer) if active & layer != 0 => state & !layer,
                        KeyEffect::Toggle(layer) => state | layer,
                        KeyEffect::To(layer) => layer,
                    };
                    let after = self.settle(after);
                    let target = *index_of.entry(after).or_insert_with(|| {
                        latched.push(after);
                        latched.len() - 1
                    });
                    targets.insert(target);
                }
            }
            edges.push(targets.into_iter().collect());
            next += 1;
        }

        // A latched state is a trap if the base state can't be reached from it. Work
        // backwards from the base state over the explored edges.
        let mut way_back = vec![false; latched.len()];
        way_back[0] = true;
        let mut changed = true;
        while changed {
            changed = false;
            for (from, targets) in edges.iter().enumerate() {
                if !way_back[from] && targets.iter().any(|target| way_back[*target]) {
                    way_back[from] = true;
                    changed = true;
                }
            }
        }
        let mut trapped = BTreeSet::new();
        for (state, back) in latched.iter().zip(&way_back).take(edges.len()) {
            if !back {
                trapped.extend(self.ids(*state & !1));
            }
        }

        Reachability {
            reachable: self.ids(reachable).collect(),
            unlock_reachable,
            trapped,
        }
    }

    /// The layers in a set that the keymap consults: below the default layer, a layer can
    /// be on but no key reaches it.
    fn ids(&self, layers: Layers) -> impl Iterator<Item = LayerId> + '_ {
        self.layers
            .iter()
            .map(|(id, _)| *id)
            .filter(move |id| bit(*id).is_some_and(|bit| layers & bit != 0))
    }
}

/// The layer a binding turns on, and how, if it does.
fn activation(binding: Binding, behaviors: &BehaviorTable) -> Option<(LayerId, Activation)> {
    layer_action(Action::from_binding(binding, behaviors))
}

fn layer_action(action: Action) -> Option<(LayerId, Activation)> {
    match action {
        Action::Momentary(layer) => Some((layer, Activation::Momentary)),
        Action::LayerTap { layer, .. } => Some((layer, Activation::LayerTap)),
        Action::StickyLayer(layer) => Some((layer, Activation::Sticky)),
        Action::Toggle(layer) => Some((layer, Activation::Toggle)),
        Action::ToLayer(layer) => Some((layer, Activation::ToLayer)),
        _ => None,
    }
}
