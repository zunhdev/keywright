//! Reviewing and saving the changes being tried (design §6, CHG-1–CHG-3): the review
//! sheet's rows, taking one back, and saving. Discarding them all is
//! [`discard_changes`](super::connection::discard_changes).
//!
//! Each row is a way the keymap differs from the one the keyboard saved, in the keycaps'
//! words. A row is named by what it's about, a key or a layer, so taking it back finds it
//! in the review list as it is then.

use keywright_core::action::Action;
use keywright_core::behavior::BehaviorTable;
use keywright_core::editor::Change;
use keywright_core::keycap;
use keywright_core::keymap::{Binding, Keymap, LayerId};
use keywright_core::support_packs::SupportPack;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::AppState;
use super::error::AppError;

/// What a review row is about: the order of the layers, a layer, or a key on one. In
/// TypeScript, `{ kind: "key", layer: 0, position: 13 }` and so on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RowKey {
    /// The layers' order.
    Order,
    /// A layer that's new.
    Added {
        /// The layer.
        layer: u32,
    },
    /// A layer that's gone.
    Removed {
        /// The layer.
        layer: u32,
    },
    /// A layer's name.
    Renamed {
        /// The layer.
        layer: u32,
    },
    /// A key on a layer.
    Key {
        /// The layer.
        layer: u32,
        /// The key.
        position: u32,
    },
}

/// One row of the review sheet (design §6): what changed, how it was when saved, and how
/// it is now.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ReviewRow {
    /// What it's about, to take it back.
    pub key: RowKey,
    /// The layer it's listed under, by ID and name, unless it's the layers' order.
    pub layer: Option<u32>,
    /// That layer's name.
    pub layer_name: Option<String>,
    /// Where the key is, for a key, such as `Left half, row 2, key 1`.
    pub place: Option<String>,
    /// How it was when saved, as a keycap shows it or in a word or two.
    pub before: String,
    /// How it is now.
    pub after: String,
    /// What it does now, in a sentence, for a key.
    pub sentence: Option<String>,
}

/// A position, which fits: a keyboard has at most a few hundred keys.
fn number(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

/// A layer's name, or `Layer 3` for an unnamed third one.
fn layer_name(layer: &keywright_core::keymap::Layer, index: usize) -> String {
    if layer.name.is_empty() {
        format!("Layer {}", index.saturating_add(1))
    } else {
        layer.name.clone()
    }
}

/// A binding as a keycap shows it, or in a word or two.
fn label(keymap: &Keymap, behaviors: &BehaviorTable, binding: Binding) -> String {
    match Action::from_binding(binding, behaviors) {
        Action::Transparent => "Same as layer below".to_owned(),
        Action::None => "Does nothing".to_owned(),
        action => match keycap::describe_action(keymap, behaviors, action).keycap {
            keycap::Keycap::Label(text) | keycap::Keycap::Sticky(text) => text,
            keycap::Keycap::TapHold { tap, hold } => match hold {
                keycap::Hold::Key(hold) => format!("{tap} / {hold}"),
                keycap::Hold::Layer { name, .. } => format!("{tap} / {name}"),
            },
            keycap::Keycap::Layer { name, .. } | keycap::Keycap::Other(name) => name,
            keycap::Keycap::Nothing => "Does nothing".to_owned(),
        },
    }
}

/// What the review sheet lists: the editor's review list, in words.
fn rows(
    current: &Keymap,
    saved: &Keymap,
    behaviors: &BehaviorTable,
    pack: Option<&SupportPack>,
    changes: &[Change],
) -> Vec<ReviewRow> {
    let named = |id: LayerId| {
        current
            .layers
            .iter()
            .enumerate()
            .find(|(_, layer)| layer.id == id)
            .or_else(|| {
                saved
                    .layers
                    .iter()
                    .enumerate()
                    .find(|(_, layer)| layer.id == id)
            })
            .map_or_else(
                || "Missing layer".to_owned(),
                |(index, layer)| layer_name(layer, index),
            )
    };
    let order = |ids: &[LayerId]| {
        ids.iter()
            .map(|id| named(*id))
            .collect::<Vec<_>>()
            .join(" · ")
    };
    changes
        .iter()
        .map(|change| match change {
            Change::LayersReordered { before, after } => ReviewRow {
                key: RowKey::Order,
                layer: None,
                layer_name: None,
                place: None,
                before: order(before),
                after: order(after),
                sentence: None,
            },
            Change::LayerAdded { layer } => ReviewRow {
                key: RowKey::Added { layer: layer.0 },
                layer: Some(layer.0),
                layer_name: Some(named(*layer)),
                place: None,
                before: "Not there".to_owned(),
                after: "Added".to_owned(),
                sentence: None,
            },
            Change::LayerRemoved { layer } => ReviewRow {
                key: RowKey::Removed { layer: layer.id.0 },
                layer: Some(layer.id.0),
                layer_name: Some(named(layer.id)),
                place: None,
                before: "There".to_owned(),
                after: "Removed".to_owned(),
                sentence: None,
            },
            Change::LayerRenamed {
                layer,
                before,
                after,
            } => ReviewRow {
                key: RowKey::Renamed { layer: layer.0 },
                layer: Some(layer.0),
                layer_name: Some(named(*layer)),
                place: None,
                before: before.clone(),
                after: after.clone(),
                sentence: None,
            },
            Change::Binding {
                layer,
                position,
                before,
                after,
            } => ReviewRow {
                key: RowKey::Key {
                    layer: layer.0,
                    position: number(*position),
                },
                layer: Some(layer.0),
                layer_name: Some(named(*layer)),
                place: Some(
                    pack.and_then(|pack| pack.key_place(*position))
                        .unwrap_or_else(|| format!("Key {}", position.saturating_add(1))),
                ),
                before: label(current, behaviors, *before),
                after: label(current, behaviors, *after),
                sentence: Some(
                    keycap::describe_action(
                        current,
                        behaviors,
                        Action::from_binding(*after, behaviors),
                    )
                    .sentence,
                ),
            },
        })
        .collect()
}

/// Whether a row of the review list is the one `key` names.
fn is(change: &Change, key: RowKey) -> bool {
    match (change, key) {
        (Change::LayersReordered { .. }, RowKey::Order) => true,
        (Change::LayerAdded { layer }, RowKey::Added { layer: id })
        | (Change::LayerRenamed { layer, .. }, RowKey::Renamed { layer: id }) => layer.0 == id,
        (Change::LayerRemoved { layer }, RowKey::Removed { layer: id }) => layer.id.0 == id,
        (
            Change::Binding {
                layer, position, ..
            },
            RowKey::Key {
                layer: id,
                position: at,
            },
        ) => layer.0 == id && number(*position) == at,
        _ => false,
    }
}

impl AppState {
    /// The review sheet's rows.
    pub async fn review(&self) -> Result<Vec<ReviewRow>, AppError> {
        let snapshot = self.session.snapshot().await?;
        let (Some(keyboard), Some(editor)) = (&snapshot.keyboard, &snapshot.editor) else {
            return Ok(Vec::new());
        };
        Ok(rows(
            editor.current(),
            editor.baseline(),
            &keyboard.behaviors,
            snapshot.support_pack(),
            &editor.changes(),
        ))
    }

    /// Takes back the row `key` names.
    pub async fn revert(&self, key: RowKey) -> Result<(), AppError> {
        let snapshot = self.session.snapshot().await?;
        let editor = snapshot.editor.as_ref().ok_or(AppError::NotReady)?;
        let change = editor
            .changes()
            .into_iter()
            .find(|change| is(change, key))
            .ok_or(AppError::EditRefused)?;
        Ok(self.session.revert(change).await?)
    }
}

/// The review sheet's rows (design §6): each way the keymap differs from the one the
/// keyboard saved, grouped by layer, in the keycaps' words.
///
/// # Errors
///
/// Fails if the session has stopped.
#[tauri::command]
#[specta::specta]
pub async fn review_changes(app: tauri::State<'_, AppState>) -> Result<Vec<ReviewRow>, AppError> {
    app.review().await
}

/// Takes back one row of the review sheet, as a change that undo undoes (design §6).
///
/// # Errors
///
/// Fails if there's no such row now, the keymap can't be changed now, or the keyboard
/// doesn't carry it out.
#[tauri::command]
#[specta::specta]
pub async fn revert_change(app: tauri::State<'_, AppState>, key: RowKey) -> Result<(), AppError> {
    app.revert(key).await
}

/// Saves the changes being tried to the keyboard (CHG-2). The history stays, so a saved
/// change can be undone, and is then tried again.
///
/// # Errors
///
/// Fails if the keymap can't be changed now, or the keyboard doesn't save.
#[tauri::command]
#[specta::specta]
pub async fn save_changes(app: tauri::State<'_, AppState>) -> Result<(), AppError> {
    Ok(app.session.save().await?)
}

#[cfg(test)]
mod tests {
    use keywright_core::edit::Edit;
    use keywright_device::{Connection, Peer, Session, State};
    use keywright_sim::Simulator;
    use serde_json::json;

    use super::{AppState, ReviewRow, RowKey};

    async fn ready_sofle() -> AppState {
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
        AppState::new(session)
    }

    /// Row 2's Q (13) as Escape, and W (14) as Q, on the default layer.
    async fn change_two_keys(app: &AppState) {
        let editor = app.session.snapshot().await.unwrap().editor.unwrap();
        let base = &editor.current().layers[0];
        let edits = |position: usize, from: usize| Edit::SetBinding {
            layer: base.id,
            position,
            before: base.bindings[position],
            after: base.bindings[from],
        };
        app.session.apply(vec![edits(13, 12)]).await.unwrap();
        app.session.apply(vec![edits(14, 13)]).await.unwrap();
    }

    #[tokio::test]
    async fn rows_say_where_each_key_is_and_what_it_was_and_is() {
        let app = ready_sofle().await;
        change_two_keys(&app).await;
        let rows = app.review().await.unwrap();
        assert_eq!(
            rows[0],
            ReviewRow {
                key: RowKey::Key {
                    layer: 0,
                    position: 13
                },
                layer: Some(0),
                layer_name: Some("default".to_owned()),
                place: Some("Left half, row 2, key 2".to_owned()),
                before: "Q".to_owned(),
                after: "⎋ Esc".to_owned(),
                sentence: Some("Types Escape.".to_owned()),
            }
        );
        assert_eq!(rows.len(), 2);
        assert_eq!(
            serde_json::to_value(rows[0].key).unwrap(),
            json!({ "kind": "key", "layer": 0, "position": 13 })
        );
    }

    /// The issue's walk-through: edit, review, take one row back, save; undoing after the
    /// save makes the change tried again.
    #[tokio::test]
    async fn one_row_is_taken_back_the_rest_saved_and_a_saved_change_undone() {
        let app = ready_sofle().await;
        change_two_keys(&app).await;
        app.revert(RowKey::Key {
            layer: 0,
            position: 14,
        })
        .await
        .unwrap();
        let rows = app.review().await.unwrap();
        assert_eq!(rows.len(), 1);
        app.session.save().await.unwrap();
        assert_eq!(app.review().await.unwrap(), []);
        // Undoing the last change, taking W back, makes W's change tried again.
        app.session.undo().await.unwrap();
        let rows = app.review().await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0].key,
            RowKey::Key {
                layer: 0,
                position: 14
            }
        );
        assert_eq!(
            app.revert(RowKey::Order).await,
            Err(crate::ipc::error::AppError::EditRefused)
        );
    }

    #[tokio::test]
    async fn layer_rows_name_the_layer_and_what_happened_to_it() {
        let app = ready_sofle().await;
        app.rename_layer(1, "Numbers".to_owned()).await.unwrap();
        app.move_layer(2, 1).await.unwrap();
        let rows = app.review().await.unwrap();
        assert_eq!(rows[0].key, RowKey::Order);
        assert_eq!(
            (rows[0].before.as_str(), rows[0].after.as_str()),
            (
                "default · Numbers · raise · adjust",
                "default · raise · Numbers · adjust"
            )
        );
        assert_eq!(rows[1].key, RowKey::Renamed { layer: 1 });
        assert_eq!(
            (rows[1].before.as_str(), rows[1].after.as_str()),
            ("lower", "Numbers")
        );
        app.session.undo().await.unwrap();
        app.remove_layer(3).await.unwrap();
        let rows = app.review().await.unwrap();
        assert_eq!(rows.last().unwrap().key, RowKey::Removed { layer: 3 });
        assert_eq!(rows.last().unwrap().layer_name.as_deref(), Some("adjust"));
    }
}
