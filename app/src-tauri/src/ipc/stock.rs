//! Restoring Keebart's stock keymap (CHG-4, CHG-5, SAF-3): for some keys or a layer, as
//! changes to try, from the support pack's copy of the stock keymap; or for the whole
//! keyboard, with the firmware's own reset, after a backup.

use std::collections::HashSet;

use keywright_core::behavior::BehaviorTable;
use keywright_core::edit::Edit;
use keywright_core::keymap::{Keymap, LayerId};
use keywright_core::support_packs::SupportPack;

use super::AppState;
use super::error::AppError;

/// The edits that would make `keymap` the stock one, from the support pack, where the
/// keyboard can take them.
fn to_stock(pack: &SupportPack, keymap: &Keymap, behaviors: &BehaviorTable) -> Vec<Edit> {
    pack.stock_keymap.import(keymap, behaviors).edits
}

/// What differs from the stock keymap: the keys, and the layers with such keys or another
/// name. Without a support pack, nothing can be restored, so nothing differs.
pub fn differences(
    pack: Option<&SupportPack>,
    keymap: &Keymap,
    behaviors: &BehaviorTable,
) -> (HashSet<(LayerId, usize)>, HashSet<LayerId>) {
    let mut keys = HashSet::new();
    let mut layers = HashSet::new();
    for edit in pack.map_or_else(Vec::new, |pack| to_stock(pack, keymap, behaviors)) {
        match edit {
            Edit::SetBinding {
                layer, position, ..
            } => {
                keys.insert((layer, position));
                layers.insert(layer);
            }
            Edit::RenameLayer { layer, .. } => {
                layers.insert(layer);
            }
            _ => {}
        }
    }
    (keys, layers)
}

impl AppState {
    /// Restores the stock action of the keys at `positions` on a layer (CHG-4), as a change
    /// to try, unless that would strand the user and not `force`.
    pub async fn restore_stock_keys(
        &self,
        layer: u32,
        positions: Vec<u32>,
        force: bool,
    ) -> Result<(), AppError> {
        let pack = self.pack().await?;
        let positions: HashSet<usize> = positions
            .into_iter()
            .filter_map(|position| usize::try_from(position).ok())
            .collect();
        self.change(force, |keymap, behaviors| {
            Ok(to_stock(pack, keymap, behaviors)
                .into_iter()
                .filter(|edit| {
                    matches!(edit, Edit::SetBinding { layer: id, position, .. }
                        if id.0 == layer && positions.contains(position))
                })
                .collect())
        })
        .await
    }

    /// Restores a layer's stock keys and name (CHG-4), as a change to try, unless that would
    /// strand the user and not `force`.
    pub async fn restore_stock_layer(&self, layer: u32, force: bool) -> Result<(), AppError> {
        let pack = self.pack().await?;
        self.change(force, |keymap, behaviors| {
            Ok(to_stock(pack, keymap, behaviors)
                .into_iter()
                .filter(|edit| match edit {
                    Edit::SetBinding { layer: id, .. } | Edit::RenameLayer { layer: id, .. } => {
                        id.0 == layer
                    }
                    _ => false,
                })
                .collect())
        })
        .await
    }

    /// The keyboard's support pack, which holds its stock keymap.
    async fn pack(&self) -> Result<&'static SupportPack, AppError> {
        self.session
            .snapshot()
            .await?
            .support_pack()
            .ok_or(AppError::NoStockKeymap)
    }
}

/// Restores the stock action of some keys on a layer, from the support pack (CHG-4).
///
/// # Errors
///
/// Fails if Keywright has no stock keymap for the keyboard, the keymap can't be changed
/// now, the change would strand the user, or the keyboard doesn't carry it out.
#[tauri::command]
#[specta::specta]
pub async fn restore_stock_keys(
    app: tauri::State<'_, AppState>,
    layer: u32,
    positions: Vec<u32>,
    force: bool,
) -> Result<(), AppError> {
    app.restore_stock_keys(layer, positions, force).await
}

/// Restores a layer's stock keys and name, from the support pack (CHG-4).
///
/// # Errors
///
/// As for [`restore_stock_keys`].
#[tauri::command]
#[specta::specta]
pub async fn restore_stock_layer(
    app: tauri::State<'_, AppState>,
    layer: u32,
    force: bool,
) -> Result<(), AppError> {
    app.restore_stock_layer(layer, force).await
}

/// Restores the whole keyboard's stock settings (CHG-5): backs up the keymap, then runs the
/// firmware's own reset (SAF-3). The changes being tried and the history go.
///
/// # Errors
///
/// Fails if the keymap can't be changed now, the backup can't be written, in which case
/// nothing changes, or the keyboard doesn't reset.
#[tauri::command]
#[specta::specta]
pub async fn restore_stock_settings(app: tauri::State<'_, AppState>) -> Result<(), AppError> {
    Ok(app.session.restore_stock_settings().await?)
}

#[cfg(test)]
mod tests {
    use keywright_device::{Connection, Peer, Session, State};
    use keywright_sim::Simulator;

    use super::AppState;
    use crate::ipc::action::Action;

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

    #[tokio::test]
    async fn a_key_and_a_layer_are_restored_as_changes_that_undo_takes_back() {
        let app = ready_sofle().await;
        app.set_action(0, vec![13, 14], Action::Nothing, false)
            .await
            .unwrap();
        app.rename_layer(1, "Numbers".to_owned()).await.unwrap();
        assert_eq!(app.review().await.unwrap().len(), 3);

        app.restore_stock_keys(0, vec![13], false).await.unwrap();
        assert_eq!(app.review().await.unwrap().len(), 2);
        app.session.undo().await.unwrap();
        assert_eq!(app.review().await.unwrap().len(), 3);

        app.restore_stock_layer(0, false).await.unwrap();
        app.restore_stock_layer(1, false).await.unwrap();
        assert_eq!(app.review().await.unwrap(), []);
    }

    #[tokio::test]
    async fn the_whole_keyboard_goes_back_to_stock() {
        let app = ready_sofle().await;
        app.set_action(0, vec![13], Action::Nothing, false)
            .await
            .unwrap();
        app.session.save().await.unwrap();
        app.session.restore_stock_settings().await.unwrap();
        let snapshot = app.session.snapshot().await.unwrap();
        let keyboard = snapshot.keyboard.as_ref().unwrap();
        let (keys, _) = super::differences(
            snapshot.support_pack(),
            snapshot.editor.as_ref().unwrap().current(),
            &keyboard.behaviors,
        );
        assert!(keys.is_empty(), "{keys:?}");
    }
}
