//! Wireless parity (KEY-60, product §11): every editing operation works the same over
//! Bluetooth as over USB. This test runs them all on a real ZMK keyboard, over the
//! connection `KEYWRIGHT_PARITY` names (`usb` or `bluetooth`), and leaves the keymap as it
//! found it: each change is undone or discarded, and the save is saved back.
//!
//! It backs the keymap up first, into a folder it prints, and discards changes a stopped run
//! left. It needs the keyboard on the named connection (for Bluetooth, the left half off the
//! Mac's cable), and someone to unlock it within five minutes. Run it on its own, once for
//! each connection (`docs/contributing/release-checklist.md`):
//!
//! ```sh
//! KEYWRIGHT_PARITY=usb cargo nextest run -p keywright-device --test parity --run-ignored only --no-capture
//! KEYWRIGHT_PARITY=bluetooth cargo nextest run -p keywright-device --test parity --run-ignored only --no-capture
//! ```

// Integration tests compile only for tests, so their helpers may unwrap, as clippy.toml
// lets tests do.
#![cfg(test)]

use std::time::Duration;

use keywright_core::behavior::BehaviorKind;
use keywright_core::edit::Edit;
use keywright_core::editor::Editor;
use keywright_core::keymap::{Binding, LayerId};
use keywright_device::selection::{self, ThisMac};
use keywright_device::{Backups, Peer, Session, SessionError, State};

/// How long the test waits for the keyboard, and for someone to unlock it.
const WAIT: Duration = Duration::from_secs(300);

/// The key the test changes: layer 1's fourth key.
const KEY: usize = 3;

/// What `KEYWRIGHT_PARITY` names.
fn wanted() -> Peer {
    match std::env::var("KEYWRIGHT_PARITY").as_deref() {
        Ok("usb") => Peer::Usb { product: None },
        Ok("bluetooth") => Peer::Bluetooth,
        _ => panic!("set KEYWRIGHT_PARITY to usb or bluetooth"),
    }
}

/// Whether the session's connection is the kind `wanted` names.
fn same_kind(peer: Option<&Peer>, wanted: &Peer) -> bool {
    matches!(
        (peer, wanted),
        (Some(Peer::Usb { .. }), Peer::Usb { .. }) | (Some(Peer::Bluetooth), Peer::Bluetooth)
    )
}

/// The editor now.
async fn editor(session: &Session) -> Editor {
    session.snapshot().await.unwrap().editor.unwrap()
}

/// Says a step passed.
fn passed(step: &str) {
    println!("✓ {step}");
}

/// Connects over the wanted connection, unlocked and ready, with nothing being tried.
async fn ready(session: &Session, wanted: &Peer) {
    tokio::time::timeout(WAIT, async {
        loop {
            let state = session
                .wait_for(|state| {
                    matches!(
                        state,
                        State::Locked { .. } | State::Ready | State::UnsavedChanges
                    )
                })
                .await
                .unwrap();
            let peer = session.snapshot().await.unwrap().peer;
            if !same_kind(peer.as_ref(), wanted) {
                tokio::time::sleep(Duration::from_millis(500)).await;
                continue;
            }
            match state {
                State::Locked { .. } => {
                    println!("press the keyboard's unlock keys");
                    session
                        .wait_for(|state| !matches!(state, State::Locked { .. }))
                        .await
                        .unwrap();
                }
                // Left by a run that stopped.
                State::UnsavedChanges => session.discard_unsaved_changes().await.unwrap(),
                _ => return,
            }
        }
    })
    .await
    .expect("the keyboard wasn't ready on that connection within five minutes");
}

/// The edit that makes layer 1's fourth key type `usage`.
async fn set_key(session: &Session, usage: u32) -> Edit {
    let snapshot = session.snapshot().await.unwrap();
    let key_press = (snapshot.keyboard.unwrap().behaviors)
        .find(BehaviorKind::KeyPress)
        .unwrap();
    let before = snapshot.editor.unwrap().current().layers[1].bindings[KEY];
    Edit::SetBinding {
        layer: LayerId(1),
        position: KEY,
        before,
        after: Binding {
            behavior: key_press,
            param1: usage,
            param2: 0,
        },
    }
}

/// A (0x04) and B (0x05), as HID usages.
const KEY_A: u32 = 0x0007_0004;
const KEY_B: u32 = 0x0007_0005;

#[tokio::test]
#[ignore = "needs a ZMK keyboard on the connection KEYWRIGHT_PARITY names: run it on its own"]
async fn every_editing_operation_works_over_this_connection() {
    let wanted = wanted();
    let folder = std::env::temp_dir().join("keywright-parity-backups");
    println!("backing the keymap up into {}", folder.display());
    let session = Session::with_backups(Backups::new(folder, Backups::DEFAULT_KEEP));
    tokio::spawn(selection::select(session.clone(), ThisMac::new(|| true)));
    ready(&session, &wanted).await;
    passed(&format!("connected over {wanted:?}, unlocked, and read"));

    let steps = tokio::spawn(run(session.clone()));
    let result = steps.await;
    // Whatever happened, nothing stays being tried.
    if let Err(error) = session.discard().await {
        println!("couldn't discard: {error}");
    }
    if let Err(failed) = result {
        std::panic::resume_unwind(failed.into_panic());
    }
}

/// Each operation, then back to how it was.
async fn run(session: Session) {
    let start = editor(&session).await;
    let keymap = start.current().clone();
    assert!(
        keymap.layers.len() >= 2,
        "the keymap has fewer than 2 layers"
    );

    // A key, undo and redo.
    let change = set_key(&session, KEY_A).await;
    session.apply(vec![change]).await.unwrap();
    assert_eq!(
        editor(&session).await.current().layers[1].bindings[KEY].param1,
        KEY_A
    );
    session.undo().await.unwrap();
    assert_eq!(editor(&session).await.current(), &keymap);
    session.redo().await.unwrap();
    assert_eq!(editor(&session).await.changes().len(), 1);
    session.undo().await.unwrap();
    passed("a key changed, undone and redone");

    // Renaming a layer, to a name of the most bytes ZMK v0.3 takes.
    let name = keymap.layers[1].name.clone();
    let rename = Edit::RenameLayer {
        layer: LayerId(1),
        before: name.clone(),
        after: "parity-check-name1".to_owned(),
    };
    session.apply(vec![rename]).await.unwrap();
    assert_eq!(
        editor(&session).await.current().layers[1].name,
        "parity-check-name1"
    );
    session.undo().await.unwrap();
    assert_eq!(editor(&session).await.current().layers[1].name, name);
    passed("a layer renamed, and back");

    // Moving a layer, and back.
    session
        .apply(vec![Edit::MoveLayer { from: 1, to: 0 }])
        .await
        .unwrap();
    assert_eq!(
        editor(&session).await.current().layers[0].id,
        keymap.layers[1].id
    );
    session.undo().await.unwrap();
    assert_eq!(editor(&session).await.current(), &keymap);
    passed("a layer moved, and back");

    // Removing the last layer, and restoring it.
    let last = keymap.layers.len() - 1;
    let remove = Edit::RemoveLayer {
        index: last,
        layer: keymap.layers[last].clone(),
    };
    session.apply(vec![remove]).await.unwrap();
    assert_eq!(editor(&session).await.current().layers.len(), last);
    session.undo().await.unwrap();
    assert_eq!(editor(&session).await.current(), &keymap);
    passed("a layer removed, and restored");

    // Adding a layer, if there's a free slot.
    match session.add_layer().await {
        Ok(()) => {
            assert_eq!(editor(&session).await.current().layers.len(), last + 2);
            session.undo().await.unwrap();
            passed("a layer added, and taken away");
        }
        Err(SessionError::Keyboard(error)) => {
            println!("- no layer added: the keyboard has no free slot ({error})");
        }
        Err(error) => panic!("adding a layer failed: {error}"),
    }

    // Reverting one change of two.
    let first = set_key(&session, KEY_A).await;
    session.apply(vec![first]).await.unwrap();
    let rename = Edit::RenameLayer {
        layer: LayerId(1),
        before: name.clone(),
        after: "parity".to_owned(),
    };
    session.apply(vec![rename]).await.unwrap();
    let changes = editor(&session).await.changes();
    assert_eq!(changes.len(), 2);
    session.revert(changes[0].clone()).await.unwrap();
    assert_eq!(editor(&session).await.changes().len(), 1);
    passed("one change of two reverted");

    // Discarding.
    session.discard().await.unwrap();
    assert_eq!(editor(&session).await.current(), &keymap);
    assert_eq!(editor(&session).await.changes().len(), 0);
    passed("the changes discarded");

    // Saving, then saving it back.
    let change = set_key(&session, KEY_B).await;
    session.apply(vec![change]).await.unwrap();
    session.save().await.unwrap();
    let saved = session.snapshot().await.unwrap();
    assert_eq!(saved.editor.unwrap().changes().len(), 0);
    session.undo().await.unwrap();
    session.save().await.unwrap();
    assert_eq!(editor(&session).await.current(), &keymap);
    passed("a change saved, and the keymap saved back");
}
