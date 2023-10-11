#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  use crate::metadata::headers::MetadataStreamHeader;
  use core::fmt;

  /// An error that occurred while reading the root metadata.
  #[derive(Debug)]
  pub enum MetadataReadError {
    /// The bytes supplied do not contain enough data to read the metadata header.
    NotEnough,
    /// The metadata signature was not `0x424A5342`.
    BadSignature(u32),
  }

  impl fmt::Display for MetadataReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        Self::NotEnough => write!(f, "Not enough bytes remaining"),
        Self::BadSignature(signature) => {
          write!(f, "Expected signature `0x424A5342`, got `{signature}`")
        }
      }
    }
  }

  #[cfg(any(feature = "std", test))]
  impl std::error::Error for MetadataReadError {}

  /// An error that occurred while reading a metadata stream.
  #[derive(Debug)]
  pub enum MetadataStreamReadError<'a> {
    /// Not enough bytes remaining to read the stream.
    NotEnough,
    /// The metadata stream header points to data outside the metadata.
    MissingData {
      /// The metadata stream header.
      header: MetadataStreamHeader<'a>,
    },
  }

  impl<'a> fmt::Display for MetadataStreamReadError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        Self::NotEnough => write!(f, "Not enough bytes remaining"),
        Self::MissingData { header } => write!(
          f,
          "The metadata stream header points to data outside the metadata: {:?}",
          header
        ),
      }
    }
  }

  #[cfg(any(feature = "std", test))]
  impl<'a> std::error::Error for MetadataStreamReadError<'a> {}
}
