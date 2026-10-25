//! Calls into the operating system's own APIs: the only place in Keywright where `unsafe`
//! is allowed (tech spec §4). Each module explains why its calls are sound.

pub mod appkit;
pub mod carbon;
pub mod core_bluetooth;
pub mod foundation;
pub mod service_management;
