//! `keywright status`: the keyboards connected, how, whether they're locked, and whether
//! their changes are saved (CLI-2, CON-4).
//!
//! Status only looks: it doesn't wait for the keyboard to be unlocked, and keeps no
//! automatic backups.

use std::fmt::Write as _;

use keywright_device::State;
use serde::Serialize;

use crate::failure::{Exit, Failure, Kind};
use crate::keyboards::{Changes, Found, Link, Source};
use crate::output::{print, print_json};
use crate::words;

/// What `status` reports about one keyboard.
#[derive(Debug, Serialize)]
struct Status {
    /// Its name, if it answered.
    name: Option<String>,
    /// The model, from its support pack, if Keywright knows it.
    model: Option<String>,
    /// How the CLI reaches it.
    connection: Link,
    /// `locked` or `unlocked`, or what stops Keywright editing it: `other-half`,
    /// `no-live-editing`, `incompatible`, `unavailable` or `disconnected`.
    state: &'static str,
    /// Once it's unlocked: `saved`, or `trying` when it has changes it hasn't saved.
    changes: Option<Changes>,
    /// The left half's battery level in percent, which only Bluetooth reports (BAR-2).
    battery: Option<u8>,
    /// How to unlock it, while it's locked.
    unlock: Option<String>,
    /// What stops Keywright editing it, in words.
    problem: Option<String>,
}

/// What `status --json` prints.
#[derive(Serialize)]
struct Report<'a> {
    keyboards: &'a [Status],
}

pub(crate) async fn run(source: Source, json: bool) -> Result<Exit, Failure> {
    let found = source.connect_all(None).await?;
    let (statuses, kinds): (Vec<Status>, Vec<Option<Kind>>) = found.iter().map(status).unzip();
    if json {
        print_json(&Report {
            keyboards: &statuses,
        })?;
    } else if statuses.is_empty() {
        print(words::NO_KEYBOARD)?;
    } else {
        let texts: Vec<String> = statuses.iter().map(text).collect();
        print(texts.join("\n"))?;
    }
    // Done if any keyboard can be edited. Otherwise, the first one's problem says why not.
    if kinds.iter().any(Option::is_none) {
        return Ok(Exit::Done);
    }
    Ok(Exit::Explained(
        kinds
            .into_iter()
            .flatten()
            .next()
            .unwrap_or(Kind::NoKeyboard),
    ))
}

/// A keyboard's status, and the kind of problem that stops Keywright editing it, if any.
fn status(found: &Found) -> (Status, Option<Kind>) {
    let keyboard = match found {
        Found::Keyboard(keyboard) => keyboard,
        Found::Unavailable { link, error } => {
            let status = Status {
                name: None,
                model: None,
                connection: link.clone(),
                state: "unavailable",
                changes: None,
                battery: None,
                unlock: None,
                problem: Some(words::unavailable(link, error)),
            };
            return (status, Some(Kind::NoKeyboard));
        }
    };
    let snapshot = &keyboard.snapshot;
    let state = &snapshot.state;
    let problem = words::problem(state);
    let status = Status {
        name: keyboard.reported_name(),
        model: snapshot.support_pack().map(|pack| pack.name.clone()),
        connection: keyboard.link.clone(),
        state: state_name(state),
        changes: keyboard.changes(),
        battery: None,
        unlock: matches!(state, State::Locked { .. })
            .then(|| words::unlock_instructions(snapshot.support_pack())),
        problem: problem.as_ref().map(|(_, message)| message.clone()),
    };
    (status, problem.map(|(kind, _)| kind))
}

/// The state, as `--json` writes it.
fn state_name(state: &State) -> &'static str {
    match state {
        State::Locked { .. } => "locked",
        State::Ready | State::UnsavedChanges | State::ChangesLost => "unlocked",
        State::WrongHalf => "other-half",
        State::NoLiveEditing => "no-live-editing",
        State::Incompatible(_) => "incompatible",
        State::NoDevice => "disconnected",
        State::Probing | State::Loading => "connecting",
    }
}

/// A keyboard's status as text: its name, then a line for each thing known about it.
fn text(status: &Status) -> String {
    let name = status
        .name
        .clone()
        .unwrap_or_else(|| match &status.connection {
            Link::Usb { port } => format!("Keyboard on {port}"),
            Link::Bluetooth {
                name: Some(name), ..
            } => name.clone(),
            Link::Bluetooth { name: None, .. } => "Keyboard on Bluetooth".to_owned(),
            Link::Simulated => "Simulated keyboard".to_owned(),
        });
    let mut lines = vec![(
        "Connection",
        match &status.connection {
            Link::Usb { port } => format!("USB, {port}"),
            Link::Bluetooth { .. } => "Bluetooth".to_owned(),
            Link::Simulated => "Simulated".to_owned(),
        },
    )];
    if let Some(model) = &status.model {
        lines.insert(0, ("Model", model.clone()));
    }
    if let Some(problem) = &status.problem {
        lines.push(("Problem", problem.clone()));
    } else {
        let lock = match &status.unlock {
            Some(unlock) => format!("Locked. {unlock}"),
            None => "Unlocked".to_owned(),
        };
        let changes = match status.changes {
            Some(Changes::Saved) => "Saved",
            Some(Changes::Trying) => "Trying changes it hasn't saved",
            None => "Unknown until it's unlocked",
        };
        let battery = match status.connection {
            Link::Usb { .. } => "Not reported over USB",
            Link::Bluetooth { .. } | Link::Simulated => "Not reported",
        };
        lines.extend([
            ("Lock", lock),
            ("Changes", changes.to_owned()),
            ("Battery", battery.to_owned()),
        ]);
    }
    let mut text = name;
    for (label, value) in lines {
        // Writing to a `String` can't fail.
        let _ = write!(text, "\n  {label:<10}  {value}");
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unlocked(changes: Changes) -> Status {
        Status {
            name: Some("Sofle Choc BT".to_owned()),
            model: Some("Keebart Sofle Choc Pro BT".to_owned()),
            connection: Link::Usb {
                port: "/dev/cu.usbmodem101".to_owned(),
            },
            state: "unlocked",
            changes: Some(changes),
            battery: None,
            unlock: None,
            problem: None,
        }
    }

    #[test]
    fn an_unlocked_keyboard_says_whether_its_changes_are_saved() {
        assert_eq!(
            text(&unlocked(Changes::Trying)),
            "Sofle Choc BT
  Model       Keebart Sofle Choc Pro BT
  Connection  USB, /dev/cu.usbmodem101
  Lock        Unlocked
  Changes     Trying changes it hasn't saved
  Battery     Not reported over USB"
        );
        assert!(text(&unlocked(Changes::Saved)).contains("  Changes     Saved\n"));
    }

    #[test]
    fn a_keyboard_on_bluetooth_says_so() {
        let status = Status {
            connection: Link::Bluetooth {
                name: Some("Sofle Choc Pro".to_owned()),
                id: "6F1C5D2A-0000-4000-8000-000000000000".to_owned(),
            },
            ..unlocked(Changes::Saved)
        };
        assert!(
            text(&status).contains("\n  Connection  Bluetooth\n"),
            "{}",
            text(&status)
        );
    }

    #[test]
    fn a_keyboard_on_bluetooth_that_cant_be_connected_is_named_by_macos_name() {
        let (status, kind) = status(&Found::Unavailable {
            link: Link::Bluetooth {
                name: Some("Sofle Choc Pro".to_owned()),
                id: "6F1C5D2A-0000-4000-8000-000000000000".to_owned(),
            },
            error: "the connection to the keyboard was closed".to_owned(),
        });
        assert_eq!(kind, Some(Kind::NoKeyboard));
        assert_eq!(
            text(&status),
            "Sofle Choc Pro
  Connection  Bluetooth
  Problem     Couldn't connect to Sofle Choc Pro over Bluetooth: the connection to the \
             keyboard was closed."
        );
    }

    #[test]
    fn a_port_that_cant_be_opened_is_named_by_its_path() {
        let (status, kind) = status(&Found::Unavailable {
            link: Link::Usb {
                port: "/dev/cu.usbmodem101".to_owned(),
            },
            error: "Resource busy".to_owned(),
        });
        assert_eq!(kind, Some(Kind::NoKeyboard));
        assert_eq!(
            text(&status),
            "Keyboard on /dev/cu.usbmodem101
  Connection  USB, /dev/cu.usbmodem101
  Problem     Couldn't open the keyboard on /dev/cu.usbmodem101: Resource busy. If \
             Keywright's app is open, quit it and try again."
        );
    }
}
