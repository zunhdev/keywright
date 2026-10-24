//! Checks tauri.conf.json and the capability files while compiling, and generates the
//! permissions the capabilities refer to.

fn main() {
    tauri_build::build();
}
