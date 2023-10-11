pub mod blobs;
pub mod guids;
pub mod strings;
pub mod tables;
pub mod user_strings;

#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;
#[cfg(feature = "write")]
#[doc(inline)]
pub use write::*;

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  use super::blobs::BlobsHeap;
  use super::guids::GuidsHeap;
  use super::strings::StringsHeap;
  use super::tables::TablesStream;
  use super::user_strings::UserStringsHeap;
  use crate::bytes::ByteSliceExt;
  use crate::metadata::errors::MetadataStreamReadError;
  use crate::metadata::headers::MetadataStreamHeader;

  /// Iterates over the metadata streams.
  pub struct MetadataStreamIter<'a> {
    len: usize,
    bytes: &'a [u8],
    offset: usize,
  }

  impl<'a> MetadataStreamIter<'a> {
    pub(crate) fn new(len: usize, offset: usize, bytes: &'a [u8]) -> Self {
      Self { len, bytes, offset }
    }
  }

  impl<'a> Iterator for MetadataStreamIter<'a> {
    type Item = Result<MetadataStream<'a>, MetadataStreamReadError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
      if self.len == 0 {
        return None;
      }

      self.len -= 1;

      let header = self.bytes.read::<MetadataStreamHeader>(&mut self.offset)?;
      let data = match header.data(self.bytes) {
        Some(data) => data,
        None => return Some(Err(MetadataStreamReadError::MissingData { header })),
      };

      Some(Ok(match header.name.to_bytes() {
        b"#~" => MetadataStream::Tables(match TablesStream::from_bytes(data) {
          Ok(tables) => tables,
          Err(err) => return Some(Err(err)),
        }),
        b"#US" => MetadataStream::UserStrings(UserStringsHeap(data)),
        b"#Blob" => MetadataStream::Blobs(BlobsHeap(data)),
        b"#GUID" => MetadataStream::Guids(GuidsHeap(data)),
        b"#Strings" => MetadataStream::Strings(StringsHeap(data)),
        _ => MetadataStream::Unrecognized { header, data },
      }))
    }
  }

  /// A ECMA-335 metadata stream.
  #[derive(Debug)]
  #[non_exhaustive]
  pub enum MetadataStream<'a> {
    /// The `#Blob` metadata stream.
    Blobs(BlobsHeap<'a>),
    /// The `#GUID` metadata stream.
    Guids(GuidsHeap<'a>),
    /// The `#~` metadata stream.
    Tables(TablesStream<'a>),
    /// The `#Strings` metadata stream.
    Strings(StringsHeap<'a>),
    /// The `#US` metadata stream.
    UserStrings(UserStringsHeap<'a>),
    /// The metadata stream name was not recognized.
    Unrecognized {
      /// The metadata stream header.
      header: MetadataStreamHeader<'a>,
      /// The contents of the stream.
      data: &'a [u8],
    },
  }

  impl<'a> MetadataStream<'a> {
    /// Returns the [BlobsHeap] if this is the `#Blob` metadata stream.
    pub fn as_blobs(&self) -> Option<BlobsHeap<'a>> {
      match self {
        Self::Blobs(blobs) => Some(*blobs),
        _ => None,
      }
    }

    /// Returns the [GuidsHeap] if this is the `#GUID` metadata stream.
    pub fn as_guids(&self) -> Option<GuidsHeap<'a>> {
      match self {
        Self::Guids(guids) => Some(*guids),
        _ => None,
      }
    }

    /// Returns the [StringsHeap] if this is the `#Strings` metadata stream.
    pub fn as_strings(&self) -> Option<StringsHeap<'a>> {
      match self {
        Self::Strings(strings) => Some(*strings),
        _ => None,
      }
    }

    /// Returns the [TablesStream] if this the `#~` metadata stream.
    pub fn as_tables(&self) -> Option<TablesStream<'a>> {
      match self {
        Self::Tables(tables) => Some(*tables),
        _ => None,
      }
    }

    /// Returns the [UserStringsHeap] if this is the `#US` metadata stream.
    pub fn as_user_strings(&self) -> Option<UserStringsHeap<'a>> {
      match self {
        Self::UserStrings(user_strings) => Some(*user_strings),
        _ => None,
      }
    }
  }
}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {}
