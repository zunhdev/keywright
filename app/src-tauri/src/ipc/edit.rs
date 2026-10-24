//! Changing keys from the canvas and the inspector (design §4.5, §4.7, §5.4, EDIT-1,
//! EDIT-5, EDIT-7): setting an action, swapping and copying keys, copying and pasting
//! actions, "same as layer below" and "does nothing", and undo and redo.
//!
//! The core builds each change's edits from the keymap the editor holds, and the session
//! carries them out as one change, which one undo undoes. Keys are named by layer ID and
//! key position, as the canvas knows them.

use keywright_core::action::Action;
use keywright_core::behavior::BehaviorTable;
use keywright_core::edit::{Edit, EditError};
use keywright_core::keymap::{Keymap, LayerId};
use serde::Deserialize;
use specta::Type;

use super::error::AppError;
use super::{AppState, action};

/// What [`set_keys`] makes keys do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum KeyAction {
    /// Same as the layer below (T on the canvas).
    SameAsLayerBelow,
    /// Nothing (⌫ on the canvas).
    DoesNothing,
}

impl From<EditError> for AppError {
    fn from(_: EditError) -> Self {
        Self::EditRefused
    }
}

/// A key position from the UI. One past what a keyboard can have is refused by the core.
fn position(position: u32) -> usize {
    usize::try_from(position).unwrap_or(usize::MAX)
}

impl AppState {
    /// Carries out the edits `build` makes from the keymap the editor holds, as one change.
    async fn change(
        &self,
        build: impl FnOnce(&Keymap, &BehaviorTable) -> Result<Vec<Edit>, AppError>,
    ) -> Result<(), AppError> {
        let snapshot = self.session.snapshot().await?;
        let (Some(keyboard), Some(editor)) = (&snapshot.keyboard, &snapshot.editor) else {
            return Err(AppError::NotReady);
        };
        let edits = build(editor.current(), &keyboard.behaviors)?;
        Ok(self.session.apply(edits).await?)
    }

    /// Swaps what two keys do on a layer, as dragging one onto the other does.
    pub async fn swap_keys(&self, layer: u32, first: u32, second: u32) -> Result<(), AppError> {
        self.change(|keymap, _| {
            Ok(Edit::swap_keys(
                keymap,
                LayerId(layer),
                position(first),
                position(second),
            )?)
        })
        .await
    }

    /// Makes the key at `to` do what the key at `from` does, on a layer, as ⌥-dragging
    /// does.
    pub async fn copy_key(&self, layer: u32, from: u32, to: u32) -> Result<(), AppError> {
        self.change(|keymap, _| {
            let layer = LayerId(layer);
            let binding = *keymap
                .layer(layer)
                .ok_or(EditError::UnknownLayer(layer))?
                .bindings
                .get(position(from))
                .ok_or(AppError::EditRefused)?;
            Ok(Edit::set_keys(keymap, layer, &[position(to)], binding)?)
        })
        .await
    }

    /// Copies what a key does, for [`paste_action`](Self::paste_action), on any layer.
    pub async fn copy_action(&self, layer: u32, position: u32) -> Result<(), AppError> {
        let snapshot = self.session.snapshot().await?;
        let editor = snapshot.editor.as_ref().ok_or(AppError::NotReady)?;
        let binding = *editor
            .current()
            .layer(LayerId(layer))
            .and_then(|layer| layer.bindings.get(self::position(position)))
            .ok_or(AppError::EditRefused)?;
        *self
            .clipboard
            .lock()
            .map_err(|_| AppError::SessionStopped)? = Some(binding);
        Ok(())
    }

    /// Makes keys on a layer do what the key last copied does.
    pub async fn paste_action(&self, layer: u32, positions: Vec<u32>) -> Result<(), AppError> {
        let copied = *self
            .clipboard
            .lock()
            .map_err(|_| AppError::SessionStopped)?;
        let binding = copied.ok_or(AppError::NothingToPaste)?;
        let positions: Vec<usize> = positions.into_iter().map(position).collect();
        self.change(|keymap, _| Ok(Edit::set_keys(keymap, LayerId(layer), &positions, binding)?))
            .await
    }

    /// Makes keys on a layer do `action` (the inspector, EDIT-1).
    pub async fn set_action(
        &self,
        layer: u32,
        positions: Vec<u32>,
        action: action::Action,
    ) -> Result<(), AppError> {
        let positions: Vec<usize> = positions.into_iter().map(position).collect();
        self.change(|keymap, behaviors| {
            let binding = Action::from(action)
                .to_binding(behaviors)
                .map_err(|_| AppError::EditRefused)?;
            Ok(Edit::set_keys(keymap, LayerId(layer), &positions, binding)?)
        })
        .await
    }

    /// Makes keys on a layer "same as layer below" or "does nothing".
    pub async fn set_keys(
        &self,
        layer: u32,
        positions: Vec<u32>,
        action: KeyAction,
    ) -> Result<(), AppError> {
        let positions: Vec<usize> = positions.into_iter().map(position).collect();
        self.change(|keymap, behaviors| {
            let action = match action {
                KeyAction::SameAsLayerBelow => Action::Transparent,
                KeyAction::DoesNothing => Action::None,
            };
            let binding = action
                .to_binding(behaviors)
                .map_err(|_| AppError::EditRefused)?;
            Ok(Edit::set_keys(keymap, LayerId(layer), &positions, binding)?)
        })
        .await
    }
}

/// Swaps what two keys do on a layer (dragging one onto the other).
///
/// # Errors
///
/// Fails if the keymap can't be changed now, a key isn't there, or the keyboard doesn't
/// carry out the change, which is then undone.
#[tauri::command]
#[specta::specta]
pub async fn swap_keys(
    app: tauri::State<'_, AppState>,
    layer: u32,
    first: u32,
    second: u32,
) -> Result<(), AppError> {
    app.swap_keys(layer, first, second).await
}

/// Makes the key at `to` do what the key at `from` does, on a layer (⌥-dragging).
///
/// # Errors
///
/// As for [`swap_keys`].
#[tauri::command]
#[specta::specta]
pub async fn copy_key(
    app: tauri::State<'_, AppState>,
    layer: u32,
    from: u32,
    to: u32,
) -> Result<(), AppError> {
    app.copy_key(layer, from, to).await
}

/// Copies what a key does, to paste onto other keys (⌘C).
///
/// # Errors
///
/// Fails if there's no keymap, or the key isn't there.
#[tauri::command]
#[specta::specta]
pub async fn copy_action(
    app: tauri::State<'_, AppState>,
    layer: u32,
    position: u32,
) -> Result<(), AppError> {
    app.copy_action(layer, position).await
}

/// Makes keys do what the key last copied does (⌘V).
///
/// # Errors
///
/// As for [`swap_keys`], and if nothing has been copied.
#[tauri::command]
#[specta::specta]
pub async fn paste_action(
    app: tauri::State<'_, AppState>,
    layer: u32,
    positions: Vec<u32>,
) -> Result<(), AppError> {
    app.paste_action(layer, positions).await
}

/// Makes keys "same as layer below" (T) or "does nothing" (⌫).
///
/// # Errors
///
/// As for [`swap_keys`].
#[tauri::command]
#[specta::specta]
pub async fn set_keys(
    app: tauri::State<'_, AppState>,
    layer: u32,
    positions: Vec<u32>,
    action: KeyAction,
) -> Result<(), AppError> {
    app.set_keys(layer, positions, action).await
}

/// Makes keys on a layer do an action (the inspector).
///
/// # Errors
///
/// As for [`swap_keys`], and if the keyboard's firmware has no behavior for the action.
#[tauri::command]
#[specta::specta]
pub async fn set_action(
    app: tauri::State<'_, AppState>,
    layer: u32,
    positions: Vec<u32>,
    action: action::Action,
) -> Result<(), AppError> {
    app.set_action(layer, positions, action).await
}

/// Undoes the last change (⌘Z).
///
/// # Errors
///
/// Fails if there's nothing to undo, or as for [`swap_keys`].
#[tauri::command]
#[specta::specta]
pub async fn undo(app: tauri::State<'_, AppState>) -> Result<(), AppError> {
    Ok(app.session.undo().await?)
}

/// Redoes the last change undone (⇧⌘Z).
///
/// # Errors
///
/// Fails if there's nothing to redo, or as for [`swap_keys`].
#[tauri::command]
#[specta::specta]
pub async fn redo(app: tauri::State<'_, AppState>) -> Result<(), AppError> {
    Ok(app.session.redo().await?)
}

#[cfg(test)]
mod tests {
    use keywright_core::keymap::Binding;
    use keywright_device::{Connection, Peer, Session, State};
    use keywright_sim::Simulator;

    use super::{AppState, KeyAction};
    use crate::ipc::error::AppError;

    /// The default layer, by ID.
    const BASE: u32 = 0;
    /// Row 2's Q and W, on the default layer.
    const Q: u32 = 13;
    const W: u32 = 14;
    /// Row 1's grave accent, see-through on Lower.
    const GRAVE: u32 = 0;
    const LOWER: u32 = 1;

    /// The simulated Sofle, unlocked and ready.
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

    /// What the keys at `positions` on a layer do now.
    async fn bindings(app: &AppState, layer: u32, positions: &[u32]) -> Vec<Binding> {
        let editor = app.session.snapshot().await.unwrap().editor.unwrap();
        let layer = editor
            .current()
            .layers
            .iter()
            .find(|each| each.id.0 == layer)
            .unwrap()
            .clone();
        positions
            .iter()
            .map(|&position| layer.bindings[position as usize])
            .collect()
    }

    /// Swapping and copying are each one change: a single undo puts every key back.
    #[tokio::test]
    async fn a_swap_or_a_copy_is_undone_in_one_step() {
        let app = ready_sofle().await;
        let stock = bindings(&app, BASE, &[Q, W]).await;

        app.swap_keys(BASE, Q, W).await.unwrap();
        assert_eq!(bindings(&app, BASE, &[Q, W]).await, [stock[1], stock[0]]);
        app.session.undo().await.unwrap();
        assert_eq!(bindings(&app, BASE, &[Q, W]).await, stock);

        app.copy_key(BASE, Q, W).await.unwrap();
        assert_eq!(bindings(&app, BASE, &[Q, W]).await, [stock[0], stock[0]]);
        app.session.undo().await.unwrap();
        assert_eq!(bindings(&app, BASE, &[Q, W]).await, stock);
    }

    /// An action copied on one layer pastes onto several keys of another, as one change.
    #[tokio::test]
    async fn a_copied_action_pastes_onto_several_keys_at_once() {
        let app = ready_sofle().await;
        assert_eq!(
            app.paste_action(BASE, vec![W]).await,
            Err(AppError::NothingToPaste)
        );
        let q = bindings(&app, BASE, &[Q]).await[0];
        let lower = bindings(&app, LOWER, &[Q, W]).await;

        app.copy_action(BASE, Q).await.unwrap();
        app.paste_action(LOWER, vec![Q, W]).await.unwrap();
        assert_eq!(bindings(&app, LOWER, &[Q, W]).await, [q, q]);
        app.session.undo().await.unwrap();
        assert_eq!(bindings(&app, LOWER, &[Q, W]).await, lower);
    }

    #[tokio::test]
    async fn keys_become_see_through_or_do_nothing() {
        let app = ready_sofle().await;
        let see_through = bindings(&app, LOWER, &[GRAVE]).await[0];
        app.set_keys(BASE, vec![Q, W], KeyAction::SameAsLayerBelow)
            .await
            .unwrap();
        assert_eq!(
            bindings(&app, BASE, &[Q, W]).await,
            [see_through, see_through]
        );
        app.set_keys(LOWER, vec![GRAVE], KeyAction::DoesNothing)
            .await
            .unwrap();
        assert_ne!(bindings(&app, LOWER, &[GRAVE]).await[0], see_through);
    }

    #[tokio::test]
    async fn keys_that_arent_there_are_refused() {
        let app = ready_sofle().await;
        assert_eq!(app.swap_keys(BASE, Q, 60).await, Err(AppError::EditRefused));
        assert_eq!(app.copy_key(9, Q, W).await, Err(AppError::EditRefused));
        assert_eq!(app.copy_action(BASE, 60).await, Err(AppError::EditRefused));
        assert_eq!(
            app.set_keys(BASE, vec![u32::MAX], KeyAction::DoesNothing)
                .await,
            Err(AppError::EditRefused)
        );
    }

    #[tokio::test]
    async fn nothing_changes_without_a_keymap() {
        let app = AppState::new(Session::start());
        assert_eq!(app.swap_keys(BASE, Q, W).await, Err(AppError::NotReady));
        assert_eq!(app.copy_action(BASE, Q).await, Err(AppError::NotReady));
    }
}
