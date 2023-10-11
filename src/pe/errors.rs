#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;
#[cfg(feature = "write")]
#[doc(inline)]
pub use write::*;

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  #[derive(Debug)]
  pub enum ReadManagedPeError {
    /// The pe file was not valid.
    InvalidPeFile,
    /// The CLI header was not found.
    MissingCliHeader,
  }

  #[cfg(feature = "object")]
  impl From<object::Error> for ReadManagedPeError {
    fn from(_: object::Error) -> Self {
      Self::InvalidPeFile
    }
  }
}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {}
