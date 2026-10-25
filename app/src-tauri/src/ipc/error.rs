//! The errors commands return (NFR-6): a stable code, plus data where it helps, and never
//! English text. The UI words each one itself.

use keywright_core::editor::EditorError;
use keywright_device::SessionError;
use serde::Serialize;
use specta::Type;

use super::firmware::FirmwareProblem;
use super::guard::GuardWarning;

/// Why a command didn't do what it was asked. In TypeScript, `{ code: "not-ready" }` and
/// so on: the codes are part of the IPC contract, so a code never changes meaning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(tag = "code", rename_all = "kebab-case")]
pub enum AppError {
    /// The keymap can't be changed in the session's state, such as while it's locked.
    NotReady,
    /// The keyboard is still carrying out the previous change.
    Busy,
    /// There's nothing to undo.
    NothingToUndo,
    /// There's nothing to redo.
    NothingToRedo,
    /// Nothing has been copied to paste.
    NothingToPaste,
    /// The editor refused an edit that doesn't fit the keymap.
    EditRefused,
    /// The keyboard didn't carry out a request, so the session rolled the change back.
    KeyboardFailed,
    /// The keymap couldn't be backed up, so a destructive command changed nothing.
    BackupFailed,
    /// The session has stopped, so the app has to be restarted.
    SessionStopped,
    /// The menu bar didn't take a change to one of its items.
    MenuFailed,
    /// A layer name the keyboard can't take: too long, or with a character it can't
    /// receive (`layer_name_problem` says which).
    InvalidLayerName,
    /// Keywright has no stock keymap for the keyboard: it has no support pack.
    NoStockKeymap,
    /// The keyboard said it couldn't restore its stock settings.
    ResetFailed,
    /// System Settings didn't open.
    SettingsNotOpened,
    /// A setting didn't change: the menu bar didn't take the item, or the setting couldn't
    /// be saved.
    SettingNotChanged,
    /// A finding of the Mac check was dismissed, but that couldn't be saved, so it comes
    /// back once Keywright quits.
    DismissalNotSaved,
    /// Every key code Keywright can make act as Globe is in the keymap already (MAC-8).
    NoGlobeCode,
    /// macOS didn't take the Globe key's mapping, or its removal.
    GlobeNotMapped,
    /// The backups folder couldn't be read.
    BackupsNotRead,
    /// There's no such backup: it was deleted, or was never one of the automatic backups.
    NoSuchBackup,
    /// The backup can't be read: the file can't be opened, or isn't a backup Keywright
    /// can read.
    UnreadableBackup,
    /// The Import preview has no backup to show or apply.
    NothingToImport,
    /// The exported keymap couldn't be written.
    ExportFailed,
    /// Finder didn't show the file.
    NotShownInFinder,
    /// Keywright doesn't know how to update the keyboard's firmware: it has no support
    /// pack for it.
    UnknownKeyboard,
    /// The file chosen isn't firmware the keyboard takes.
    NotFirmware {
        /// Why not.
        problem: FirmwareProblem,
    },
    /// No firmware file is chosen for the half.
    NoFirmwareChosen,
    /// macOS can't say when a half's drive appears.
    DrivesNotWatched,
    /// Copying the firmware onto the half's drive failed, as when it's unplugged.
    CopyFailed,
    /// The half didn't restart after the copy, so it may not take the file.
    DidNotRestart,
    /// The half restarted, but didn't come back with its new firmware.
    DidNotComeBack,
    /// Waiting for the half's drive was cancelled.
    UpdateCancelled,
    /// The update stopped unexpectedly.
    UpdateFailed,
    /// The change would strand the user (SAF-2), so it wasn't applied: sent again with
    /// `force`, it is.
    WouldStrand {
        /// What it would break.
        findings: Vec<GuardWarning>,
    },
}

impl From<SessionError> for AppError {
    fn from(error: SessionError) -> Self {
        match error {
            SessionError::NotReady => Self::NotReady,
            SessionError::Editor(EditorError::Busy) => Self::Busy,
            SessionError::Editor(EditorError::NothingToUndo) => Self::NothingToUndo,
            SessionError::Editor(EditorError::NothingToRedo) => Self::NothingToRedo,
            SessionError::Editor(EditorError::Edit(_)) => Self::EditRefused,
            SessionError::Keyboard(_) => Self::KeyboardFailed,
            SessionError::Backup(_) => Self::BackupFailed,
            SessionError::ResetFailed => Self::ResetFailed,
            SessionError::Stopped => Self::SessionStopped,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::sync::Arc;

    use keywright_core::editor::EditorError;
    use keywright_device::SessionError;
    use serde_json::json;

    use super::AppError;

    #[test]
    fn errors_reach_the_ui_as_codes_without_text() {
        let cases = [
            (SessionError::NotReady, "not-ready"),
            (SessionError::Editor(EditorError::Busy), "busy"),
            (
                SessionError::Editor(EditorError::NothingToUndo),
                "nothing-to-undo",
            ),
            (
                SessionError::Editor(EditorError::NothingToRedo),
                "nothing-to-redo",
            ),
            (
                SessionError::Backup(Arc::new(io::Error::other("disk full"))),
                "backup-failed",
            ),
            (SessionError::Stopped, "session-stopped"),
        ];
        for (error, code) in cases {
            let json = serde_json::to_value(AppError::from(error)).unwrap();
            assert_eq!(json, json!({ "code": code }));
        }
    }
}
