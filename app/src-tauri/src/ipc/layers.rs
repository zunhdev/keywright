//! Changing layers from the layers sidebar and the layer overview (design §3.1, §5.1,
//! LAY-1–LAY-6): renaming, reordering, adding and removing them. Each is one change, which
//! one undo undoes; undoing a removal restores the layer.
//!
//! Layers are named by ID, as the canvas knows them, and placed by index in the order.

use keywright_core::edit::{Edit, EditError};
use keywright_core::keymap::LayerId;
use keywright_device::{InvalidRequest, check_layer_name};
use serde::Serialize;
use specta::Type;

use super::AppState;
use super::error::AppError;

/// Why a layer name can't be used (LAY-1, spike S1). In TypeScript, `{ kind: "too-long",
/// max: 18 }` and so on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum NameProblem {
    /// It's longer than the keyboard takes, in bytes of UTF-8: 18 letters in English,
    /// fewer in other scripts.
    TooLong {
        /// The most bytes it can have.
        max: u32,
    },
    /// It has a character the keyboard's firmware can't receive, such as `Э`.
    Character {
        /// The character.
        character: String,
    },
}

/// What's wrong with a layer name, if anything.
fn problem(name: &str) -> Option<NameProblem> {
    match check_layer_name(name) {
        Err(InvalidRequest::LayerNameCharacter(character)) => Some(NameProblem::Character {
            character: character.to_string(),
        }),
        Err(InvalidRequest::LayerNameTooLong { max, .. }) => Some(NameProblem::TooLong {
            max: u32::try_from(max).unwrap_or(u32::MAX),
        }),
        // Only the two above come from a name.
        _ => None,
    }
}

/// An index from the UI. One past what a keymap can have is refused by the core.
fn index(index: u32) -> usize {
    usize::try_from(index).unwrap_or(usize::MAX)
}

impl AppState {
    /// Renames a layer.
    pub async fn rename_layer(&self, layer: u32, name: String) -> Result<(), AppError> {
        if problem(&name).is_some() {
            return Err(AppError::InvalidLayerName);
        }
        // A name strands no one.
        self.change(true, |keymap, _| {
            let id = LayerId(layer);
            let before = keymap
                .layer(id)
                .ok_or(EditError::UnknownLayer(id))?
                .name
                .clone();
            Ok(vec![Edit::RenameLayer {
                layer: id,
                before,
                after: name,
            }])
        })
        .await
    }

    /// Moves a layer to index `to` in the order. Layer keys still turn on the same layer,
    /// since they name it by ID (LAY-2).
    pub async fn move_layer(&self, layer: u32, to: u32, force: bool) -> Result<(), AppError> {
        self.change(force, |keymap, _| {
            let id = LayerId(layer);
            let from = keymap
                .layers
                .iter()
                .position(|each| each.id == id)
                .ok_or(EditError::UnknownLayer(id))?;
            Ok(vec![Edit::MoveLayer {
                from,
                to: index(to),
            }])
        })
        .await
    }

    /// Removes a layer. Undoing puts it back, with its name and keys (LAY-6).
    pub async fn remove_layer(&self, layer: u32, force: bool) -> Result<(), AppError> {
        self.change(force, |keymap, _| {
            let id = LayerId(layer);
            let (index, layer) = keymap
                .layers
                .iter()
                .enumerate()
                .find(|(_, each)| each.id == id)
                .ok_or(EditError::UnknownLayer(id))?;
            Ok(vec![Edit::RemoveLayer {
                index,
                layer: layer.clone(),
            }])
        })
        .await
    }
}

/// What's wrong with a layer name, for the rename field to say as it's typed, or `null`
/// if the keyboard can take it.
#[tauri::command]
#[specta::specta]
#[must_use]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn layer_name_problem(name: String) -> Option<NameProblem> {
    problem(&name)
}

/// Renames a layer (LAY-1).
///
/// # Errors
///
/// Fails if the keyboard can't take the name, the keymap can't be changed now, or the
/// keyboard doesn't rename it.
#[tauri::command]
#[specta::specta]
pub async fn rename_layer(
    app: tauri::State<'_, AppState>,
    layer: u32,
    name: String,
) -> Result<(), AppError> {
    app.rename_layer(layer, name).await
}

/// Moves a layer to another place in the order (LAY-2), unless that would strand the user
/// and not `force` (SAF-2).
///
/// # Errors
///
/// Fails if there's no such layer or place, the keymap can't be changed now, the move
/// would strand the user, or the keyboard doesn't move it.
#[tauri::command]
#[specta::specta]
pub async fn move_layer(
    app: tauri::State<'_, AppState>,
    layer: u32,
    to: u32,
    force: bool,
) -> Result<(), AppError> {
    app.move_layer(layer, to, force).await
}

/// Removes a layer, which undo puts back (LAY-3, LAY-6), unless that would strand the
/// user and not `force` (SAF-2).
///
/// # Errors
///
/// Fails if there's no such layer, the keymap can't be changed now, the removal would
/// strand the user, or the keyboard doesn't remove it.
#[tauri::command]
#[specta::specta]
pub async fn remove_layer(
    app: tauri::State<'_, AppState>,
    layer: u32,
    force: bool,
) -> Result<(), AppError> {
    app.remove_layer(layer, force).await
}

/// Adds a layer, in the free slot the keyboard picks (LAY-3). Undo removes it again.
///
/// # Errors
///
/// Fails if the keymap can't be changed now, or the keyboard has no room for another
/// layer.
#[tauri::command]
#[specta::specta]
pub async fn add_layer(app: tauri::State<'_, AppState>) -> Result<(), AppError> {
    Ok(app.session.add_layer().await?)
}

#[cfg(test)]
mod tests {
    use keywright_core::keymap::Keymap;
    use keywright_device::{Connection, Peer, Session, State};
    use keywright_sim::{Firmware, Keyboard, Simulator};
    use serde_json::json;

    use super::{AppState, NameProblem, problem};
    use crate::ipc::error::AppError;

    const LOWER: u32 = 1;
    const RAISE: u32 = 2;
    /// The default layer's Lower and Raise keys.
    const LOWER_KEY: usize = 53;
    const RAISE_KEY: usize = 56;

    /// The simulated Sofle, unlocked and ready, with `reserved` free layer slots.
    async fn ready_sofle(reserved: usize) -> AppState {
        let session = Session::start();
        let keyboard = Simulator::start(Keyboard::new(
            Firmware::sofle().unwrap().with_reserved_layers(reserved),
        ));
        keyboard.press_unlock_key();
        session
            .connect(Connection::new(keyboard.connect(), Peer::Simulated))
            .await
            .unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
        AppState::new(session)
    }

    async fn keymap(app: &AppState) -> Keymap {
        app.session
            .snapshot()
            .await
            .unwrap()
            .editor
            .unwrap()
            .current()
            .clone()
    }

    fn order(keymap: &Keymap) -> Vec<u32> {
        keymap.layers.iter().map(|layer| layer.id.0).collect()
    }

    #[test]
    fn a_name_is_checked_as_the_keyboard_takes_it() {
        assert_eq!(problem("Navigation"), None);
        assert_eq!(
            problem("A name far too long"),
            Some(NameProblem::TooLong { max: 18 })
        );
        assert_eq!(
            problem("Эхо"),
            Some(NameProblem::Character {
                character: "Э".to_owned()
            })
        );
        assert_eq!(
            serde_json::to_value(NameProblem::TooLong { max: 18 }).unwrap(),
            json!({ "kind": "too-long", "max": 18 })
        );
    }

    #[tokio::test]
    async fn a_layer_is_renamed_and_the_rename_undone() {
        let app = ready_sofle(0).await;
        app.rename_layer(LOWER, "Numbers".to_owned()).await.unwrap();
        assert_eq!(keymap(&app).await.layers[1].name, "Numbers");
        app.session.undo().await.unwrap();
        assert_eq!(keymap(&app).await.layers[1].name, "lower");
        assert_eq!(
            app.rename_layer(LOWER, "Эхо".to_owned()).await,
            Err(AppError::InvalidLayerName)
        );
    }

    #[tokio::test]
    async fn after_reordering_layer_keys_still_turn_on_the_same_layer() {
        let app = ready_sofle(0).await;
        let before = keymap(&app).await;
        app.move_layer(RAISE, 1, false).await.unwrap();
        let after = keymap(&app).await;
        assert_eq!(order(&after), [0, 2, 1, 3]);
        // The keys didn't change: they name their layers by ID.
        assert_eq!(
            after
                .layer(keywright_core::keymap::LayerId(0))
                .unwrap()
                .bindings[LOWER_KEY],
            before.layers[0].bindings[LOWER_KEY]
        );
        assert_eq!(
            after
                .layer(keywright_core::keymap::LayerId(0))
                .unwrap()
                .bindings[RAISE_KEY],
            before.layers[0].bindings[RAISE_KEY]
        );
        app.session.undo().await.unwrap();
        assert_eq!(order(&keymap(&app).await), [0, 1, 2, 3]);
    }

    #[tokio::test]
    async fn a_removed_layer_comes_back_with_undo() {
        let app = ready_sofle(0).await;
        let before = keymap(&app).await;
        // Raise holds the Unlock key, so the guard would ask first.
        app.remove_layer(RAISE, true).await.unwrap();
        let removed = keymap(&app).await;
        assert_eq!(order(&removed), [0, 1, 3]);
        assert_eq!(removed.free_layer_slots, 1);
        app.session.undo().await.unwrap();
        assert_eq!(keymap(&app).await, before);
    }

    #[tokio::test]
    async fn a_layer_is_added_where_there_is_room_and_not_where_there_isnt() {
        let full = ready_sofle(0).await;
        assert!(full.session.add_layer().await.is_err());
        let roomy = ready_sofle(1).await;
        roomy.session.add_layer().await.unwrap();
        let added = keymap(&roomy).await;
        assert_eq!(added.layers.len(), 5);
        assert_eq!(added.free_layer_slots, 0);
        roomy.session.undo().await.unwrap();
        assert_eq!(keymap(&roomy).await.layers.len(), 4);
    }
}
