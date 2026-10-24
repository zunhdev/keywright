//! The lockout guard (design §5.1, §5.5, SAF-2): a change that would remove the last key
//! that unlocks the keyboard, make a layer unreachable, or leave no way back from one isn't
//! applied straight away. The command refuses it with what it would break, and the UI asks;
//! sent again with `force`, it's applied.
//!
//! The keymap payload lists what's wrong with the keymap as it is, for the layer overview
//! and the layers sidebar.

use keywright_core::behavior::BehaviorTable;
use keywright_core::edit::Edit;
use keywright_core::guard::{self, Activation, ConditionalLayer, GuardFinding, Reachability};
use keywright_core::keycap;
use keywright_core::keymap::{Keymap, LayerId};
use serde::Serialize;
use specta::Type;

/// What a change would break (SAF-2). In TypeScript, `{ kind: "last-unlock-key" }` and
/// so on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum GuardWarning {
    /// No key could unlock the keyboard any more, so Keywright couldn't change it again
    /// until it's reset.
    LastUnlockKey,
    /// No key could turn the layer on any more.
    LayerUnreachable {
        /// The layer, by ID.
        layer: u32,
        /// Its name.
        name: String,
    },
    /// Once on the layer, there'd be no way back to the base layer.
    NoWayBack {
        /// The layer, by ID.
        layer: u32,
        /// Its name.
        name: String,
    },
}

impl GuardWarning {
    /// A finding, with the layer named as `keymap` names it.
    fn of(finding: GuardFinding, keymap: &Keymap) -> Self {
        let name = |id: LayerId| keycap::layer_name(keymap, id);
        match finding {
            GuardFinding::LastUnlockKeyRemoved => Self::LastUnlockKey,
            GuardFinding::LayerUnreachable(id) => Self::LayerUnreachable {
                layer: id.0,
                name: name(id),
            },
            GuardFinding::NoWayBack(id) => Self::NoWayBack {
                layer: id.0,
                name: name(id),
            },
        }
    }
}

/// What applying `edits` to `keymap` would break, that isn't broken already. Edits that
/// don't fit break nothing here: the editor refuses them.
pub fn check(
    keymap: &Keymap,
    edits: &[Edit],
    behaviors: &BehaviorTable,
    conditional_layers: &[ConditionalLayer],
) -> Vec<GuardWarning> {
    let mut proposed = keymap.clone();
    if edits.iter().any(|edit| edit.apply(&mut proposed).is_err()) {
        return Vec::new();
    }
    guard::guard(keymap, &proposed, behaviors, conditional_layers)
        .into_iter()
        .map(|finding| GuardWarning::of(finding, keymap))
        .collect()
}

/// A key on a layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
pub struct KeyRef {
    /// The layer, by ID.
    pub layer: u32,
    /// The key, by key position.
    pub position: u32,
}

/// What's wrong with a layer as the keymap is now (design §5.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum LayerWarning {
    /// No key can turn it on.
    Unreachable,
    /// Once on it, there's no way back to the base layer: these keys switch to it or
    /// toggle it on, and leave it on.
    NoWayBack {
        /// The keys, to show.
        keys: Vec<KeyRef>,
    },
}

/// What's wrong with the keymap as it is: each layer's warnings, by its place in the
/// order, and whether some key can unlock the keyboard.
pub fn warnings(
    keymap: &Keymap,
    behaviors: &BehaviorTable,
    conditional_layers: &[ConditionalLayer],
) -> (Vec<Vec<LayerWarning>>, bool) {
    let reachability = Reachability::of(keymap, behaviors, conditional_layers);
    let keys = guard::layer_keys(keymap, behaviors);
    let layers = keymap
        .layers
        .iter()
        .map(|layer| {
            let mut found = Vec::new();
            if !reachability.is_reachable(layer.id) {
                found.push(LayerWarning::Unreachable);
            } else if reachability.is_trapped(layer.id) {
                let keys = keys
                    .iter()
                    .filter(|key| {
                        key.target == layer.id
                            && matches!(key.activation, Activation::Toggle | Activation::ToLayer)
                    })
                    .map(|key| KeyRef {
                        layer: key.layer.0,
                        position: u32::try_from(key.position).unwrap_or(u32::MAX),
                    })
                    .collect();
                found.push(LayerWarning::NoWayBack { keys });
            }
            found
        })
        .collect();
    (layers, reachability.unlock_reachable())
}

#[cfg(test)]
mod tests {
    use keywright_core::action::Action;
    use keywright_core::behavior::BehaviorTable;
    use keywright_core::edit::Edit;
    use keywright_core::guard::ConditionalLayer;
    use keywright_core::keymap::{Keymap, LayerId};
    use keywright_device::{Connection, Peer, Session, State};
    use keywright_sim::Simulator;

    use super::{GuardWarning, KeyRef, LayerWarning, check, warnings};
    use crate::ipc::AppState;
    use crate::ipc::error::AppError;

    const RAISE: LayerId = LayerId(2);
    /// The raise layer's B: the stock firmware's Unlock key.
    const UNLOCK: usize = 41;

    /// The simulated Sofle's stock keymap, its behaviors, and Adjust = Lower + Raise.
    async fn sofle() -> (Keymap, BehaviorTable, Vec<ConditionalLayer>) {
        let session = Session::start();
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        session
            .connect(Connection::new(keyboard.connect(), Peer::Simulated))
            .await
            .unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
        let snapshot = session.snapshot().await.unwrap();
        let conditional = snapshot
            .support_pack()
            .unwrap()
            .hints
            .conditional_layers
            .clone();
        (
            snapshot.editor.unwrap().current().clone(),
            snapshot.keyboard.unwrap().behaviors,
            conditional,
        )
    }

    fn set(
        keymap: &Keymap,
        behaviors: &BehaviorTable,
        layer: LayerId,
        position: usize,
        action: Action,
    ) -> Edit {
        Edit::SetBinding {
            layer,
            position,
            before: keymap.layer(layer).unwrap().bindings[position],
            after: action.to_binding(behaviors).unwrap(),
        }
    }

    #[tokio::test]
    async fn replacing_the_unlock_key_is_caught() {
        let (keymap, behaviors, adjust) = sofle().await;
        let edit = set(&keymap, &behaviors, RAISE, UNLOCK, Action::None);
        assert_eq!(
            check(&keymap, &[edit], &behaviors, &adjust),
            [GuardWarning::LastUnlockKey]
        );
    }

    #[tokio::test]
    async fn losing_the_raise_key_makes_raise_unreachable() {
        let (keymap, behaviors, adjust) = sofle().await;
        // The default layer's raise key, as nothing.
        let edit = set(&keymap, &behaviors, LayerId(0), 56, Action::None);
        let found = check(&keymap, &[edit], &behaviors, &adjust);
        assert!(found.contains(&GuardWarning::LayerUnreachable {
            layer: 2,
            name: "raise".to_owned()
        }));
        assert!(found.contains(&GuardWarning::LastUnlockKey));
    }

    #[tokio::test]
    async fn switching_to_a_layer_with_no_way_back_is_caught_and_listed() {
        let (keymap, behaviors, adjust) = sofle().await;
        let edit = set(
            &keymap,
            &behaviors,
            LayerId(0),
            0,
            Action::ToLayer(LayerId(1)),
        );
        assert_eq!(
            check(&keymap, std::slice::from_ref(&edit), &behaviors, &adjust),
            [GuardWarning::NoWayBack {
                layer: 1,
                name: "lower".to_owned()
            }]
        );
        let mut changed = keymap.clone();
        edit.apply(&mut changed).unwrap();
        let (layers, unlockable) = warnings(&changed, &behaviors, &adjust);
        assert!(unlockable);
        assert_eq!(
            layers[1],
            [LayerWarning::NoWayBack {
                keys: vec![KeyRef {
                    layer: 0,
                    position: 0
                }]
            }]
        );
    }

    /// Replacing raise's B, the stock Unlock key, through the inspector: refused with what
    /// it would break, then applied with `force`, which undo undoes (SAF-2).
    #[tokio::test]
    async fn a_risky_change_waits_for_change_anyway_and_can_be_undone() {
        let session = Session::start();
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        session
            .connect(Connection::new(keyboard.connect(), Peer::Simulated))
            .await
            .unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
        let app = AppState::new(session);
        let nothing = || crate::ipc::action::Action::Nothing;
        assert_eq!(
            app.set_action(2, vec![41], nothing(), false).await,
            Err(AppError::WouldStrand {
                findings: vec![GuardWarning::LastUnlockKey]
            })
        );
        assert_eq!(app.review().await.unwrap(), []);
        app.set_action(2, vec![41], nothing(), true).await.unwrap();
        assert_eq!(app.review().await.unwrap().len(), 1);
        app.session.undo().await.unwrap();
        assert_eq!(app.review().await.unwrap(), []);
        // Removing raise strands it and its Unlock key; the guard says so first.
        assert!(matches!(
            app.remove_layer(2, false).await,
            Err(AppError::WouldStrand { .. })
        ));
    }

    #[tokio::test]
    async fn the_stock_keymap_has_nothing_wrong_and_harmless_changes_pass() {
        let (keymap, behaviors, adjust) = sofle().await;
        let (layers, unlockable) = warnings(&keymap, &behaviors, &adjust);
        assert!(unlockable && layers.iter().all(Vec::is_empty));
        let edit = set(&keymap, &behaviors, LayerId(0), 13, Action::None);
        assert_eq!(check(&keymap, &[edit], &behaviors, &adjust), []);
    }
}
