//! The `#Blob` metadata stream.

#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;
#[cfg(feature = "write")]
#[doc(inline)]
pub use write::*;

/// A handle to a blob of bytes in the `#Blob` metadata stream.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlobId(usize);

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  use super::BlobId;
  use crate::bytes::{ByteSize, ByteSliceExt, CompressedLength, FromBytes};
  use crate::metadata::headers::HeapSizes;
  use core::fmt;

  /// The `#Blob` heap.
  ///
  /// Contains blobs of bytes prefixed with a variable length integer indicating the length of the
  /// blob.
  #[repr(transparent)]
  #[derive(Default, Clone, Copy)]
  pub struct BlobsHeap<'a>(pub(crate) &'a [u8]);

  impl<'a> BlobsHeap<'a> {
    /// Returns the guid at the given [BlobId].
    ///
    /// Will return `None` if the given id is out of bounds.
    pub fn get(&self, id: BlobId) -> Option<&'a [u8]> {
      let mut offset = id.0;

      let len = self.0.read_with(&mut offset, CompressedLength)?;
      let data = self.0.read_with(&mut offset, len)?;

      Some(data)
    }
  }

  impl fmt::Debug for BlobsHeap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_tuple("BlobsHeap").finish()
    }
  }

  impl FromBytes<'_, HeapSizes> for BlobId {
    #[inline]
    fn from_bytes(buf: &[u8], offset: &mut usize, heap_sizes: HeapSizes) -> Option<Self> {
      Some(Self(match Self::byte_size(heap_sizes) {
        4 => buf.read::<u32>(offset)? as _,
        2 => buf.read::<u16>(offset)? as _,
        _ => unreachable!(),
      }))
    }
  }

  impl ByteSize<HeapSizes> for BlobId {
    #[inline]
    fn byte_size(heap_sizes: HeapSizes) -> usize {
      match heap_sizes.contains(HeapSizes::WIDE_BLOB_HEAP) {
        true => 4,
        false => 2,
      }
    }
  }
}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {
  // TODO: implement [BlobsHeapBuilder].
}
