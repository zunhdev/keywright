//! The Mac compatibility check (MAC-7, design §5.1): the keys that type something that
//! doesn't work on a Mac, such as the keyboard page's own Copy key, each with why and, where
//! there is one, its Mac replacement.
//!
//! The check runs whenever the keymap is read or changes: each layer of the keymap payload
//! lists its [`MacFinding`]s, and its keys show ⚠. [`fix_mac_keys`] puts the replacements
//! in, as one change to try, and [`dismiss_mac_findings`] dismisses findings for good, on
//! this keyboard ([`crate::mac_check`]).

use std::sync::Arc;

use keywright_core::action::Action;
use keywright_core::behavior::BehaviorTable;
use keywright_core::edit::Edit;
use keywright_core::hid::UsagePage;
use keywright_core::keycap;
use keywright_core::keymap::{DeviceIdentity, Keymap, LayerId};
use keywright_core::mac::compat;
use keywright_core::support_packs::SupportPack;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::action::Page;
use super::error::AppError;
use super::guard::KeyRef;
use super::{AppState, Context};
use crate::mac_check::Dismissed;

/// A key that types something that doesn't work on a Mac.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct MacFinding {
    /// The finding, to dismiss it: the key, and what it types.
    pub finding: FindingRef,
    /// Where the key is, in words, such as `Right half, row 2, key 3`.
    pub place: String,
    /// What it types, such as `Copy (Linux style)`.
    pub name: String,
    /// What goes wrong, in one sentence, such as "It doesn't do anything on a Mac."
    pub reason: String,
    /// What the key shows once fixed, such as `⌘C`, if there's a Mac replacement the
    /// keyboard can take.
    pub fix: Option<String>,
}

/// A finding of the Mac check: a key on a layer, and the key it types that doesn't work on
/// a Mac.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct FindingRef {
    /// The layer, by ID.
    pub layer: u32,
    /// The key, by key position.
    pub position: u32,
    /// The usage page of the key it types.
    pub page: Page,
    /// That key's usage ID.
    pub id: u16,
}

impl From<FindingRef> for Dismissed {
    fn from(finding: FindingRef) -> Self {
        Self {
            layer: finding.layer,
            position: finding.position,
            page: UsagePage::from(finding.page).number(),
            id: finding.id,
        }
    }
}

/// The findings the person hasn't dismissed on this keyboard, layer by layer in the
/// keymap's order, each with where its key is.
pub fn findings(
    keymap: &Keymap,
    behaviors: &BehaviorTable,
    keyboard: &DeviceIdentity,
    pack: Option<&SupportPack>,
    context: &Context,
) -> Vec<Vec<MacFinding>> {
    let found = compat::check(keymap, behaviors, compat::RULES);
    keymap
        .layers
        .iter()
        .map(|layer| {
            found
                .iter()
                .filter(|finding| finding.layer == layer.id)
                .filter(|finding| {
                    let dismissed =
                        Dismissed::new(finding.layer, finding.position, finding.rule.usage);
                    !context.dismissed.contains(keyboard, dismissed)
                })
                .map(|finding| {
                    let action = layer
                        .bindings
                        .get(finding.position)
                        .map(|binding| Action::from_binding(*binding, behaviors));
                    let fix = action
                        .and_then(|action| compat::fix(action, compat::RULES))
                        .and_then(|fixed| fixed.to_binding(behaviors).ok())
                        .map(|fixed| keycap::label(keymap, behaviors, fixed, &context.labels));
                    MacFinding {
                        finding: FindingRef {
                            layer: layer.id.0,
                            position: number(finding.position),
                            page: finding.rule.usage.page.into(),
                            id: finding.rule.usage.id,
                        },
                        place: pack
                            .and_then(|pack| pack.key_place(finding.position))
                            .unwrap_or_else(|| {
                                format!("Key {}", finding.position.saturating_add(1))
                            }),
                        name: finding.rule.name.to_owned(),
                        reason: finding.rule.reason.to_owned(),
                        fix,
                    }
                })
                .collect()
        })
        .collect()
}

/// A key position, which fits: a keyboard has at most a few hundred keys.
fn number(position: usize) -> u32 {
    u32::try_from(position).unwrap_or(u32::MAX)
}

impl AppState {
    /// Puts the Mac replacements into `keys`, as one change. Keys with nothing to fix are
    /// left as they are.
    pub async fn fix_mac_keys(&self, keys: Vec<KeyRef>, force: bool) -> Result<(), AppError> {
        self.change(force, |keymap, behaviors| {
            let mut edits = Vec::new();
            for key in keys {
                let layer = LayerId(key.layer);
                let position = usize::try_from(key.position).unwrap_or(usize::MAX);
                let binding = *keymap
                    .layer(layer)
                    .and_then(|layer| layer.bindings.get(position))
                    .ok_or(AppError::EditRefused)?;
                let action = Action::from_binding(binding, behaviors);
                let Some(fixed) = compat::fix(action, compat::RULES) else {
                    continue;
                };
                let fixed = fixed
                    .to_binding(behaviors)
                    .map_err(|_| AppError::EditRefused)?;
                edits.extend(Edit::set_keys(keymap, layer, &[position], fixed)?);
            }
            Ok(edits)
        })
        .await
    }

    /// Dismisses `findings` on the keyboard connected, and remembers that.
    pub async fn dismiss_mac_findings(&self, findings: Vec<FindingRef>) -> Result<(), AppError> {
        let snapshot = self.session.snapshot().await?;
        let keyboard = snapshot.keyboard.ok_or(AppError::NotReady)?.identity;
        self.context.send_modify(|context| {
            Arc::make_mut(context)
                .dismissed
                .dismiss(&keyboard, findings.into_iter().map(Dismissed::from));
        });
        // They stay dismissed until Keywright quits, even if they can't be saved.
        let Some(file) = &self.dismissals else {
            return Ok(());
        };
        self.context()
            .dismissed
            .write(file)
            .map_err(|_| AppError::DismissalNotSaved)
    }
}

/// Puts the Mac replacements into keys, as one change to try (MAC-7): what _Fix_ and _Fix
/// All_ do. Like every command that changes keys, it refuses a change that would strand
/// the user unless `force` (SAF-2).
///
/// # Errors
///
/// Fails if the keymap can't be changed now, a key isn't there, the change would strand
/// the user, or the keyboard doesn't carry out the change, which is then undone.
#[tauri::command]
#[specta::specta]
pub async fn fix_mac_keys(
    app: tauri::State<'_, AppState>,
    keys: Vec<KeyRef>,
    force: bool,
) -> Result<(), AppError> {
    app.fix_mac_keys(keys, force).await
}

/// Dismisses findings of the Mac check on this keyboard, for good (MAC-7).
///
/// # Errors
///
/// Fails if no keyboard has been read, or the session has stopped; or if the dismissal
/// can't be saved, in which case it lasts until Keywright quits.
#[tauri::command]
#[specta::specta]
pub async fn dismiss_mac_findings(
    app: tauri::State<'_, AppState>,
    findings: Vec<FindingRef>,
) -> Result<(), AppError> {
    app.dismiss_mac_findings(findings).await
}

#[cfg(test)]
mod tests {
    use keywright_device::{Connection, Peer, Session, State};
    use keywright_sim::Simulator;

    use super::{FindingRef, MacFinding};
    use crate::ipc::AppState;
    use crate::ipc::action::Page;
    use crate::ipc::guard::KeyRef;
    use crate::ipc::keymap::Keymap;

    /// The raise layer, by ID.
    const RAISE: u32 = 2;

    /// The simulated Sofle, unlocked and ready, with its stock keymap.
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

    /// The keymap the canvas draws now.
    async fn keymap(app: &AppState) -> Keymap {
        Keymap::of(&app.session.snapshot().await.unwrap(), &app.context()).unwrap()
    }

    /// The raise layer's findings, by what they're about and their fix.
    fn raise(keymap: &Keymap) -> Vec<(u32, String, Option<String>)> {
        keymap.layers[2]
            .mac_findings
            .iter()
            .map(|found| {
                (
                    found.finding.position,
                    found.name.clone(),
                    found.fix.clone(),
                )
            })
            .collect()
    }

    fn found(position: u32, name: &str, fix: Option<&str>) -> (u32, String, Option<String>) {
        (position, name.to_owned(), fix.map(str::to_owned))
    }

    /// The Done-when: the stock raise layer's Undo, Cut, Copy and Paste are flagged, as
    /// are its Windows keys, which have no fix.
    #[tokio::test]
    async fn the_stock_raise_layers_editing_keys_are_flagged_with_their_fixes() {
        let app = ready_sofle().await;
        let keymap = keymap(&app).await;
        assert_eq!(
            raise(&keymap),
            [
                found(13, "Insert (Windows style)", None),
                found(14, "Print Screen (Windows style)", Some("⇧⌘3")),
                found(15, "Menu key (Windows style)", None),
                found(37, "Undo (Linux style)", Some("⌘Z")),
                found(38, "Cut (Linux style)", Some("⌘X")),
                found(39, "Copy (Linux style)", Some("⌘C")),
                found(40, "Paste (Linux style)", Some("⌘V")),
            ]
        );
        let copy = &keymap.layers[2].mac_findings[5];
        assert_eq!(
            *copy,
            MacFinding {
                finding: FindingRef {
                    layer: RAISE,
                    position: 39,
                    page: Page::Keyboard,
                    id: 0x7C,
                },
                place: "Left half, row 4, key 4".to_owned(),
                name: "Copy (Linux style)".to_owned(),
                reason: "It doesn't do anything on a Mac.".to_owned(),
                fix: Some("⌘C".to_owned()),
            }
        );
        assert!(keymap.layers[2].keys[39].mac_warning);
        // The other layers have none.
        assert!(
            keymap
                .layers
                .iter()
                .filter(|layer| layer.id != RAISE)
                .all(|layer| layer.mac_findings.is_empty())
        );
    }

    /// Fix All puts the replacements in as one change, which Review lists and one undo
    /// takes back; keys with no fix stay.
    #[tokio::test]
    async fn fixing_puts_the_mac_replacements_in_as_one_change() {
        let app = ready_sofle().await;
        let keys = [13, 14, 15, 37, 38, 39, 40].map(|position| KeyRef {
            layer: RAISE,
            position,
        });
        app.fix_mac_keys(keys.to_vec(), false).await.unwrap();

        let fixed = keymap(&app).await;
        assert_eq!(
            raise(&fixed),
            [
                found(13, "Insert (Windows style)", None),
                found(15, "Menu key (Windows style)", None),
            ]
        );
        assert!(!fixed.layers[2].keys[39].mac_warning);
        assert!(fixed.layers[2].keys[39].changed);
        let rows: Vec<_> = app
            .review()
            .await
            .unwrap()
            .into_iter()
            .map(|row| (row.before, row.after))
            .collect();
        let row = |before: &str, after: &str| (before.to_owned(), after.to_owned());
        assert_eq!(
            rows,
            [
                row("Print Screen", "⇧⌘3"),
                row("Undo", "⌘Z"),
                row("Cut", "⌘X"),
                row("Copy", "⌘C"),
                row("Paste", "⌘V"),
            ]
        );

        app.session.undo().await.unwrap();
        assert_eq!(raise(&keymap(&app).await).len(), 7);
    }

    /// A dismissed finding isn't listed, and its key shows no ⚠; the others stay.
    #[tokio::test]
    async fn a_dismissed_finding_goes_away() {
        let app = ready_sofle().await;
        let copy = keymap(&app).await.layers[2].mac_findings[5].finding;
        app.dismiss_mac_findings(vec![copy]).await.unwrap();

        let keymap = keymap(&app).await;
        assert!(!keymap.layers[2].keys[39].mac_warning);
        assert!(keymap.layers[2].keys[40].mac_warning);
        assert_eq!(
            raise(&keymap)
                .into_iter()
                .map(|(position, ..)| position)
                .collect::<Vec<_>>(),
            [13, 14, 15, 37, 38, 40]
        );
    }
}
