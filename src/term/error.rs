#[derive(Debug)]
pub enum Error {
	/// Error returned when the window size obtained through a system call is invalid.
	InvalidWindowSize,
	/// Error setting or retrieving the cursor position.
	CursorPosition,
	/// Wrapper around `std::io::Error`
	Io(std::io::Error),
}

impl From<std::io::Error> for Error {
	/// Convert an Io Error into a term Error.
	fn from(err: std::io::Error) -> Self {
		Self::Io(err)
	}
}
