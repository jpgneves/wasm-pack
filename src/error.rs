//! Code related to error handling for wasm-pack
use serde_json;
use std::borrow::Cow;
use std::io;
use std::process::ExitStatus;
use toml;

/// Errors that can potentially occur in `wasm-pack`.
#[derive(Debug, Fail)]
pub enum Error {
    /// Maps any underlying I/O errors that are thrown to this variant
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    /// A JSON serialization or deserialization error.
    #[fail(display = "{}", _0)]
    SerdeJson(#[cause] serde_json::Error),

    /// A TOML serialization or deserialization error.
    #[fail(display = "{}", _0)]
    SerdeToml(#[cause] toml::de::Error),

    #[fail(display = "{}", _0)]
    /// An error in parsing your rustc version.
    RustcMissing {
        /// Error message
        message: String,
    },

    #[fail(display = "{}", _0)]
    /// An error from having an unsupported rustc version.
    RustcVersion {
        /// Error message
        message: String,
        /// The minor version of the local rust
        local_minor_version: String,
    },

    /// An error invoking another CLI tool.
    #[fail(display = "`{}` exited with {}", tool, exit_status)]
    Cli {
        /// Error message.
        tool: String,
        /// The underlying CLI's `stdout` output.
        stdout: String,
        /// The underlying CLI's `stderr` output.
        stderr: String,
        /// The exit status of the subprocess
        exit_status: ExitStatus,
    },

    /// A crate configuration error.
    #[fail(display = "{}", message)]
    CrateConfig {
        /// A message describing the configuration error.
        message: String,
    },

    #[fail(display = "{}", message)]
    /// Error when the 'pkg' directory is not found.
    PkgNotFound {
        /// Message describing the error.
        message: String,
    },

    #[fail(display = "{}", message)]
    /// Error when some operation or feature is unsupported for the current
    /// target or environment.
    Unsupported {
        /// Error message.
        message: String,
    },
}

impl Error {
    /// Construct a CLI error.
    pub fn cli(tool: &str, stdout: Cow<str>, stderr: Cow<str>, exit_status: ExitStatus) -> Self {
        Error::Cli {
            tool: tool.to_string(),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            exit_status,
        }
    }

    /// Construct a crate configuration error.
    pub fn crate_config(message: &str) -> Self {
        Error::CrateConfig {
            message: message.to_string(),
        }
    }

    /// Construct an unsupported error.
    pub fn unsupported(message: &str) -> Self {
        Error::Unsupported {
            message: message.to_string(),
        }
    }

    /// Construct a rustc version error.
    pub fn rustc_version_error(message: &str, local_version: &str) -> Self {
        Error::RustcVersion {
            message: message.to_string(),
            local_minor_version: local_version.to_string(),
        }
    }

    /// Get a string description of this error's type.
    pub fn error_type(&self) -> String {
        match self {
            Error::Io(_) => "There was an I/O error. Details:\n\n",
            Error::SerdeJson(_) => "There was an JSON error. Details:\n\n",
            Error::SerdeToml(_) => "There was an TOML error. Details:\n\n",
            Error::RustcMissing {
              message: _,
            } =>  "We can't figure out what your Rust version is- which means you might not have Rust installed. Please install Rust version 1.30.0 or higher.",
            Error::RustcVersion {
              message: _,
              local_minor_version: _,
            } => "Your rustc version is not supported. Please install version 1.30.0 or higher.",
            Error::Cli {
                tool: _,
                stdout: _,
                stderr: _,
                exit_status: _,
            } => "There was an error while calling another CLI tool. Details:\n\n",
            Error::CrateConfig { message: _ } => {
                "There was a crate configuration error. Details:\n\n"
            }
            Error::PkgNotFound {
                message: _,
            } => "Unable to find the 'pkg' directory at the path, set the path as the parent of the 'pkg' directory \n\n",
            Error::Unsupported {..} => "There was an unsupported operation attempted. Details:\n\n",
        }.to_string()
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::SerdeToml(e)
    }
}
