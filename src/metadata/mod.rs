//! ECMA-335 metadata physical layout.

pub mod errors;
pub mod headers;
pub mod streams;

#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;
#[cfg(feature = "write")]
#[doc(inline)]
pub use write::*;

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  use super::errors::MetadataReadError;
  use super::headers::MetadataHeader;
  use super::streams::MetadataStreamIter;
  use crate::bytes::ByteSliceExt;
  use crate::metadata::headers::METADATA_MAGIC;

  /// A ECMA-335 metadata reader.
  pub struct MetadataReader<'a> {
    bytes: &'a [u8],
    header: MetadataHeader<'a>,
    streams: usize,
  }

  impl<'a> MetadataReader<'a> {
    /// Creates a new [MetadataReader] from the given bytes.
    ///
    /// Attempts to parse and verify the [MetadataHeader] from the given bytes before returning the
    /// reader.
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, MetadataReadError> {
      let offset = &mut 0;
      let header = bytes
        .read::<MetadataHeader>(offset)
        .ok_or(MetadataReadError::NotEnough)?;

      if header.signature != METADATA_MAGIC {
        return Err(MetadataReadError::BadSignature(header.signature));
      }

      Ok(Self {
        bytes,
        header,
        streams: *offset,
      })
    }

    /// Gets the [MetadataHeader].
    pub const fn header(&self) -> &MetadataHeader<'a> {
      &self.header
    }

    /// Gets the [MetadataStreamIter].
    pub fn streams(&self) -> MetadataStreamIter<'a> {
      MetadataStreamIter::new(self.header.streams as _, self.streams, self.bytes)
    }
  }
}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {}
