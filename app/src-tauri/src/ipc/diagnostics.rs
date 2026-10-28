//! _Copy Diagnostics_ (NFR-4, tech spec §16): a report for a bug report, which the person
//! reads before pasting it anywhere. Settings › Advanced copies it and shows it. Help ›
//! _Report an Issue…_ shows it in a sheet, then copies it and opens a new issue on GitHub,
//! in the browser. Keywright itself sends nothing anywhere.
//!
//! The report ([`diagnostics`]) says which Keywright and macOS this is, the keyboard, how
//! it's connected and what the session is doing, then the log's last lines
//! ([`crate::logs`]). It's English Markdown, for the issue tracker, rather than words the
//! UI shows. Serial numbers, other long hex IDs and the home folder in paths are taken out
//! ([`Redaction`]): from the report, and again from what [`copy_diagnostics`] copies, so
//! nothing the webview sends can put them on the clipboard.

use std::fmt::Write as _;
use std::path::Path;
use std::time::Duration;

use keywright_device::{Snapshot, State};
use keywright_platform::{bluetooth, pasteboard, paths, system};
use tauri::{AppHandle, Manager};
use tauri_plugin_opener::OpenerExt;

use super::AppState;
use super::bluetooth::BluetoothPermission;
use super::error::AppError;
use super::session::{self, SessionStatus};

/// How many of the log's last lines the report has.
const LOG_LINES: usize = 100;

/// How long the report waits for the session to say what it holds. The session may be
/// busy, as with a keyboard that doesn't answer, which is when a report helps most: the
/// report then has its state alone.
const SESSION_WAIT: Duration = Duration::from_secs(2);

/// GitHub's page for a new issue in Keywright's repository.
const NEW_ISSUE: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new");

/// What a serial number becomes.
const SERIAL: &str = "[serial]";

/// What another long hex ID becomes, such as a Bluetooth device's UUID.
const ID: &str = "[id]";

/// What someone else's home folder becomes, after `/Users/`.
const USER: &str = "[user]";

/// The fewest bytes of a serial number taken out wherever they appear: fewer would take
/// out ordinary numbers too.
const SERIAL_BYTES: usize = 4;

/// The fewest hex digits in an ID taken out wherever it appears.
const ID_DIGITS: usize = 12;

/// The report on Keywright now, for the person to read before copying it.
#[tauri::command]
#[specta::specta]
pub async fn diagnostics(app: AppHandle) -> String {
    let (report, redaction) = gather(&app.state::<AppState>(), paths::logs_dir().as_deref()).await;
    redaction.apply(&report.text())
}

/// Copies `report`, as [`diagnostics`] gave it, to the clipboard, taking out anything
/// private again.
///
/// # Errors
///
/// Fails if macOS doesn't take the text.
#[tauri::command]
#[specta::specta]
pub async fn copy_diagnostics(app: AppHandle, report: String) -> Result<(), AppError> {
    let copied = to_copy(&app.state::<AppState>(), &report).await;
    pasteboard::copy_text(&copied).map_err(|_| AppError::NotCopied)
}

/// What [`copy_diagnostics`] copies of `report`: it, without anything private.
async fn to_copy(app: &AppState, report: &str) -> String {
    let (_, redaction) = session_now(app).await;
    redaction.apply(report)
}

/// Opens a new issue in Keywright's repository on GitHub, in the browser, for the report to
/// be pasted in.
///
/// # Errors
///
/// Fails if the browser doesn't open.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn open_new_issue(app: AppHandle) -> Result<(), AppError> {
    app.opener()
        .open_url(NEW_ISSUE, None::<&str>)
        .map_err(|_| AppError::PageNotOpened)
}

/// What the report says now, with the log's last lines from `logs`, and what to take out
/// of it.
async fn gather(app: &AppState, logs: Option<&Path>) -> (Report, Redaction) {
    let (session, redaction) = session_now(app).await;
    let report = Report {
        macos: system::macos_version_name(),
        session,
        bluetooth: bluetooth::permission().into(),
        logs: logs.map(|dir| {
            (
                dir.display().to_string(),
                crate::logs::recent_lines(dir, LOG_LINES),
            )
        }),
    };
    (report, redaction)
}

/// The session's status now, or `None` once it has stopped, and what to take out of text:
/// the home folder, and its keyboard's serial number.
async fn session_now(app: &AppState) -> (Option<SessionStatus>, Redaction) {
    let labels = &app.context().labels;
    let (status, serial) = match tokio::time::timeout(SESSION_WAIT, app.session.snapshot()).await {
        Ok(Ok(snapshot)) => (
            Some(session::status(&snapshot.state, Some(&snapshot), labels)),
            serial_number(&snapshot.state, Some(&snapshot)),
        ),
        Ok(Err(_)) => (None, Vec::new()),
        // The session is busy: its state alone.
        Err(_) => {
            let state = app.session.state();
            (
                Some(session::status(&state, None, labels)),
                serial_number(&state, None),
            )
        }
    };
    let home = std::env::var("HOME").ok();
    (status, Redaction::new(home.as_deref(), &serial))
}

/// The keyboard's serial number, as it said in `state`, or as `snapshot` last read it.
fn serial_number(state: &State, snapshot: Option<&Snapshot>) -> Vec<u8> {
    let read = snapshot
        .and_then(|snapshot| snapshot.keyboard.as_ref())
        .map(|keyboard| &keyboard.identity);
    match (read, state) {
        (Some(identity), _)
        | (
            None,
            State::Locked {
                device: identity, ..
            },
        ) => identity.serial_number.clone(),
        (None, _) => Vec::new(),
    }
}

/// What the report says.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Report {
    /// The version of macOS, if it's known.
    macos: Option<String>,
    /// The session's status, or `None` once it has stopped.
    session: Option<SessionStatus>,
    /// What macOS says about Keywright using Bluetooth.
    bluetooth: BluetoothPermission,
    /// The log folder and its last lines, oldest first, or `None` without a folder.
    logs: Option<(String, Vec<String>)>,
}

impl Report {
    /// The report, as Markdown.
    fn text(&self) -> String {
        let build = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };
        let mut text = String::from("### Diagnostics\n\n");
        // Writing to a `String` can't fail.
        let _ = writeln!(
            text,
            "- Keywright {} ({build} build, {})",
            env!("CARGO_PKG_VERSION"),
            std::env::consts::ARCH
        );
        let _ = writeln!(
            text,
            "- macOS {}",
            self.macos.as_deref().unwrap_or("unknown")
        );
        match &self.session {
            Some(status) => {
                let keyboard = match (&status.keyboard, &status.pack) {
                    (Some(name), Some(pack)) => format!("{name} (support pack {pack})"),
                    (Some(name), None) => format!("{name} (no support pack)"),
                    (None, _) => "none".to_owned(),
                };
                let _ = writeln!(text, "- Keyboard: {keyboard}");
                let _ = match status.transport {
                    Some(transport) => writeln!(text, "- Connection: {transport:?}"),
                    None => writeln!(text, "- Connection: none"),
                };
                let _ = writeln!(
                    text,
                    "- Session: {:?}, {} changes being tried",
                    status.state, status.changes
                );
            }
            None => {
                let _ = writeln!(text, "- Session: stopped");
            }
        }
        let _ = writeln!(text, "- Bluetooth: {:?}", self.bluetooth);
        text.push_str("\n#### Log\n\n");
        match &self.logs {
            Some((_, lines)) if lines.is_empty() => text.push_str("Nothing logged yet.\n"),
            Some((folder, lines)) => {
                let _ = writeln!(text, "The last {} lines, from {folder}:", lines.len());
                text.push_str("\n```text\n");
                for line in lines {
                    text.push_str(line);
                    text.push('\n');
                }
                text.push_str("```\n");
            }
            None => text.push_str("Keywright has no log folder.\n"),
        }
        text
    }
}

/// What the report leaves out: the home folder in paths, and serial numbers and other long
/// hex IDs, which identify a keyboard or a Mac.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Redaction {
    /// The home folder, such as `/Users/someone`, which becomes `~`.
    home: Option<String>,
    /// The keyboard's serial number, as text may show it: in hex, upper and lower case, and
    /// as Rust shows a list of its bytes.
    serials: Vec<String>,
}

impl Redaction {
    /// Takes out the home folder `home`, and the serial number `serial`.
    fn new(home: Option<&str>, serial: &[u8]) -> Self {
        let home = home
            .map(|home| home.trim_end_matches('/'))
            .filter(|home| !home.is_empty())
            .map(str::to_owned);
        let mut serials = Vec::new();
        if serial.len() >= SERIAL_BYTES {
            let mut hex = String::with_capacity(serial.len() * 2);
            for byte in serial {
                // Writing to a `String` can't fail.
                let _ = write!(hex, "{byte:02X}");
            }
            serials.push(hex.to_lowercase());
            serials.push(hex);
            serials.push(format!("{serial:?}"));
        }
        Self { home, serials }
    }

    /// `text`, without what it leaves out.
    fn apply(&self, text: &str) -> String {
        let mut text = match &self.home {
            Some(home) => replace_folder(text, home, "~"),
            None => text.to_owned(),
        };
        text = other_homes(&text);
        for serial in &self.serials {
            text = text.replace(serial.as_str(), SERIAL);
        }
        long_hex_ids(&text)
    }
}

/// Whether `c` can be part of a file or folder name, so that a folder followed by it is
/// another folder: `/Users/someone` in `/Users/someone2`.
fn in_name(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '-' | '_' | '.')
}

/// `text`, with the folder `folder` replaced by `with` wherever it's a whole folder, and
/// not the start of another name.
fn replace_folder(text: &str, folder: &str, with: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(folder) {
        let after = &rest[at + folder.len()..];
        out.push_str(&rest[..at]);
        if after.chars().next().is_some_and(in_name) {
            out.push_str(folder);
        } else {
            out.push_str(with);
        }
        rest = after;
    }
    out.push_str(rest);
    out
}

/// `text`, with each other account's name in a path under `/Users/` taken out. The shared
/// folder stays.
fn other_homes(text: &str) -> String {
    const USERS: &str = "/Users/";
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(USERS) {
        out.push_str(&rest[..at + USERS.len()]);
        let after = &rest[at + USERS.len()..];
        let end = after.find(|c| !in_name(c)).unwrap_or(after.len());
        let name = &after[..end];
        out.push_str(if name.is_empty() || name == "Shared" {
            name
        } else {
            USER
        });
        rest = &after[end..];
    }
    out.push_str(rest);
    out
}

/// `text`, with each word of [`ID_DIGITS`] or more hex digits, at least one of them a
/// digit, replaced by [`ID`]: alone, after `0x`, or in groups joined by `-` or `:`, as in
/// UUIDs and addresses.
fn long_hex_ids(text: &str) -> String {
    let is_part = |c: char| c.is_ascii_hexdigit() || matches!(c, '-' | ':');
    let joins = ['-', ':'];
    let mut out = String::with_capacity(text.len());
    let mut copied = 0;
    let mut from = 0;
    // Hex digits and joins are ASCII, so each run's ends are on character boundaries.
    while let Some(found) = text[from..].find(is_part) {
        let start = from + found;
        let end = text[start..]
            .find(|c| !is_part(c))
            .map_or(text.len(), |length| start + length);
        let run = &text[start..end];
        // The joins at either end belong to what's around it, such as `id: `.
        let id_start = start + (run.len() - run.trim_start_matches(joins).len());
        let id = run.trim_matches(joins);
        let id_end = id_start + id.len();
        if is_id(id) && stands_alone(text, id_start, id_end) {
            out.push_str(&text[copied..id_start]);
            out.push_str(ID);
            copied = id_end;
        }
        from = end;
    }
    out.push_str(&text[copied..]);
    out
}

/// Whether `run`, hex digits and joins, is long enough to be an ID, and has a digit, as
/// IDs do and words don't.
fn is_id(run: &str) -> bool {
    run.chars().filter(char::is_ascii_hexdigit).count() >= ID_DIGITS
        && run.chars().any(|c| c.is_ascii_digit())
}

/// Whether the bytes of `text` from `start` to `end` are a word of their own, or follow
/// `0x`.
fn stands_alone(text: &str, start: usize, end: usize) -> bool {
    let in_word = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
    let before = &text[..start];
    (!in_word(before.chars().next_back()) || before.ends_with("0x"))
        && !in_word(text[end..].chars().next())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use keywright_device::{Connection, Peer, Session, State};
    use keywright_sim::Simulator;

    use super::{AppState, Redaction, Report, gather, long_hex_ids, to_copy};

    /// The simulated Sofle's serial number, as text may show it.
    const SERIALS: [&str; 3] = [
        "5A3C910E224BD708",
        "5a3c910e224bd708",
        "[90, 60, 145, 14, 34, 75, 215, 8]",
    ];

    /// The simulated Sofle, connected, and unlocked if `unlocked`.
    async fn sofle(unlocked: bool) -> AppState {
        let session = Session::start();
        let keyboard = Simulator::sofle().unwrap();
        if unlocked {
            keyboard.press_unlock_key();
        }
        session
            .connect(Connection::new(keyboard.connect(), Peer::Simulated))
            .await
            .unwrap();
        session
            .wait_for(|state| {
                if unlocked {
                    *state == State::Ready
                } else {
                    matches!(state, State::Locked { .. })
                }
            })
            .await
            .unwrap();
        AppState::new(session)
    }

    /// A log folder for one test, holding `lines`. The test removes it.
    fn logs(name: &str, lines: &[String]) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "keywright-diagnostics-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("Keywright.2026-10-28.log"), lines.join("\n")).unwrap();
        dir
    }

    /// The home folder, which the tests' log lines name.
    fn home() -> String {
        std::env::var("HOME").unwrap()
    }

    /// Lines naming the keyboard's serial number, in each way text may show it, and paths
    /// in the home folder.
    fn private_lines() -> Vec<String> {
        let home = home();
        let mut lines: Vec<String> = SERIALS
            .iter()
            .map(|serial| format!("2026-10-28T08:40:19.587123Z  INFO keywright: serial {serial}"))
            .collect();
        lines.push(format!(
            "2026-10-28T08:40:20.000000Z  WARN keywright: couldn't read {home}/Library/Application Support/dev.zunh.keywright/settings.json"
        ));
        lines.push(format!("panicked at {home}/.cargo/registry/src/x.rs:1:2:"));
        lines
    }

    /// Whether `text` holds the serial number or the home folder.
    fn has_private(text: &str) -> bool {
        SERIALS.iter().any(|serial| text.contains(serial)) || text.contains(&home())
    }

    #[tokio::test]
    async fn serials_and_home_paths_never_reach_the_copied_report() {
        for unlocked in [true, false] {
            let app = sofle(unlocked).await;
            let lines = private_lines();
            let dir = logs(&format!("private-{unlocked}"), &lines);

            let (report, redaction) = gather(&app, Some(&dir)).await;
            let raw = report.text();
            let shown = redaction.apply(&raw);
            // Even with what the webview sends added to it.
            let tampered = format!("{shown}\n{}", lines.join("\n"));
            let copied = to_copy(&app, &tampered).await;
            fs::remove_dir_all(&dir).unwrap();

            assert!(has_private(&raw), "the log lines name them");
            assert!(!has_private(&shown), "{shown}");
            assert!(!has_private(&copied), "{copied}");
            assert!(copied.contains("serial [serial]"), "{copied}");
            assert!(
                copied.contains("~/Library/Application Support/dev.zunh.keywright/settings.json"),
                "{copied}"
            );
        }
    }

    #[tokio::test]
    async fn the_report_says_which_keywright_macos_and_keyboard_and_the_last_lines() {
        let app = sofle(true).await;
        let lines: Vec<String> = (1..=150).map(|n| format!("line {n}")).collect();
        let dir = logs("report", &lines);
        let (report, redaction) = gather(&app, Some(&dir)).await;
        let text = redaction.apply(&report.text());
        fs::remove_dir_all(&dir).unwrap();

        assert!(
            text.starts_with("### Diagnostics\n\n- Keywright "),
            "{text}"
        );
        let header: Vec<&str> = text.lines().skip(4).take(4).collect();
        assert_eq!(
            header,
            [
                "- Keyboard: Sofle Choc BT (support pack keebart-sofle-choc-pro-bt)",
                "- Connection: Simulated",
                "- Session: Ready, 0 changes being tried",
                &format!("- Bluetooth: {:?}", report.bluetooth),
            ]
        );
        assert!(text.contains("The last 100 lines, from "), "{text}");
        assert!(text.contains("```text\nline 51\n"), "{text}");
        assert!(text.ends_with("line 150\n```\n"), "{text}");
    }

    #[tokio::test]
    async fn without_a_keyboard_or_a_log_folder_the_report_says_so() {
        let app = AppState::new(Session::start());
        let (report, _) = gather(&app, None).await;
        let text = report.text();
        assert!(
            text.contains("- Keyboard: none\n- Connection: none\n"),
            "{text}"
        );
        assert!(
            text.contains("- Session: NoDevice, 0 changes being tried\n"),
            "{text}"
        );
        assert!(text.ends_with("Keywright has no log folder.\n"), "{text}");

        let stopped = Report {
            session: None,
            logs: Some(("~/Library/Logs/Keywright".to_owned(), Vec::new())),
            ..report
        };
        let text = stopped.text();
        assert!(text.contains("- Session: stopped\n"), "{text}");
        assert!(text.ends_with("Nothing logged yet.\n"), "{text}");
    }

    #[test]
    fn the_home_folder_becomes_a_tilde_and_other_accounts_go() {
        let redaction = Redaction::new(Some("/Users/someone/"), &[]);
        assert_eq!(
            redaction.apply(
                "/Users/someone/Library, /Users/someone2/x, /Users/Shared/y, file:///Users/someone"
            ),
            "~/Library, /Users/[user]/x, /Users/Shared/y, file://~"
        );
    }

    #[test]
    fn long_hex_ids_go_and_dates_numbers_and_words_stay() {
        let line = "2026-10-28T08:40:19.587123Z id 0F6E1C2A-1B2C-4D5E-8F90-A1B2C3D4E5F6, \
                    at 0x7ffee3b0c8a8: request 123 of keebart-sofle-choc-pro-bt, \
                    deadbeefcafebabe, 5a3c910e";
        assert_eq!(
            long_hex_ids(line),
            "2026-10-28T08:40:19.587123Z id [id], at 0x[id]: request 123 of \
             keebart-sofle-choc-pro-bt, deadbeefcafebabe, 5a3c910e"
        );
    }

    #[test]
    fn a_short_serial_number_is_left_to_the_ids_rule() {
        let redaction = Redaction::new(None, &[0x12, 0x34]);
        assert_eq!(redaction.apply("1234 and 0x1234"), "1234 and 0x1234");
    }

    #[test]
    fn taking_out_twice_changes_nothing_more() {
        let redaction = Redaction::new(Some("/Users/someone"), &[0x5a, 0x3c, 0x91, 0x0e]);
        let once = redaction
            .apply("/Users/someone/x 5A3C910E /Users/other/y 0F6E1C2A-1B2C-4D5E-8F90-A1B2C3D4E5F6");
        assert_eq!(once, "~/x [serial] /Users/[user]/y [id]");
        assert_eq!(redaction.apply(&once), once);
    }
}
