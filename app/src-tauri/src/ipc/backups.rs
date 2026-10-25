//! Backups (design §9.1, §9.2, BAK-3, BAK-4, SAF-4): the Backups window's list of
//! automatic backups and its preview of each against the keymap, and the Import preview,
//! which applies a backup as changes to try.
//!
//! The UI never names a file by its path. The Backups window names a backup by its file's
//! name, which has to be one of those in the backups folder. _Restore…_ there makes that
//! backup the pending import, brings the main window forward, and tells it to show the
//! Import preview ([`ImportRequested`], `import://requested`), which previews and applies
//! the pending import. File › Import… (⌘O) makes the file chosen in the open panel the
//! pending import the same way, and so does opening a `.keywright` file from Finder
//! ([`open_files`], BAK-6). A file that can't be read is pending too, so the preview says
//! why. File › Export… (⇧⌘E) writes the keymap where the save panel says (BAK-1).
//!
//! The preview's rows are the review sheet's, for the edits that make the keymap match the
//! backup ([`Import::preview`]), and what can't be applied is listed with why, never
//! dropped. Applying goes through the lockout guard (SAF-2), and the session backs the
//! keymap up first (SAF-3).

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

use keywright_core::backup::{self, Backup, Import, Reason, Unresolved};
use keywright_core::keymap::LayerId;
use keywright_core::support_packs::SupportPack;
use keywright_device::backups::{self, ReadError};
use serde::Serialize;
use specta::Type;
use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_dialog::DialogExt;
use tauri_specta::Event as _;
use tokio::sync::oneshot;

use super::AppState;
use super::error::AppError;
use super::review::{ReviewRow, RowKey, rows};

/// The Backups window's label.
pub const WINDOW: &str = "backups";

/// `import://requested`: the main window should show the Import preview for the pending
/// import.
#[derive(Debug, Clone, Serialize, Type, tauri_specta::Event)]
#[tauri_specta(event_name = "import://requested")]
pub struct ImportRequested;

/// Why a backup was made, as the Backups window says (design §9.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum BackupReason {
    /// Someone exported the keymap.
    Export,
    /// Keywright met the keyboard for the first time this session.
    FirstConnect,
    /// Before restoring the keyboard's stock settings.
    BeforeRestore,
    /// Before importing a backup.
    BeforeImport,
    /// Before updating the keyboard's firmware.
    BeforeFirmware,
}

impl From<Reason> for BackupReason {
    fn from(reason: Reason) -> Self {
        match reason {
            Reason::Export => Self::Export,
            Reason::FirstConnect => Self::FirstConnect,
            Reason::BeforeRestore => Self::BeforeRestore,
            Reason::BeforeImport => Self::BeforeImport,
            Reason::BeforeFirmware => Self::BeforeFirmware,
        }
    }
}

/// One backup, as the Backups window lists it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct BackupEntry {
    /// Its file's name in the backups folder, which names it to the other commands.
    pub id: String,
    /// When it was made, in UTC, such as `2026-10-25T12:36:45Z`.
    pub created: String,
    /// Why.
    pub reason: BackupReason,
    /// The keyboard it was made from, by the name it reports.
    pub keyboard: String,
}

/// A backup compared with the keymap (design §9.1, §9.2): how the keymap would change if
/// it were imported, and what can't be.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportPreview {
    /// The backup's file name, such as `sofle.keywright`.
    pub file: String,
    /// When it was made, in UTC.
    pub created: String,
    /// Why.
    pub reason: BackupReason,
    /// The keyboard it was made from.
    pub keyboard: String,
    /// How the keymap would change, as the review sheet's rows.
    pub rows: Vec<ReviewRow>,
    /// How many of the rows are keys.
    pub keys: u32,
    /// What can't be applied, and why.
    pub cannot_apply: Vec<CannotApply>,
}

/// Something in a backup that the keyboard can't take (BAK-3). Layers are the backup's,
/// by name. In TypeScript, `{ kind: "missing-action", layer: "lower", … }` and so on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(
    tag = "kind",
    rename_all = "kebab-case",
    rename_all_fields = "camelCase"
)]
pub enum CannotApply {
    /// The backup's layers have another number of keys: it's for another kind of
    /// keyboard, so nothing can be applied.
    KeyCount {
        /// The backup's keys.
        backup: u32,
        /// The keyboard's.
        keyboard: u32,
    },
    /// The backup has more layers than the keyboard has room for, so the last ones are
    /// left out.
    TooManyLayers {
        /// The backup's layers.
        backup: u32,
        /// How many the keyboard has room for.
        keyboard: u32,
    },
    /// A key's action isn't in the keyboard's firmware, such as Mouse Move.
    MissingAction {
        /// The layer.
        layer: String,
        /// Where the key is, such as `Left half, row 2, key 1`.
        place: String,
        /// The action, by the name of its behavior.
        action: String,
    },
    /// A key turns on a layer that's left out.
    MissingLayer {
        /// The layer.
        layer: String,
        /// Where the key is.
        place: String,
        /// The layer it turns on.
        target: String,
    },
    /// A key's action can't be read.
    Unreadable {
        /// The layer.
        layer: String,
        /// Where the key is.
        place: String,
    },
}

/// A backup, by its file's name, read; or why it can't be.
type Read = Result<(String, Backup), AppError>;

/// A backup waiting in the Import preview to be applied or cancelled, or why the file
/// chosen can't be.
#[derive(Debug, Default)]
pub struct PendingImport(Mutex<Option<Read>>);

impl PendingImport {
    fn set(&self, read: Read) {
        if let Ok(mut pending) = self.0.lock() {
            *pending = Some(read);
        }
    }

    /// The pending backup, or why it can't be read.
    fn get(&self) -> Result<(String, Backup), AppError> {
        (self.0.lock().ok())
            .and_then(|pending| pending.clone())
            .unwrap_or(Err(AppError::NothingToImport))
    }

    fn is_set(&self) -> bool {
        self.0.lock().is_ok_and(|pending| pending.is_some())
    }

    fn clear(&self) {
        if let Ok(mut pending) = self.0.lock() {
            *pending = None;
        }
    }
}

/// A count, which fits: backups have at most 64 layers of 512 keys.
fn number(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

/// Why a backup can't be used, for the UI.
fn unreadable(error: &ReadError) -> AppError {
    match error {
        ReadError::NotFound => AppError::NoSuchBackup,
        ReadError::Io(_) | ReadError::Backup(_) => AppError::UnreadableBackup,
    }
}

impl AppState {
    /// The automatic backups, newest first. Files in the folder that can't be read are
    /// left out: Keywright wrote them all, so they'd have been changed by hand.
    pub fn backups(&self) -> Result<Vec<BackupEntry>, AppError> {
        let Some(folder) = &self.backups else {
            return Ok(Vec::new());
        };
        let paths = folder.list().map_err(|_| AppError::BackupsNotRead)?;
        Ok(paths
            .iter()
            .filter_map(|path| {
                let id = path.file_name()?.to_str()?.to_owned();
                let backup = backups::read(path).ok()?;
                Some(BackupEntry {
                    id,
                    created: backup.created.to_string(),
                    reason: backup.reason.into(),
                    keyboard: backup.keyboard.name,
                })
            })
            .collect())
    }

    /// The automatic backup with the file name `id`: only one in the backups folder.
    fn backup_file(&self, id: &str) -> Result<PathBuf, AppError> {
        let folder = self.backups.as_ref().ok_or(AppError::NoSuchBackup)?;
        let paths = folder.list().map_err(|_| AppError::BackupsNotRead)?;
        paths
            .into_iter()
            .find(|path| path.file_name().and_then(|name| name.to_str()) == Some(id))
            .ok_or(AppError::NoSuchBackup)
    }

    /// The automatic backup `id`, read.
    fn read_backup(&self, id: &str) -> Result<Backup, AppError> {
        backups::read(&self.backup_file(id)?).map_err(|error| unreadable(&error))
    }

    /// The automatic backup `id`, compared with the keymap.
    pub async fn backup_preview(&self, id: &str) -> Result<ImportPreview, AppError> {
        let backup = self.read_backup(id)?;
        self.preview(id, &backup).await
    }

    /// Makes the automatic backup `id` the pending import, for the Import preview.
    pub fn restore_backup(&self, id: &str) -> Result<(), AppError> {
        let backup = self.read_backup(id)?;
        self.import.set(Ok((id.to_owned(), backup)));
        Ok(())
    }

    /// Makes the backup file at `path` the pending import, or, if it can't be read, why.
    pub fn open_backup(&self, path: &Path) {
        let name = path
            .file_name()
            .map_or_else(String::new, |name| name.to_string_lossy().into_owned());
        let read = backups::read(path)
            .map(|backup| (name, backup))
            .map_err(|error| unreadable(&error));
        self.import.set(read);
    }

    /// Whether a backup is waiting for the Import preview, as when Keywright was opened
    /// with one before the window was ready to show it.
    pub fn import_pending(&self) -> bool {
        self.import.is_set()
    }

    /// The keymap as a backup file's text (BAK-1), with the name to suggest for it.
    pub async fn exported(&self) -> Result<(String, String), AppError> {
        let snapshot = self.session.snapshot().await?;
        let created =
            keywright_device::backups::created(SystemTime::now()).ok_or(AppError::ExportFailed)?;
        let exported = snapshot
            .backup(created, Reason::Export)
            .ok_or(AppError::NotReady)?;
        let rows = snapshot.support_pack().map_or(&[][..], |pack| &pack.rows);
        let name = backup::file_name(&exported.keyboard.name, &exported.created);
        Ok((name, exported.to_toml(rows)))
    }

    /// Shows the automatic backup `id` in Finder.
    pub fn show_backup_in_finder(&self, id: &str) -> Result<(), AppError> {
        tauri_plugin_opener::reveal_item_in_dir(self.backup_file(id)?)
            .map_err(|_| AppError::NotShownInFinder)
    }

    /// The pending import, compared with the keymap.
    pub async fn import_preview(&self) -> Result<ImportPreview, AppError> {
        let (file, backup) = self.import.get()?;
        self.preview(&file, &backup).await
    }

    /// Applies the pending import as one change to try, after the session backs the keymap
    /// up (SAF-3), unless that would strand the user and not `force` (SAF-2). It stays
    /// pending if it isn't applied.
    pub async fn apply_import(&self, force: bool) -> Result<(), AppError> {
        let (_, backup) = self.import.get()?;
        let edits = self
            .checked_edits(force, |keymap, behaviors| {
                Ok(backup.import(keymap, behaviors).edits)
            })
            .await?;
        self.session.import(edits).await?;
        self.import.clear();
        Ok(())
    }

    /// Forgets the pending import.
    pub fn cancel_import(&self) {
        self.import.clear();
    }

    /// `backup`, from the file named `file`, compared with the keymap being edited.
    async fn preview(&self, file: &str, backup: &Backup) -> Result<ImportPreview, AppError> {
        let snapshot = self.session.snapshot().await?;
        let (Some(keyboard), Some(editor)) = (&snapshot.keyboard, &snapshot.editor) else {
            return Err(AppError::NotReady);
        };
        let keymap = editor.current();
        let import = backup.import(keymap, &keyboard.behaviors);
        let preview = import.preview(keymap).map_err(|_| AppError::EditRefused)?;
        let pack = snapshot.support_pack();
        let rows = rows(
            preview.current(),
            preview.baseline(),
            &keyboard.behaviors,
            pack,
            &preview.changes(),
            &self.context().labels,
        );
        let keys = rows
            .iter()
            .filter(|row| matches!(row.key, RowKey::Key { .. }))
            .count();
        Ok(ImportPreview {
            file: file.to_owned(),
            created: backup.created.to_string(),
            reason: backup.reason.into(),
            keyboard: backup.keyboard.name.clone(),
            rows,
            keys: number(keys),
            cannot_apply: cannot_apply(&import, backup, pack),
        })
    }
}

/// What in `backup` can't be applied, in the order the import found it.
fn cannot_apply(import: &Import, backup: &Backup, pack: Option<&SupportPack>) -> Vec<CannotApply> {
    let layer = |id: LayerId| {
        backup
            .layers
            .iter()
            .enumerate()
            .find(|(_, layer)| layer.id == id)
            .map_or_else(
                || format!("Layer {}", id.0.saturating_add(1)),
                |(index, layer)| {
                    if layer.name.is_empty() {
                        format!("Layer {}", index.saturating_add(1))
                    } else {
                        layer.name.clone()
                    }
                },
            )
    };
    let place = |position: usize| {
        pack.and_then(|pack| pack.key_place(position))
            .unwrap_or_else(|| format!("Key {}", position.saturating_add(1)))
    };
    import
        .unresolved
        .iter()
        .map(|unresolved| match unresolved {
            Unresolved::KeyCountMismatch { backup, keyboard } => CannotApply::KeyCount {
                backup: number(*backup),
                keyboard: number(*keyboard),
            },
            Unresolved::TooManyLayers { backup, keyboard } => CannotApply::TooManyLayers {
                backup: number(*backup),
                keyboard: number(*keyboard),
            },
            Unresolved::MissingBehavior {
                layer: id,
                position,
                behavior,
            } => CannotApply::MissingAction {
                layer: layer(*id),
                place: place(*position),
                action: behavior.clone(),
            },
            Unresolved::MissingLayer {
                layer: id,
                position,
                target,
            } => CannotApply::MissingLayer {
                layer: layer(*id),
                place: place(*position),
                target: layer(*target),
            },
            Unresolved::Unreadable {
                layer: id,
                position,
                ..
            } => CannotApply::Unreadable {
                layer: layer(*id),
                place: place(*position),
            },
        })
        .collect()
}

/// Opens the Backups window, or brings it to the front if it's open (⇧⌘B, design §9.1).
///
/// # Errors
///
/// Fails if the window can't be made.
pub fn open_window<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window(WINDOW) {
        window.unminimize()?;
        window.show()?;
        return window.set_focus();
    }
    // The bundled UI, which draws the Backups window for this address.
    WebviewWindowBuilder::new(app, WINDOW, WebviewUrl::App("index.html#backups".into()))
        .title("Backups")
        .inner_size(860.0, 540.0)
        .min_inner_size(640.0, 400.0)
        .center()
        .focused(true)
        .build()?;
    Ok(())
}

/// The automatic backups, newest first (BAK-4), for the Backups window.
///
/// # Errors
///
/// Fails if the backups folder can't be read.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn backups(app: tauri::State<'_, AppState>) -> Result<Vec<BackupEntry>, AppError> {
    app.backups()
}

/// An automatic backup compared with the keymap being edited, for the Backups window's
/// preview (design §9.1).
///
/// # Errors
///
/// Fails if there's no such backup, it can't be read, or the keymap isn't loaded.
#[tauri::command]
#[specta::specta]
pub async fn backup_preview(
    app: tauri::State<'_, AppState>,
    id: String,
) -> Result<ImportPreview, AppError> {
    app.backup_preview(&id).await
}

/// _Restore…_ (design §9.1): makes an automatic backup the pending import, brings the main
/// window forward, and tells it to show the Import preview.
///
/// # Errors
///
/// Fails if there's no such backup, or it can't be read.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn restore_backup(
    handle: AppHandle,
    app: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    app.restore_backup(&id)?;
    request_import(&handle);
    Ok(())
}

/// _Show in Finder_ (design §9.1): shows an automatic backup's file in Finder.
///
/// # Errors
///
/// Fails if there's no such backup, or Finder doesn't show it.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn show_backup_in_finder(app: tauri::State<'_, AppState>, id: String) -> Result<(), AppError> {
    app.show_backup_in_finder(&id)
}

/// The pending import compared with the keymap being edited, for the Import preview
/// (design §9.2).
///
/// # Errors
///
/// Fails if nothing is pending, or the keymap isn't loaded.
#[tauri::command]
#[specta::specta]
pub async fn import_preview(app: tauri::State<'_, AppState>) -> Result<ImportPreview, AppError> {
    app.import_preview().await
}

/// _Apply as changes_ (design §9.2, BAK-3): applies the pending import as one change to
/// try, after backing the keymap up. Nothing is saved until the person saves.
///
/// # Errors
///
/// Fails if nothing is pending, the keymap can't be changed now, the change would strand
/// the user and not `force`, the keymap can't be backed up first, or the keyboard doesn't
/// carry it out.
#[tauri::command]
#[specta::specta]
pub async fn apply_import(app: tauri::State<'_, AppState>, force: bool) -> Result<(), AppError> {
    app.apply_import(force).await
}

/// Tells the main window to show the Import preview, bringing it forward.
fn request_import<R: Runtime>(handle: &AppHandle<R>) {
    crate::show_main_window(handle);
    // The main window is there for as long as the app runs.
    let _ = ImportRequested.emit(handle);
}

/// Backup files opened from Finder while Keywright was starting, kept until it has: macOS
/// hands over the files that start an app before the app has finished launching.
#[derive(Debug, Default)]
pub struct OpenedEarly(Mutex<Vec<PathBuf>>);

/// Opens backup files from Finder (BAK-6), as when one is double-clicked: the first becomes
/// the pending import, in the Import preview. Keywright imports one at a time. Files opened
/// while Keywright is starting wait for [`open_early_files`].
pub fn open_files<R: Runtime>(handle: &AppHandle<R>, paths: &[PathBuf]) {
    let Some(app) = handle.try_state::<AppState>() else {
        if let Some(early) = handle.try_state::<OpenedEarly>()
            && let Ok(mut waiting) = early.0.lock()
        {
            waiting.extend_from_slice(paths);
        }
        return;
    };
    let Some(path) = paths.first() else {
        return;
    };
    app.open_backup(path);
    request_import(handle);
}

/// Opens the backup files opened from Finder while Keywright was starting. Call it once the
/// commands' state is there.
pub fn open_early_files<R: Runtime>(handle: &AppHandle<R>) {
    let paths = (handle.try_state::<OpenedEarly>())
        .and_then(|early| {
            early
                .0
                .lock()
                .ok()
                .map(|mut waiting| std::mem::take(&mut *waiting))
        })
        .unwrap_or_default();
    open_files(handle, &paths);
}

/// Whether a backup is waiting for the Import preview: the main window asks as it loads, in
/// case Keywright was opened with one.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn import_pending(app: tauri::State<'_, AppState>) -> bool {
    app.import_pending()
}

/// File › Import… (⌘O, BAK-3): shows the open panel, then the Import preview for the file
/// chosen. Nothing happens if the person cancels.
#[tauri::command]
#[specta::specta]
pub async fn import_file(
    handle: AppHandle,
    app: tauri::State<'_, AppState>,
) -> Result<(), AppError> {
    let (chosen, path) = oneshot::channel();
    let mut panel = handle
        .dialog()
        .file()
        .add_filter("Keywright backup", &[backup::EXTENSION]);
    if let Some(window) = handle.get_webview_window(crate::MAIN_WINDOW) {
        panel = panel.set_parent(&window);
    }
    panel.pick_file(move |path| {
        let _ = chosen.send(path.and_then(|path| path.into_path().ok()));
    });
    if let Some(path) = path.await.ok().flatten() {
        app.open_backup(&path);
        request_import(&handle);
    }
    Ok(())
}

/// File › Export… (⇧⌘E, BAK-1): writes the keymap the keyboard holds now, changes being
/// tried included, to a `.keywright` file where the save panel says. Gives the file's name,
/// or `null` if the person cancelled.
///
/// # Errors
///
/// Fails if the keymap isn't loaded, or the file can't be written.
#[tauri::command]
#[specta::specta]
pub async fn export_keymap(
    handle: AppHandle,
    app: tauri::State<'_, AppState>,
) -> Result<Option<String>, AppError> {
    let (name, text) = app.exported().await?;
    let (chosen, path) = oneshot::channel();
    let mut panel = handle
        .dialog()
        .file()
        .set_file_name(&name)
        .add_filter("Keywright backup", &[backup::EXTENSION]);
    if let Some(window) = handle.get_webview_window(crate::MAIN_WINDOW) {
        panel = panel.set_parent(&window);
    }
    panel.save_file(move |path| {
        let _ = chosen.send(path.and_then(|path| path.into_path().ok()));
    });
    let Some(path) = path.await.ok().flatten() else {
        return Ok(None);
    };
    std::fs::write(&path, text).map_err(|_| AppError::ExportFailed)?;
    Ok(path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned()))
}

/// _Cancel_ in the Import preview: forgets the pending import.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn cancel_import(app: tauri::State<'_, AppState>) {
    app.cancel_import();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use keywright_core::backup::Reason;
    use keywright_device::{Backups, Connection, Peer, Session, State};
    use keywright_sim::Simulator;

    use super::{AppState, BackupReason, CannotApply};
    use crate::ipc::action::Action;
    use crate::ipc::error::AppError;
    use crate::ipc::review::RowKey;

    /// A backups folder of its own, deleted afterwards.
    struct Folder(PathBuf);

    impl Folder {
        fn new(test: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "keywright-app-backups-{test}-{}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&path);
            Self(path)
        }

        fn backups(&self) -> Backups {
            Backups::new(&self.0, Backups::DEFAULT_KEEP)
        }
    }

    impl Drop for Folder {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// An unlocked Sofle, whose session backs up into `folder`: first, as it connects.
    async fn ready_sofle(folder: &Folder) -> AppState {
        let session = Session::with_backups(folder.backups());
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
        AppState::new(session).with_backups(Some(folder.backups()))
    }

    #[tokio::test]
    async fn restoring_an_automatic_backup_tries_exactly_what_it_takes_back() {
        let folder = Folder::new("restore");
        let app = ready_sofle(&folder).await;
        let listed = app.backups().unwrap();
        assert_eq!(listed.len(), 1);
        let first = &listed[0];
        assert_eq!(first.reason, BackupReason::FirstConnect);
        assert_eq!(first.keyboard, "Sofle Choc BT");

        // Two keys and a layer's name change, and are saved.
        app.set_action(0, vec![13, 14], Action::Nothing, false)
            .await
            .unwrap();
        app.rename_layer(1, "Numbers".to_owned()).await.unwrap();
        app.session.save().await.unwrap();
        assert_eq!(app.review().await.unwrap(), []);

        let preview = app.backup_preview(&first.id).await.unwrap();
        let what: Vec<RowKey> = preview.rows.iter().map(|row| row.key).collect();
        assert_eq!(
            what,
            [
                RowKey::Key {
                    layer: 0,
                    position: 13
                },
                RowKey::Key {
                    layer: 0,
                    position: 14
                },
                RowKey::Renamed { layer: 1 },
            ]
        );
        assert_eq!(preview.keys, 2);
        assert_eq!(preview.cannot_apply, []);
        assert_eq!(preview.rows[0].before, "Does nothing");
        assert_eq!(preview.rows[0].after, "Q");
        assert_eq!(
            (
                preview.rows[2].before.as_str(),
                preview.rows[2].after.as_str()
            ),
            ("Numbers", "lower")
        );

        app.restore_backup(&first.id).unwrap();
        assert_eq!(app.import_preview().await.unwrap(), preview);
        app.apply_import(false).await.unwrap();
        // The changes being tried are the preview's rows, nothing more, and nothing is
        // saved: the keyboard keeps the keymap it had.
        assert_eq!(app.review().await.unwrap(), preview.rows);
        assert_eq!(app.import_preview().await, Err(AppError::NothingToImport));
        // The session backed up the keymap before importing.
        let reasons: Vec<BackupReason> = app
            .backups()
            .unwrap()
            .iter()
            .map(|entry| entry.reason)
            .collect();
        assert_eq!(
            reasons,
            [BackupReason::BeforeImport, BackupReason::FirstConnect]
        );
        // Undo takes the whole import back.
        app.session.undo().await.unwrap();
        assert_eq!(app.review().await.unwrap(), []);
    }

    #[tokio::test]
    async fn what_cant_be_applied_is_listed_with_why() {
        let folder = Folder::new("unresolved");
        let app = ready_sofle(&folder).await;
        let first = &app.backups().unwrap()[0];
        let mut backup = keywright_device::backups::read(&folder.0.join(&first.id)).unwrap();
        backup.reason = Reason::BeforeImport;
        backup.layers[0].bindings[1] = "other \"Mouse Move\" 0x1 0x0".to_owned();
        backup.layers[0].bindings[2] = "kp NOT_A_KEY".to_owned();
        backup.layers[0].bindings[3] = "kp X".to_owned();
        let path = folder.backups().write(&backup, &[]).unwrap();
        let id = path.file_name().unwrap().to_str().unwrap();

        let preview = app.backup_preview(id).await.unwrap();
        assert_eq!(
            preview.cannot_apply,
            [
                CannotApply::MissingAction {
                    layer: "default".to_owned(),
                    place: "Left half, row 1, key 2".to_owned(),
                    action: "Mouse Move".to_owned(),
                },
                CannotApply::Unreadable {
                    layer: "default".to_owned(),
                    place: "Left half, row 1, key 3".to_owned(),
                },
            ]
        );
        // The key that can be applied is still a row.
        assert_eq!(preview.keys, 1);
        assert_eq!(preview.rows[0].after, "X");
    }

    #[tokio::test]
    async fn only_backups_in_the_folder_can_be_named() {
        let folder = Folder::new("names");
        let app = ready_sofle(&folder).await;
        for id in ["../settings.json", "/etc/hosts", "missing.keywright"] {
            assert_eq!(
                app.backup_preview(id).await,
                Err(AppError::NoSuchBackup),
                "{id}"
            );
            assert_eq!(app.restore_backup(id), Err(AppError::NoSuchBackup), "{id}");
        }
        assert_eq!(
            app.apply_import(false).await,
            Err(AppError::NothingToImport)
        );
    }

    #[tokio::test]
    async fn an_export_edited_as_text_imports_as_the_edit() {
        let folder = Folder::new("export");
        let app = ready_sofle(&folder).await;
        let (name, text) = app.exported().await.unwrap();
        assert!(name.starts_with("sofle-choc-bt-"), "{name}");
        assert!(name.ends_with(".keywright"), "{name}");
        std::fs::create_dir_all(&folder.0).unwrap();
        let path = folder.0.join("mine.keywright");

        // As exported, it's the keymap.
        std::fs::write(&path, &text).unwrap();
        app.open_backup(&path);
        assert!(app.import_pending());
        let preview = app.import_preview().await.unwrap();
        assert_eq!(preview.file, "mine.keywright");
        assert_eq!((preview.rows.len(), preview.cannot_apply.len()), (0, 0));

        // Edited in a text editor: Q becomes X.
        std::fs::write(&path, text.replacen("\"kp Q\",", "\"kp X\",", 1)).unwrap();
        app.open_backup(&path);
        let preview = app.import_preview().await.unwrap();
        assert_eq!(preview.keys, 1);
        assert_eq!(
            (
                preview.rows[0].before.as_str(),
                preview.rows[0].after.as_str()
            ),
            ("Q", "X")
        );
        app.apply_import(false).await.unwrap();
        assert_eq!(app.review().await.unwrap(), preview.rows);
        assert!(!app.import_pending());
    }

    #[tokio::test]
    async fn a_file_that_cant_be_read_says_why_in_the_preview() {
        let folder = Folder::new("unreadable");
        let app = ready_sofle(&folder).await;
        std::fs::create_dir_all(&folder.0).unwrap();
        let notes = folder.0.join("notes.keywright");
        std::fs::write(&notes, "Not a backup.").unwrap();
        app.open_backup(&notes);
        assert!(app.import_pending());
        assert_eq!(app.import_preview().await, Err(AppError::UnreadableBackup));
        assert_eq!(
            app.apply_import(false).await,
            Err(AppError::UnreadableBackup)
        );
        app.open_backup(&folder.0.join("gone.keywright"));
        assert_eq!(app.import_preview().await, Err(AppError::NoSuchBackup));
        app.cancel_import();
        assert!(!app.import_pending());
    }

    #[tokio::test]
    async fn without_a_folder_there_are_no_backups() {
        let folder = Folder::new("none");
        let app = ready_sofle(&folder).await.with_backups(None);
        assert_eq!(app.backups().unwrap(), []);
        assert_eq!(
            app.backup_preview("anything.keywright").await,
            Err(AppError::NoSuchBackup)
        );
    }
}
