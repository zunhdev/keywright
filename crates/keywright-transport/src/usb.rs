//! USB serial: finding ZMK keyboards' serial ports, watching them come and go, and a
//! [`Transport`] over one (tech spec §6.1).
//!
//! A ZMK keyboard enumerates a USB serial port (CDC ACM) with ZMK's VID and PID. On a split
//! keyboard both halves do, but only the half connected to the computer serves Studio; the
//! other never answers. That half types over the same cable, so its USB device has a
//! keyboard (HID) interface, and the other half's doesn't ([`UsbPort::keyboard_interface`],
//! spike S4): `serialport` sees only the serial interface, so [`ports`] reads the devices'
//! interfaces with `nusb`, matched by USB serial number. A port belongs to the keyboard
//! whose `get_device_info` reports the serial number the port has, in hex
//! ([`UsbPort::is_device`]). Spikes S1 and S4 checked all of this on the Sofle.
//!
//! [`UsbTransport`] reads on a thread of its own, which hands chunks to
//! [`Transport::recv`] over a channel, so `recv` is cancel-safe. Writes go to a second
//! thread, each frame in one write, without pacing: ZMK v0.3 corrupts a string that arrives
//! in pieces. When the transport is dropped, both threads discard any unsent bytes, so
//! closing the port never waits for a keyboard that has stopped reading.

#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::io::{self, Read as _, Write as _};
use std::sync::mpsc as std_mpsc;
use std::thread;
use std::time::Duration;

use keywright_studio::transport::{Transport, TransportError, TransportKind};
use nusb::MaybeFuture as _;
use serialport::{ClearBuffer, SerialPort, SerialPortInfo, SerialPortType};
use tokio::sync::{mpsc, oneshot};

/// ZMK's USB vendor ID.
pub const ZMK_VID: u16 = 0x1D50;

/// ZMK's USB product ID.
pub const ZMK_PID: u16 = 0x615E;

/// The USB interface class of a keyboard, and of HID devices in general.
const HID_CLASS: u8 = 0x03;

/// How long a read waits for bytes before checking that the transport is still wanted.
const READ_POLL: Duration = Duration::from_millis(50);

/// How long a write may take. A keyboard that is reading takes a frame at once.
const WRITE_TIMEOUT: Duration = Duration::from_millis(500);

/// The baud rate, which a USB serial port ignores.
const BAUD_RATE: u32 = 115_200;

/// A ZMK keyboard's USB serial port.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UsbPort {
    /// The device's path, such as `/dev/cu.usbmodem101`. It follows the USB socket.
    pub path: String,
    /// The USB serial number: the keyboard's hardware ID, in uppercase hex.
    pub serial_number: Option<String>,
    /// The USB product string: the board's name, such as `Sofle Choc Pro`. The half of a
    /// split keyboard that doesn't serve Studio may have another, such as `Generic CDC`.
    pub product: Option<String>,
    /// Whether the port's USB device has a keyboard (HID) interface, which the half of a
    /// split keyboard that talks to the computer always has: it types over the same cable.
    /// The other half's has only the serial ones (spike S4). `None` if it couldn't be read.
    pub keyboard_interface: Option<bool>,
}

impl UsbPort {
    /// Whether this is the port of the keyboard whose `get_device_info` reports this serial
    /// number. A keyboard that can't read its hardware ID reports none, and matches no port.
    #[must_use]
    pub fn is_device(&self, serial_number: &[u8]) -> bool {
        let mut hex = String::with_capacity(serial_number.len() * 2);
        for byte in serial_number {
            // Writing to a `String` can't fail.
            let _ = write!(hex, "{byte:02X}");
        }
        !hex.is_empty()
            && self
                .serial_number
                .as_deref()
                .is_some_and(|usb| usb.eq_ignore_ascii_case(&hex))
    }
}

/// The serial ports of the ZMK keyboards connected now, in order of path, with whether
/// each one's USB device has a keyboard interface. Listing blocks for a moment.
///
/// # Errors
///
/// Fails if the system can't list its serial ports. If it can't list the USB devices, the
/// ports say nothing about their interfaces.
pub fn ports() -> io::Result<Vec<UsbPort>> {
    let ports = zmk_ports(serialport::available_ports()?);
    Ok(with_devices(ports, &zmk_devices()))
}

/// What a ZMK keyboard's USB device says about itself, beyond its serial port.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Device {
    product: Option<String>,
    keyboard_interface: bool,
}

/// The USB devices with ZMK's IDs, by serial number, or none if they can't be listed.
fn zmk_devices() -> HashMap<String, Device> {
    let devices = match nusb::list_devices().wait() {
        Ok(devices) => devices,
        Err(error) => {
            tracing::warn!(%error, "couldn't list the USB devices");
            return HashMap::new();
        }
    };
    devices
        .filter(|device| (device.vendor_id(), device.product_id()) == (ZMK_VID, ZMK_PID))
        .filter_map(|device| {
            let serial_number = device.serial_number()?.to_ascii_uppercase();
            let found = Device {
                product: device.product_string().map(str::to_owned),
                keyboard_interface: device
                    .interfaces()
                    .any(|interface| interface.class() == HID_CLASS),
            };
            Some((serial_number, found))
        })
        .collect()
}

/// The ports, with what their USB devices, by serial number, say: whether they have a
/// keyboard interface, and the product, where the serial interface has none.
fn with_devices(ports: Vec<UsbPort>, devices: &HashMap<String, Device>) -> Vec<UsbPort> {
    ports
        .into_iter()
        .map(|port| {
            let device = (port.serial_number.as_deref())
                .and_then(|serial| devices.get(&serial.to_ascii_uppercase()));
            let Some(device) = device else {
                return port;
            };
            UsbPort {
                product: port.product.or_else(|| device.product.clone()),
                keyboard_interface: Some(device.keyboard_interface),
                ..port
            }
        })
        .collect()
}

/// Keeps the USB ports with ZMK's IDs, in order of path. Of a callout device
/// (`/dev/cu.*`) and its dial-in twin (`/dev/tty.*`), only the callout device is kept: the
/// dial-in device waits for a carrier when opened.
fn zmk_ports(ports: Vec<SerialPortInfo>) -> Vec<UsbPort> {
    let names: HashSet<String> = ports.iter().map(|port| port.port_name.clone()).collect();
    let mut zmk: Vec<UsbPort> = ports
        .into_iter()
        .filter_map(|port| {
            let SerialPortType::UsbPort(usb) = port.port_type else {
                return None;
            };
            if (usb.vid, usb.pid) != (ZMK_VID, ZMK_PID) {
                return None;
            }
            if let Some(name) = port.port_name.strip_prefix("/dev/tty.")
                && names.contains(&format!("/dev/cu.{name}"))
            {
                return None;
            }
            Some(UsbPort {
                path: port.port_name,
                serial_number: usb.serial_number,
                product: usb.product,
                keyboard_interface: None,
            })
        })
        .collect();
    zmk.sort_by(|a, b| a.path.cmp(&b.path));
    zmk
}

/// A keyboard's port appearing or going away.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortEvent {
    /// A keyboard was plugged in.
    Connected(UsbPort),
    /// A keyboard was unplugged.
    Disconnected(UsbPort),
}

/// Watches for keyboards being plugged in and unplugged, by listing the ports every
/// `interval`, in a task on the current Tokio runtime. It first reports every keyboard
/// connected now. It stops when the receiver is dropped.
///
/// # Panics
///
/// Panics if called outside a Tokio runtime.
#[must_use]
pub fn watch(interval: Duration) -> mpsc::Receiver<PortEvent> {
    let (events, receiver) = mpsc::channel(16);
    tokio::spawn(async move {
        let mut known = Vec::new();
        loop {
            match tokio::task::spawn_blocking(ports).await {
                Ok(Ok(now)) => {
                    for event in changes(&known, &now) {
                        if events.send(event).await.is_err() {
                            return;
                        }
                    }
                    known = now;
                }
                Ok(Err(error)) => tracing::warn!(%error, "couldn't list the serial ports"),
                Err(error) => tracing::warn!(%error, "listing the serial ports failed"),
            }
            tokio::select! {
                () = tokio::time::sleep(interval) => {}
                () = events.closed() => return,
            }
        }
    });
    receiver
}

/// What changed from one list of ports to the next: the ports that went away, then the
/// ports that appeared. A port whose serial number changed went away and appeared again.
fn changes(before: &[UsbPort], after: &[UsbPort]) -> Vec<PortEvent> {
    let gone = before
        .iter()
        .filter(|port| !after.contains(port))
        .cloned()
        .map(PortEvent::Disconnected);
    let new = after
        .iter()
        .filter(|port| !before.contains(port))
        .cloned()
        .map(PortEvent::Connected);
    gone.chain(new).collect()
}

/// A connection to a keyboard over its USB serial port.
#[derive(Debug)]
pub struct UsbTransport {
    to_writer: std_mpsc::Sender<Frame>,
    from_reader: mpsc::UnboundedReceiver<io::Result<Vec<u8>>>,
}

/// Bytes for the writer thread, and the way to report how writing them went.
struct Frame {
    bytes: Vec<u8>,
    written: oneshot::Sender<io::Result<()>>,
}

impl UsbTransport {
    /// Opens a keyboard's port. Nothing else can open it while the transport exists.
    ///
    /// # Errors
    ///
    /// Fails if the port can't be opened, for example because the keyboard was unplugged
    /// or another program has the port open.
    pub fn open(port: &UsbPort) -> Result<Self, TransportError> {
        let serial = serialport::new(&port.path, BAUD_RATE)
            .timeout(READ_POLL)
            .open()
            .map_err(io::Error::from)?;
        Self::over(serial)
    }

    /// A transport over an open port, which should time out its reads after [`READ_POLL`].
    fn over(reader: Box<dyn SerialPort>) -> Result<Self, TransportError> {
        let mut writer = reader.try_clone().map_err(io::Error::from)?;
        writer.set_timeout(WRITE_TIMEOUT).map_err(io::Error::from)?;
        let (to_client, from_reader) = mpsc::unbounded_channel();
        let (to_writer, frames) = std_mpsc::channel();
        thread::Builder::new()
            .name("keywright-usb-reader".to_owned())
            .spawn(move || read(reader, &to_client))?;
        thread::Builder::new()
            .name("keywright-usb-writer".to_owned())
            .spawn(move || write(writer, &frames))?;
        Ok(Self {
            to_writer,
            from_reader,
        })
    }
}

impl Transport for UsbTransport {
    fn send(&mut self, bytes: &[u8]) -> impl Future<Output = Result<(), TransportError>> + Send {
        let (written, result) = oneshot::channel();
        let queued = self.to_writer.send(Frame {
            bytes: bytes.to_vec(),
            written,
        });
        async move {
            queued.map_err(|_| TransportError::Closed)?;
            result
                .await
                .map_err(|_| TransportError::Closed)?
                .map_err(transport_error)
        }
    }

    async fn recv(&mut self) -> Result<Vec<u8>, TransportError> {
        match self.from_reader.recv().await {
            Some(Ok(chunk)) => Ok(chunk),
            Some(Err(error)) => Err(transport_error(error)),
            None => Err(TransportError::Closed),
        }
    }

    fn kind(&self) -> TransportKind {
        TransportKind::Usb
    }
}

/// Reads chunks into the channel until the port fails or the transport is dropped.
fn read(mut port: Box<dyn SerialPort>, to_client: &mpsc::UnboundedSender<io::Result<Vec<u8>>>) {
    let mut buffer = [0; 1024];
    while !to_client.is_closed() {
        match port.read(&mut buffer) {
            // The other end is gone.
            Ok(0) => break,
            Ok(len) => {
                if to_client.send(Ok(buffer[..len].to_vec())).is_err() {
                    break;
                }
            }
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::TimedOut
                        | io::ErrorKind::Interrupted
                        | io::ErrorKind::WouldBlock
                ) => {}
            Err(error) => {
                let _ = to_client.send(Err(error));
                break;
            }
        }
    }
    // Discard what the keyboard hasn't taken, so closing the port doesn't wait for it.
    let _ = port.clear(ClearBuffer::All);
}

/// Writes each frame in one write, until a write fails or the transport is dropped.
fn write(mut port: Box<dyn SerialPort>, frames: &std_mpsc::Receiver<Frame>) {
    for Frame { bytes, written } in frames {
        // Not flushed: on a serial port, flushing waits, without a timeout, until the
        // keyboard has taken every byte, and a keyboard that stops reading never does.
        let result = port.write_all(&bytes);
        let failed = result.is_err();
        // The caller may have stopped waiting.
        let _ = written.send(result);
        if failed {
            break;
        }
    }
    let _ = port.clear(ClearBuffer::Output);
}

/// A broken pipe means the keyboard is gone, as when it's unplugged.
fn transport_error(error: io::Error) -> TransportError {
    if error.kind() == io::ErrorKind::BrokenPipe {
        TransportError::Closed
    } else {
        TransportError::Io(error)
    }
}
