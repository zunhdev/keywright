//! Low-battery notifications (BAR-3, design §9.5): "Sofle Choc BT: left half at 14%."
//!
//! [`LowBattery`] says when to notify: when a half's *reported* level crosses below the
//! threshold, once. It notifies again only after the level has come back up past the
//! threshold by [`HYSTERESIS`], so a level wavering around the threshold doesn't flap. A
//! level that isn't reported, as while the keyboard is away, changes nothing: coming back
//! still low isn't a new crossing. Raising the threshold above a level is a crossing.

use crate::ipc::BatteryStatus;

/// How far above the threshold a level has to come back before it can cross again.
pub const HYSTERESIS: u8 = 5;

/// A half of the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    /// The left half.
    Left,
    /// The right half.
    Right,
}

/// Which halves are low, as last notified.
#[derive(Debug, Default)]
pub struct LowBattery {
    left: bool,
    right: bool,
}

impl LowBattery {
    /// The halves to notify about, with their levels, now that the battery reads `battery`
    /// and the threshold is `threshold` percent.
    pub fn update(&mut self, battery: BatteryStatus, threshold: u8) -> Vec<(Half, u8)> {
        let mut notify = Vec::new();
        for (half, level, low) in [
            (Half::Left, battery.left, &mut self.left),
            (Half::Right, battery.right, &mut self.right),
        ] {
            let Some(percent) = level else {
                continue;
            };
            if !*low && percent < threshold {
                *low = true;
                notify.push((half, percent));
            } else if *low && percent >= threshold.saturating_add(HYSTERESIS) {
                *low = false;
            }
        }
        notify
    }
}

/// What the notification says, such as "Sofle Choc BT: left half at 14%."
pub fn message(keyboard: Option<&str>, half: Half, percent: u8) -> String {
    let half = match half {
        Half::Left => "left",
        Half::Right => "right",
    };
    format!(
        "{}: {half} half at {percent}%.",
        keyboard.unwrap_or("Your keyboard")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn left(percent: u8) -> BatteryStatus {
        BatteryStatus {
            left: Some(percent),
            right: None,
        }
    }

    #[test]
    fn crossing_below_the_threshold_notifies_once() {
        let mut low = LowBattery::default();
        assert_eq!(low.update(left(16), 15), []);
        assert_eq!(low.update(left(15), 15), []);
        assert_eq!(low.update(left(14), 15), [(Half::Left, 14)]);
        // Still low: no more notifications.
        assert_eq!(low.update(left(13), 15), []);
        assert_eq!(low.update(left(5), 15), []);
    }

    #[test]
    fn a_level_wavering_around_the_threshold_doesnt_flap() {
        let mut low = LowBattery::default();
        assert_eq!(low.update(left(14), 15), [(Half::Left, 14)]);
        // Back up a little, and down again: the same crossing.
        assert_eq!(low.update(left(16), 15), []);
        assert_eq!(low.update(left(14), 15), []);
        // Charged past the threshold and the hysteresis: the next drop notifies again.
        assert_eq!(low.update(left(20), 15), []);
        assert_eq!(low.update(left(14), 15), [(Half::Left, 14)]);
    }

    #[test]
    fn a_level_not_reported_isnt_a_crossing() {
        let mut low = LowBattery::default();
        assert_eq!(low.update(left(14), 15), [(Half::Left, 14)]);
        // The keyboard goes away, and comes back still low.
        assert_eq!(low.update(BatteryStatus::default(), 15), []);
        assert_eq!(low.update(left(13), 15), []);
        // Nothing reported never notifies.
        assert_eq!(
            LowBattery::default().update(BatteryStatus::default(), 15),
            []
        );
    }

    #[test]
    fn raising_the_threshold_above_a_level_is_a_crossing() {
        let mut low = LowBattery::default();
        assert_eq!(low.update(left(18), 15), []);
        assert_eq!(low.update(left(18), 20), [(Half::Left, 18)]);
        // Lowering it again, but not past the hysteresis, keeps it low.
        assert_eq!(low.update(left(18), 15), []);
        assert_eq!(low.update(left(18), 20), []);
        // Lowered well below the level: it's fine again, and can cross again.
        assert_eq!(low.update(left(18), 10), []);
        assert_eq!(low.update(left(18), 20), [(Half::Left, 18)]);
    }

    #[test]
    fn each_half_crosses_on_its_own() {
        let mut low = LowBattery::default();
        let both = |left, right| BatteryStatus {
            left: Some(left),
            right: Some(right),
        };
        assert_eq!(low.update(both(14, 50), 15), [(Half::Left, 14)]);
        assert_eq!(low.update(both(14, 12), 15), [(Half::Right, 12)]);
    }

    #[test]
    fn the_notification_names_the_keyboard_and_the_half() {
        assert_eq!(
            message(Some("Sofle Choc BT"), Half::Left, 14),
            "Sofle Choc BT: left half at 14%."
        );
        assert_eq!(
            message(None, Half::Right, 9),
            "Your keyboard: right half at 9%."
        );
    }
}
