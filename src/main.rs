//! # Kibi
//!
//! Kibi is a text editor in ≤1024 lines of code.

pub use crate::{config::Config, editor::Editor, error::Error};

use term::ansi_escape;
mod config;
mod editor;
mod error;
mod row;
mod syntax;
mod terminal;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as sys;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
mod xdg;
#[cfg(unix)]
use unix as sys;

#[cfg(target_os = "wasi")]
mod wasi;
#[cfg(target_os = "wasi")]
mod xdg;
#[cfg(target_os = "wasi")]
use wasi as sys;

/// Load the configuration, initialize the editor and run the program, optionally opening a file if
/// an argument is given.
///
/// # Errors
///
/// Any error that occur during the execution of the program will be returned by this function.
fn main() -> Result<(), Error> {
    let mut args = std::env::args();
    match (args.nth(1), /*remaining_args=*/ args.len()) {
        (Some(arg), 0) if arg == "--version" => println!("kibi {}", env!("KIBI_VERSION")),
        (Some(arg), 0) if arg.starts_with('-') => return Err(Error::UnrecognizedOption(arg)),
        (file_name, 0) => Editor::new(Config::load()?)?.run(&file_name)?,
        (_, n_remaining_args) => return Err(Error::TooManyArguments(n_remaining_args + 1)),
    }
    Ok(())
}
