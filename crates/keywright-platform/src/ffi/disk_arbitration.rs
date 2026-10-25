//! DiskArbitration's notifications, declared by hand: `objc2-disk-arbitration` doesn't have
//! the calls that register them. A drive appearing, being mounted, unmounted and going
//! away, for UF2 drives (tech spec §11.3). Watching needs no permission.

#![allow(unsafe_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, c_char, c_void};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::thread;

use objc2_core_foundation::{
    CFArray, CFDictionary, CFRetained, CFRunLoop, CFRunLoopRunResult, CFString, CFType, CFURL,
    kCFRunLoopDefaultMode,
};

use super::iokit;
use crate::uf2::{Drive, DriveError, DriveEvent, Watch};

/// A DiskArbitration session (`DASessionRef`), or a disk (`DADiskRef`): CoreFoundation
/// objects.
type DaObject = CFType;

/// What DiskArbitration calls when a disk appears or goes away.
type DiskCallback = unsafe extern "C-unwind" fn(disk: NonNull<DaObject>, context: *mut c_void);

/// What DiskArbitration calls when a disk's description changes, such as when it's mounted.
type ChangedCallback = unsafe extern "C-unwind" fn(
    disk: NonNull<DaObject>,
    keys: NonNull<CFArray>,
    context: *mut c_void,
);

/// The USB device's property with its serial number, which IOKit's USB family sets.
const USB_SERIAL_NUMBER: &str = "USB Serial Number";

/// How long the watching thread's run loop runs before it checks whether to stop, in
/// seconds.
const SLICE: f64 = 0.2;

#[link(name = "DiskArbitration", kind = "framework")]
unsafe extern "C" {
    static kDADiskDescriptionVolumeNameKey: Option<&'static CFString>;
    static kDADiskDescriptionVolumePathKey: Option<&'static CFString>;

    fn DASessionCreate(allocator: *const c_void) -> Option<NonNull<DaObject>>;
    fn DASessionScheduleWithRunLoop(session: &DaObject, run_loop: &CFRunLoop, mode: &CFString);
    fn DASessionUnscheduleFromRunLoop(session: &DaObject, run_loop: &CFRunLoop, mode: &CFString);
    fn DARegisterDiskAppearedCallback(
        session: &DaObject,
        matching: Option<&CFDictionary>,
        callback: DiskCallback,
        context: *mut c_void,
    );
    fn DARegisterDiskDisappearedCallback(
        session: &DaObject,
        matching: Option<&CFDictionary>,
        callback: DiskCallback,
        context: *mut c_void,
    );
    fn DARegisterDiskDescriptionChangedCallback(
        session: &DaObject,
        matching: Option<&CFDictionary>,
        watch: Option<&CFArray>,
        callback: ChangedCallback,
        context: *mut c_void,
    );
    fn DAUnregisterCallback(session: &DaObject, callback: *const c_void, context: *mut c_void);
    fn DADiskCopyDescription(disk: &DaObject) -> Option<NonNull<CFDictionary>>;
    fn DADiskCopyIOMedia(disk: &DaObject) -> iokit::IoObject;
    fn DADiskGetBSDName(disk: &DaObject) -> *const c_char;
}

/// What the callbacks share: the drives named `name` that are mounted, by BSD name, such
/// as `disk4`, and what to tell.
struct Watcher {
    name: String,
    mounted: RefCell<HashMap<String, Drive>>,
    event: Box<dyn Fn(DriveEvent) + Send>,
    stop: Arc<AtomicBool>,
}

/// Calls `event` on a thread of its own each time a drive named `name` is mounted, or
/// unmounted or goes away, until the returned [`Watch`] is dropped. The drives mounted
/// already count as appearing.
pub fn watch(name: String, event: Box<dyn Fn(DriveEvent) + Send>) -> Result<Watch, DriveError> {
    let stop = Arc::new(AtomicBool::new(false));
    let (started, start) = mpsc::sync_channel(1);
    let watcher = Watcher {
        name,
        mounted: RefCell::new(HashMap::new()),
        event,
        stop: Arc::clone(&stop),
    };
    thread::Builder::new()
        .name("keywright-uf2-drives".to_owned())
        .spawn(move || run(&watcher, &started))
        .map_err(|error| DriveError::Failed(error.to_string()))?;
    start
        .recv()
        .map_err(|_| DriveError::Failed("the watching thread stopped".to_owned()))??;
    Ok(Watch::new(stop))
}

/// The watching thread: sessions' callbacks come on its run loop, which runs until told to
/// stop.
fn run(watcher: &Watcher, started: &mpsc::SyncSender<Result<(), DriveError>>) {
    let session = match Session::start(watcher) {
        Ok(session) => session,
        Err(error) => {
            let _ = started.send(Err(error));
            return;
        }
    };
    let _ = started.send(Ok(()));
    // SAFETY: CoreFoundation's constant, set before any of our code runs.
    let mode = unsafe { kCFRunLoopDefaultMode };
    while !watcher.stop.load(Ordering::Relaxed) {
        if CFRunLoop::run_in_mode(mode, SLICE, false) == CFRunLoopRunResult::Finished {
            // Nothing is scheduled any more, so the run loop doesn't wait.
            thread::sleep(std::time::Duration::from_secs_f64(SLICE));
        }
    }
    drop(session);
}

/// A DiskArbitration session, scheduled on the current thread's run loop with the
/// callbacks registered, until dropped. It mustn't outlive the watcher its callbacks get.
struct Session {
    object: CFRetained<DaObject>,
    run_loop: CFRetained<CFRunLoop>,
    context: *mut c_void,
    /// The description's keys the session watches, kept for as long as it does.
    _keys: CFRetained<CFArray<CFString>>,
}

impl Session {
    fn start(watcher: &Watcher) -> Result<Self, DriveError> {
        let failed = || DriveError::Failed("DiskArbitration has no session".to_owned());
        // SAFETY: a null allocator is the default one. It returns a session we own, or null.
        let session = unsafe { DASessionCreate(std::ptr::null()) }
            // SAFETY: as just said, we own it (the "Create" rule).
            .map(|session| unsafe { CFRetained::from_raw(session) })
            .ok_or_else(failed)?;
        let run_loop = CFRunLoop::current().ok_or_else(failed)?;
        // SAFETY: CoreFoundation's constant, set before any of our code runs.
        let mode = unsafe { kCFRunLoopDefaultMode }.ok_or_else(failed)?;
        // SAFETY: DiskArbitration's constant, set before any of our code runs.
        let path_key = unsafe { kDADiskDescriptionVolumePathKey }.ok_or_else(failed)?;
        let keys = CFArray::from_objects(&[path_key]);
        let context = std::ptr::from_ref(watcher).cast_mut().cast::<c_void>();
        // SAFETY: the session is live; no matching dictionary matches every disk; the
        // callbacks treat `context` as the watcher it is, which outlives the session, since
        // dropping the session unregisters them; `keys` lives as long as the session.
        unsafe {
            DARegisterDiskAppearedCallback(&session, None, appeared, context);
            DARegisterDiskDisappearedCallback(&session, None, disappeared, context);
            DARegisterDiskDescriptionChangedCallback(
                &session,
                None,
                Some(keys.as_opaque()),
                changed,
                context,
            );
            DASessionScheduleWithRunLoop(&session, &run_loop, mode);
        }
        Ok(Self {
            object: session,
            run_loop,
            context,
            _keys: keys,
        })
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // SAFETY: the session is live, and was scheduled on this run loop in this mode, with
        // these callbacks and context. Nothing calls them once this returns.
        unsafe {
            if let Some(mode) = kCFRunLoopDefaultMode {
                DASessionUnscheduleFromRunLoop(&self.object, &self.run_loop, mode);
            }
            DAUnregisterCallback(&self.object, appeared as *const c_void, self.context);
            DAUnregisterCallback(&self.object, disappeared as *const c_void, self.context);
            DAUnregisterCallback(&self.object, changed as *const c_void, self.context);
        }
    }
}

/// DiskArbitration's callback for a disk appearing, the ones there already included.
unsafe extern "C-unwind" fn appeared(disk: NonNull<DaObject>, context: *mut c_void) {
    // SAFETY: DiskArbitration passes a live disk, and the context `Session::start` gave it.
    unsafe { notify(disk, context, Watcher::update) };
}

/// DiskArbitration's callback for a disk's description changing: here, its volume path,
/// as when it's mounted or unmounted.
unsafe extern "C-unwind" fn changed(
    disk: NonNull<DaObject>,
    _keys: NonNull<CFArray>,
    context: *mut c_void,
) {
    // SAFETY: as for `appeared`.
    unsafe { notify(disk, context, Watcher::update) };
}

/// DiskArbitration's callback for a disk going away.
unsafe extern "C-unwind" fn disappeared(disk: NonNull<DaObject>, context: *mut c_void) {
    // SAFETY: as for `appeared`.
    unsafe { notify(disk, context, Watcher::gone) };
}

/// Hands a callback's disk to the watcher.
///
/// # Safety
///
/// `disk` must be live for the call, and `context` must be a live [`Watcher`].
unsafe fn notify(disk: NonNull<DaObject>, context: *mut c_void, handle: fn(&Watcher, &DaObject)) {
    // SAFETY: as the caller promises.
    let (disk, watcher) = unsafe { (disk.as_ref(), &*context.cast::<Watcher>()) };
    // A panic mustn't unwind into DiskArbitration. If one happens, this change is missed.
    let _ = catch_unwind(AssertUnwindSafe(|| handle(watcher, disk)));
}

impl Watcher {
    /// A disk appeared or changed: tells if it's one of the drives and was just mounted,
    /// or was just unmounted.
    fn update(&self, disk: &DaObject) {
        let Some(bsd_name) = bsd_name(disk) else {
            return;
        };
        let path = mount(disk, &self.name);
        let event = {
            let mut mounted = self.mounted.borrow_mut();
            match path {
                Some(path) if !mounted.contains_key(&bsd_name) => {
                    let drive = Drive {
                        path,
                        // SAFETY: the disk is live; it returns its media, which we own.
                        serial_number: iokit::provider_string(
                            unsafe { DADiskCopyIOMedia(disk) },
                            USB_SERIAL_NUMBER,
                        ),
                    };
                    mounted.insert(bsd_name, drive.clone());
                    Some(DriveEvent::Appeared(drive))
                }
                Some(_) => None,
                None => mounted.remove(&bsd_name).map(DriveEvent::Disappeared),
            }
        };
        self.tell(event);
    }

    /// A disk went away: tells if it was one of the drives, mounted.
    fn gone(&self, disk: &DaObject) {
        let event = bsd_name(disk)
            .and_then(|bsd_name| self.mounted.borrow_mut().remove(&bsd_name))
            .map(DriveEvent::Disappeared);
        self.tell(event);
    }

    fn tell(&self, event: Option<DriveEvent>) {
        if let Some(event) = event
            && !self.stop.load(Ordering::Relaxed)
        {
            (self.event)(event);
        }
    }
}

/// The disk's BSD name, such as `disk4`.
fn bsd_name(disk: &DaObject) -> Option<String> {
    // SAFETY: the disk is live. It returns a C string the disk owns, or null.
    let name = unsafe { DADiskGetBSDName(disk) };
    if name.is_null() {
        return None;
    }
    // SAFETY: it's a C string, live while the disk is.
    let name = unsafe { CStr::from_ptr(name) };
    Some(name.to_string_lossy().into_owned())
}

/// Where the disk is mounted, if it's a volume named `name` and mounted.
fn mount(disk: &DaObject, name: &str) -> Option<std::path::PathBuf> {
    // SAFETY: the disk is live. It returns a description we own, or null.
    let description = unsafe { DADiskCopyDescription(disk) }
        // SAFETY: as just said, we own it (the "Copy" rule).
        .map(|description| unsafe { CFRetained::from_raw(description) })?;
    // SAFETY: DiskArbitration's constants, set before any of our code runs.
    let (name_key, path_key) = unsafe {
        (
            kDADiskDescriptionVolumeNameKey?,
            kDADiskDescriptionVolumePathKey?,
        )
    };
    // SAFETY: a description's keys are strings.
    let description = unsafe { description.cast_unchecked::<CFString, CFType>() };
    let volume = description.get(name_key)?.downcast::<CFString>().ok()?;
    if volume.to_string() != name {
        return None;
    }
    description
        .get(path_key)?
        .downcast::<CFURL>()
        .ok()?
        .to_file_path()
}
