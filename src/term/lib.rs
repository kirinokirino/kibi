pub mod ansi_escape;
pub mod terminal;

#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use windows as sys;

#[cfg(unix)]
pub mod unix;
#[cfg(unix)]
pub use unix as sys;

#[cfg(target_os = "wasi")]
pub mod wasi;
#[cfg(target_os = "wasi")]
pub use wasi as sys;

pub mod error;

pub fn test() -> bool {
	true
}
