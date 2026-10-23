//! Writing results and messages.
//!
//! Results go to standard output: text, or JSON with `--json`. Messages for the person at
//! the keyboard, such as how to unlock it, go to standard error, so they never mix with a
//! result a script reads.

use std::fmt::Display;
use std::io::{self, Write as _};

use serde::Serialize;

use crate::failure::Failure;

/// Writes a result to standard output. Output that can't be written, as when it goes to a
/// program that has stopped reading, fails the command rather than crashing it.
pub(crate) fn print(text: impl Display) -> Result<(), Failure> {
    writeln!(io::stdout().lock(), "{text}")
        .map_err(|error| Failure::file(format!("Couldn't write the output: {error}.")))
}

/// Writes a result to standard output as JSON.
pub(crate) fn print_json(value: &impl Serialize) -> Result<(), Failure> {
    let json = serde_json::to_string_pretty(value)
        .map_err(|error| Failure::file(format!("Couldn't write the output: {error}.")))?;
    print(json)
}

/// Writes a message for the person to standard error. If that fails, there is nobody to
/// tell.
pub(crate) fn tell(text: impl Display) {
    let _ = writeln!(io::stderr().lock(), "{text}");
}

/// Asks the person a question on standard error, leaving the cursor after it for the
/// answer.
pub(crate) fn ask(question: impl Display) {
    let mut stderr = io::stderr().lock();
    let _ = write!(stderr, "{question} ");
    let _ = stderr.flush();
}
