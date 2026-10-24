//! A simulated keyboard running in a task, reached over an [`InMemoryTransport`], with
//! faults to inject.
//!
//! The [`Simulator`] adds to the [`Keyboard`] what needs a clock or a connection:
//!
//! - **The idle lock.** Like ZMK, the keyboard locks [`IDLE_LOCK_TIMEOUT`] after the last
//!   request it handled, or the unlock key press, and says so.
//! - **The byte stream.** Requests are read as ZMK v0.3 reads them, with its 30-byte
//!   receive buffer and the bugs spike S1 found (module `receiver`), and requests that
//!   don't decode get no answer. What the keyboard sends arrives in chunks of up to
//!   [`CHUNK_LEN`] bytes, like USB packets, so frames span chunks.
//! - **Faults**, for testing how Keywright copes:
//!   - latency before everything the keyboard sends ([`Simulator::set_latency`]);
//!   - responses that never arrive ([`Simulator::drop_responses`]);
//!   - notifications that never arrive, as spike S5 saw an unlock notification lost
//!     ([`Simulator::drop_notifications`]);
//!   - requests that arrive without their end ([`Simulator::drop_received`]);
//!   - bytes that aren't messages ([`Simulator::inject_garbage`]);
//!   - the keyboard's output moving to another transport, which locks it and leaves this
//!     connection unanswered ([`Simulator::switch_transport`]);
//!   - a restart, which drops the connection and forgets unsaved changes
//!     ([`Simulator::restart`]);
//!   - the wrong half, which never answers ([`Simulator::set_wrong_half`]).
//!
//! The handle's methods take effect in the order they're called, before any request the
//! connection sends afterwards. The task runs while the handle or a connection exists.

mod receiver;

use std::future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use keywright_studio::frame;
use keywright_studio::message::DeviceMessage;
use keywright_studio::transport::{Transport, TransportError, TransportKind};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{Instant, sleep, sleep_until};

use self::receiver::Receiver;
use crate::firmware::{Firmware, FirmwareError};
use crate::keyboard::{IDLE_LOCK_TIMEOUT, Keyboard};

/// The most bytes the keyboard sends in one chunk: a USB full-speed packet.
pub const CHUNK_LEN: usize = 64;

/// A handle to a simulated keyboard. Clones control the same keyboard.
#[derive(Debug, Clone)]
pub struct Simulator {
    events: mpsc::UnboundedSender<Event>,
}

/// The client's end of a connection to a simulated keyboard.
#[derive(Debug)]
pub struct InMemoryTransport {
    connection: u64,
    to_keyboard: mpsc::UnboundedSender<Event>,
    from_keyboard: mpsc::UnboundedReceiver<Vec<u8>>,
}

/// Something for the keyboard's task to do.
#[derive(Debug)]
enum Event {
    Connect {
        connection: u64,
        to_client: mpsc::UnboundedSender<Vec<u8>>,
    },
    Bytes {
        connection: u64,
        bytes: Vec<u8>,
    },
    PressUnlockKey,
    Lock,
    Restart,
    SetLatency(Duration),
    DropResponses(usize),
    DropNotifications(usize),
    DropReceived(usize),
    InjectGarbage(Vec<u8>),
    SwitchTransport {
        away: bool,
    },
    SetWrongHalf(bool),
    Inspect(oneshot::Sender<Keyboard>),
}

/// Numbers connections, so bytes from one that was replaced are ignored.
static NEXT_CONNECTION: AtomicU64 = AtomicU64::new(0);

impl Simulator {
    /// Starts a keyboard in a task on the current Tokio runtime.
    ///
    /// # Panics
    ///
    /// Panics if called outside a Tokio runtime.
    #[must_use]
    pub fn start(keyboard: Keyboard) -> Self {
        let (events, receiver) = mpsc::unbounded_channel();
        tokio::spawn(Device::new(keyboard).run(receiver));
        Self { events }
    }

    /// Starts Keebart's Sofle Choc Pro BT with its stock firmware, as from the factory.
    ///
    /// # Errors
    ///
    /// Fails if its support pack doesn't make a keyboard, which the tests rule out.
    ///
    /// # Panics
    ///
    /// Panics if called outside a Tokio runtime.
    pub fn sofle() -> Result<Self, FirmwareError> {
        Ok(Self::start(Keyboard::new(Firmware::sofle()?)))
    }

    /// Connects to the keyboard, as plugging it in does. A previous connection closes.
    #[must_use]
    pub fn connect(&self) -> InMemoryTransport {
        let connection = NEXT_CONNECTION.fetch_add(1, Ordering::Relaxed);
        let (to_client, from_keyboard) = mpsc::unbounded_channel();
        self.send(Event::Connect {
            connection,
            to_client,
        });
        InMemoryTransport {
            connection,
            to_keyboard: self.events.clone(),
            from_keyboard,
        }
    }

    /// Someone presses the unlock key.
    pub fn press_unlock_key(&self) {
        self.send(Event::PressUnlockKey);
    }

    /// The keyboard locks itself, as it does after being idle, and says so.
    pub fn lock(&self) {
        self.send(Event::Lock);
    }

    /// The keyboard restarts, as when its battery runs out: it forgets unsaved changes,
    /// locks, and the connection closes. [`connect`](Self::connect) reconnects.
    pub fn restart(&self) {
        self.send(Event::Restart);
    }

    /// Delays everything the keyboard sends from now on. Zero turns the delay off.
    pub fn set_latency(&self, latency: Duration) {
        self.send(Event::SetLatency(latency));
    }

    /// Loses the next `count` responses. The keyboard handles the requests, and sends its
    /// notifications.
    pub fn drop_responses(&self, count: usize) {
        self.send(Event::DropResponses(count));
    }

    /// Loses the next `count` notifications the keyboard sends, such as the one saying it
    /// was unlocked. The keyboard changes all the same.
    pub fn drop_notifications(&self, count: usize) {
        self.send(Event::DropNotifications(count));
    }

    /// Loses the last `count` bytes of the next write the keyboard receives, as when its
    /// receive buffer overflows. A frame that loses its end takes the next frame with it,
    /// unless a lone `EOF` comes first.
    pub fn drop_received(&self, count: usize) {
        self.send(Event::DropReceived(count));
    }

    /// Sends bytes that aren't a message before the next thing the keyboard sends.
    pub fn inject_garbage(&self, bytes: Vec<u8>) {
        self.send(Event::InjectGarbage(bytes));
    }

    /// The keyboard's output moves to another transport, as when the cable is unplugged and
    /// it carries on over Bluetooth. It locks, says so on the other transport, and stops
    /// reading this connection, until [`switch_transport_back`](Self::switch_transport_back).
    pub fn switch_transport(&self) {
        self.send(Event::SwitchTransport { away: true });
    }

    /// The keyboard's output moves back to this connection. It stays locked.
    pub fn switch_transport_back(&self) {
        self.send(Event::SwitchTransport { away: false });
    }

    /// Whether the connection reaches the wrong half: the one not connected to the computer,
    /// which doesn't serve Studio and never answers.
    pub fn set_wrong_half(&self, wrong_half: bool) {
        self.send(Event::SetWrongHalf(wrong_half));
    }

    /// The keyboard as it is now, once the events before have taken effect.
    pub async fn keyboard(&self) -> Option<Keyboard> {
        let (reply, keyboard) = oneshot::channel();
        self.send(Event::Inspect(reply));
        keyboard.await.ok()
    }

    fn send(&self, event: Event) {
        // The task runs while this handle exists, so this only fails if it panicked.
        let _ = self.events.send(event);
    }
}

impl Transport for InMemoryTransport {
    fn send(&mut self, bytes: &[u8]) -> impl Future<Output = Result<(), TransportError>> + Send {
        let sent = self
            .to_keyboard
            .send(Event::Bytes {
                connection: self.connection,
                bytes: bytes.to_vec(),
            })
            .map_err(|_| TransportError::Closed);
        future::ready(sent)
    }

    async fn recv(&mut self) -> Result<Vec<u8>, TransportError> {
        self.from_keyboard
            .recv()
            .await
            .ok_or(TransportError::Closed)
    }

    fn kind(&self) -> TransportKind {
        TransportKind::InMemory
    }
}

/// The keyboard's task: the keyboard, its connection and the faults.
struct Device {
    keyboard: Keyboard,
    /// What the keyboard has received. It belongs to the keyboard, not to a connection,
    /// so it lasts until the keyboard restarts.
    receiver: Receiver,
    connection: Option<Connection>,
    /// When the keyboard locks itself, if nothing happens before.
    lock_at: Option<Instant>,
    latency: Duration,
    responses_to_drop: usize,
    notifications_to_drop: usize,
    received_to_drop: usize,
    garbage: Vec<u8>,
    transport_away: bool,
    wrong_half: bool,
}

struct Connection {
    id: u64,
    to_client: mpsc::UnboundedSender<Vec<u8>>,
}

impl Device {
    fn new(keyboard: Keyboard) -> Self {
        Self {
            keyboard,
            receiver: Receiver::new(),
            connection: None,
            lock_at: None,
            latency: Duration::ZERO,
            responses_to_drop: 0,
            notifications_to_drop: 0,
            received_to_drop: 0,
            garbage: Vec::new(),
            transport_away: false,
            wrong_half: false,
        }
    }

    async fn run(mut self, mut events: mpsc::UnboundedReceiver<Event>) {
        loop {
            let event = tokio::select! {
                event = events.recv() => match event {
                    Some(event) => event,
                    None => return,
                },
                () = idle(self.lock_at) => {
                    self.lock_at = None;
                    let notification = self.keyboard.lock();
                    self.output(notification.into_iter().collect()).await;
                    continue;
                }
            };
            self.handle(event).await;
        }
    }

    async fn handle(&mut self, event: Event) {
        match event {
            Event::Connect {
                connection,
                to_client,
            } => {
                self.connection = Some(Connection {
                    id: connection,
                    to_client,
                });
            }
            Event::Bytes { connection, bytes } => self.read(connection, &bytes).await,
            Event::PressUnlockKey => {
                let notification = self.keyboard.unlock();
                self.lock_at = Some(Instant::now() + IDLE_LOCK_TIMEOUT);
                self.output(notification.into_iter().collect()).await;
            }
            Event::Lock => {
                let notification = self.keyboard.lock();
                self.lock_at = None;
                self.output(notification.into_iter().collect()).await;
            }
            Event::Restart => {
                self.keyboard.restart();
                self.receiver = Receiver::new();
                self.lock_at = None;
                self.connection = None;
            }
            Event::SetLatency(latency) => self.latency = latency,
            Event::DropResponses(count) => self.responses_to_drop = count,
            Event::DropNotifications(count) => self.notifications_to_drop = count,
            Event::DropReceived(count) => self.received_to_drop = count,
            Event::InjectGarbage(bytes) => self.garbage.extend(bytes),
            Event::SwitchTransport { away } => {
                if away {
                    // The notification goes to the other transport.
                    let _ = self.keyboard.lock();
                }
                self.transport_away = away;
            }
            Event::SetWrongHalf(wrong_half) => self.wrong_half = wrong_half,
            Event::Inspect(reply) => {
                let _ = reply.send(self.keyboard.clone());
            }
        }
    }

    /// Reads bytes from a connection, and answers each request in them.
    async fn read(&mut self, connection: u64, bytes: &[u8]) {
        if self.transport_away || self.wrong_half {
            return;
        }
        if self
            .connection
            .as_ref()
            .is_none_or(|current| current.id != connection)
        {
            return;
        }
        let kept = bytes.len().saturating_sub(self.received_to_drop);
        self.received_to_drop = 0;
        let requests = self.receiver.receive(&bytes[..kept], Instant::now());
        for request in requests {
            let Some(mut messages) = self.keyboard.handle(&request) else {
                continue;
            };
            self.lock_at = Some(Instant::now() + IDLE_LOCK_TIMEOUT);
            if self.responses_to_drop > 0 {
                self.responses_to_drop -= 1;
                messages.retain(|message| matches!(message, DeviceMessage::Notification(_)));
            }
            self.output(messages).await;
        }
    }

    /// Sends messages, after any garbage, on this connection, unless the keyboard serves
    /// another transport, or this connection reaches the wrong half.
    async fn output(&mut self, mut messages: Vec<DeviceMessage>) {
        if self.transport_away || self.wrong_half {
            return;
        }
        messages.retain(|message| {
            let lost =
                matches!(message, DeviceMessage::Notification(_)) && self.notifications_to_drop > 0;
            if lost {
                self.notifications_to_drop -= 1;
            }
            !lost
        });
        let Some(to_client) = self
            .connection
            .as_ref()
            .map(|current| current.to_client.clone())
        else {
            return;
        };
        if messages.is_empty() && self.garbage.is_empty() {
            return;
        }
        self.receiver.answered();
        let mut bytes = std::mem::take(&mut self.garbage);
        for message in &messages {
            bytes.extend(frame::encode(&message.encode()));
        }
        if !self.latency.is_zero() {
            sleep(self.latency).await;
        }
        for chunk in bytes.chunks(CHUNK_LEN) {
            // The client may have gone; the keyboard doesn't care.
            let _ = to_client.send(chunk.to_vec());
        }
    }
}

/// Waits until the keyboard locks itself, or forever if it won't.
async fn idle(lock_at: Option<Instant>) {
    match lock_at {
        Some(deadline) => sleep_until(deadline).await,
        None => future::pending().await,
    }
}
