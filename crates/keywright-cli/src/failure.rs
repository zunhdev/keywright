//! How commands end, and the exit codes that say so (CLI-4). `keywright --help` lists the
//! codes ([`EXIT_CODES`](crate::EXIT_CODES)).

use std::fmt;
use std::process::ExitCode;

use serde::Serialize;

/// How a command ended, when it didn't fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Exit {
    /// Done. For `diff`, the keyboard matches the file. Exit code 0.
    Done,
    /// `diff` found differences, or the person answered no when `restore` asked. Exit
    /// code 1.
    No,
    /// The command has explained a problem itself, as `status` does for a keyboard it
    /// can't edit. The exit code says what kind.
    Explained(Kind),
}

impl Exit {
    pub(crate) fn code(self) -> ExitCode {
        match self {
            Self::Done => ExitCode::SUCCESS,
            Self::No => ExitCode::from(1),
            Self::Explained(kind) => kind.code(),
        }
    }
}

/// What kind of problem stopped a command. Each has its exit code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Kind {
    /// The command line is wrong, or `restore` needs `--yes`. Exit code 2, as for the
    /// mistakes the argument parser finds.
    Usage,
    /// No keyboard that Keywright can edit is connected. Exit code 3.
    NoKeyboard,
    /// The keyboard stayed locked. Exit code 4.
    Locked,
    /// The keyboard didn't do what it was asked, or its firmware doesn't work with
    /// Keywright. Exit code 5.
    Keyboard,
    /// A file couldn't be read or written, or isn't a backup for this keyboard. Exit
    /// code 6.
    File,
}

impl Kind {
    pub(crate) fn code(self) -> ExitCode {
        ExitCode::from(match self {
            Self::Usage => 2,
            Self::NoKeyboard => 3,
            Self::Locked => 4,
            Self::Keyboard => 5,
            Self::File => 6,
        })
    }
}

/// Why a command stopped, in plain words.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct Failure {
    kind: Kind,
    message: String,
}

impl Failure {
    pub(crate) fn new(kind: Kind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub(crate) fn usage(message: impl Into<String>) -> Self {
        Self::new(Kind::Usage, message)
    }

    pub(crate) fn keyboard(message: impl Into<String>) -> Self {
        Self::new(Kind::Keyboard, message)
    }

    pub(crate) fn file(message: impl Into<String>) -> Self {
        Self::new(Kind::File, message)
    }

    pub(crate) fn exit_code(&self) -> ExitCode {
        self.kind.code()
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}
