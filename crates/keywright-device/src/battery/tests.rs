//! Which level is which half, noticing sleep, and the watch against a fake Bluetooth.

use std::sync::{Arc, Mutex};

use super::*;

fn level(description: Option<&str>, percent: Option<u8>) -> BatteryLevel {
    BatteryLevel {
        description: description.map(str::to_owned),
        percent,
    }
}

#[test]
fn stock_firmware_reports_the_half_that_talks_to_the_computer_only() {
    assert_eq!(
        Battery::of(&[level(None, Some(82))]),
        Battery {
            central: Some(82),
            peripheral: None,
        }
    );
}

#[test]
fn zmks_battery_proxy_adds_the_other_half() {
    let levels = [level(None, Some(82)), level(Some("Peripheral 0"), Some(64))];
    assert_eq!(
        Battery::of(&levels),
        Battery {
            central: Some(82),
            peripheral: Some(64),
        }
    );
}

#[test]
fn a_level_not_read_is_not_reported() {
    assert_eq!(Battery::of(&[]), Battery::default());
    assert_eq!(
        Battery::of(&[level(None, None), level(Some("Peripheral 0"), None)]),
        Battery::default()
    );
}

#[test]
fn the_mac_slept_when_the_system_clock_ran_on_without_the_monotonic_one() {
    let wall = SystemTime::now();
    let monotonic = Instant::now();
    let mut sleeps = Sleeps::new(wall, monotonic);
    // A second, on both clocks.
    let (wall, monotonic) = (wall + LOOK_EVERY, monotonic + LOOK_EVERY);
    assert!(!sleeps.woke(wall, monotonic));
    // Ten minutes on the system clock, a second on the monotonic one.
    let (wall, monotonic) = (wall + Duration::from_secs(600), monotonic + LOOK_EVERY);
    assert!(sleeps.woke(wall, monotonic));
    // The system clock set back an hour isn't sleep.
    let wall = wall.checked_sub(Duration::from_secs(3600)).unwrap();
    assert!(!sleeps.woke(wall, monotonic + LOOK_EVERY));
}

/// Bluetooth with keyboards whose levels a test sets, and counts the reads asked for.
#[derive(Clone, Default)]
struct Fake {
    keyboards: Arc<Mutex<Vec<BlePeripheral>>>,
    /// The levels the next reader starts from, and the sender that changes them.
    levels: Arc<Mutex<Option<watch::Sender<Vec<BatteryLevel>>>>>,
    reads: Arc<Mutex<usize>>,
}

impl Fake {
    fn connect(&self, percent: u8) {
        *self.keyboards.lock().unwrap() = vec![BlePeripheral {
            id: "sofle".to_owned(),
            name: Some("Sofle Choc Pro".to_owned()),
        }];
        let (levels, _) = watch::channel(vec![level(None, Some(percent))]);
        *self.levels.lock().unwrap() = Some(levels);
    }

    fn notify(&self, percent: u8) {
        if let Some(levels) = &*self.levels.lock().unwrap() {
            levels.send_replace(vec![level(None, Some(percent))]);
        }
    }

    /// The keyboard goes away: its reader closes.
    fn disconnect(&self) {
        self.keyboards.lock().unwrap().clear();
        self.levels.lock().unwrap().take();
    }
}

impl BatterySource for Fake {
    fn keyboards(&mut self) -> impl Future<Output = Vec<BlePeripheral>> + Send {
        std::future::ready(self.keyboards.lock().unwrap().clone())
    }

    fn open(
        &mut self,
        _: &BlePeripheral,
    ) -> impl Future<Output = Result<BatteryReader, TransportError>> + Send {
        let levels = self
            .levels
            .lock()
            .unwrap()
            .as_ref()
            .map(watch::Sender::subscribe);
        let reads = Arc::clone(&self.reads);
        std::future::ready(levels.ok_or(TransportError::Closed).map(|levels| {
            let (to_reader, mut requests) = mpsc::unbounded_channel();
            tokio::spawn(async move {
                while requests.recv().await.is_some() {
                    *reads.lock().unwrap() += 1;
                }
            });
            BatteryReader::over(levels, to_reader)
        }))
    }
}

/// Waits until the levels are `wanted`.
async fn until(watch: &BatteryWatch, wanted: Battery) {
    let mut levels = watch.levels();
    tokio::time::timeout(
        Duration::from_secs(60),
        levels.wait_for(|now| *now == wanted),
    )
    .await
    .expect("the levels never got there")
    .unwrap();
}

fn central(percent: u8) -> Battery {
    Battery {
        central: Some(percent),
        peripheral: None,
    }
}

#[tokio::test(start_paused = true)]
async fn the_level_follows_the_keyboard_coming_notifying_and_going() {
    let fake = Fake::default();
    let watch = Battery::watch(fake.clone());
    assert_eq!(*watch.levels().borrow(), Battery::default());

    fake.connect(82);
    until(&watch, central(82)).await;
    fake.notify(81);
    until(&watch, central(81)).await;

    // Gone: not reported, until it's back.
    fake.disconnect();
    until(&watch, Battery::default()).await;
    fake.connect(80);
    until(&watch, central(80)).await;
}

#[tokio::test(start_paused = true)]
async fn a_read_asked_for_reaches_the_reader() {
    let fake = Fake::default();
    fake.connect(82);
    let watch = Battery::watch(fake.clone());
    until(&watch, central(82)).await;
    watch.read();
    tokio::time::sleep(LOOK_EVERY).await;
    assert_eq!(*fake.reads.lock().unwrap(), 1);
}

#[tokio::test(start_paused = true)]
async fn with_two_keyboards_on_bluetooth_neither_is_reported() {
    let fake = Fake::default();
    fake.connect(82);
    fake.keyboards.lock().unwrap().push(BlePeripheral {
        id: "other".to_owned(),
        name: Some("Corne".to_owned()),
    });
    let watch = Battery::watch(fake.clone());
    tokio::time::sleep(LOOK_EVERY * 5).await;
    assert_eq!(*watch.levels().borrow(), Battery::default());
}

#[test]
fn the_half_that_talks_to_the_computer_is_the_packs_usb_half_or_the_left() {
    let battery = Battery {
        central: Some(82),
        peripheral: Some(64),
    };
    let sofle = keywright_core::support_packs::embedded()
        .iter()
        .find(|pack| pack.id == "keebart-sofle-choc-pro-bt")
        .unwrap();
    assert_eq!(battery.halves(Some(sofle)), (Some(82), Some(64)));
    assert_eq!(battery.halves(None), (Some(82), Some(64)));
}
