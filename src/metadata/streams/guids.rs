//! The `#GUID` metadata stream.

#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;
#[cfg(feature = "write")]
#[doc(inline)]
pub use write::*;

/// A handle to a guid in the `#GUID` metadata stream.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GuidId(usize);

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  use super::GuidId;
  use crate::bytes::{ByteSize, ByteSliceExt, FromBytes};
  use crate::metadata::headers::HeapSizes;
  use core::fmt;

  /// A `#GUID` heap.
  ///
  /// Contains 16-byte GUIDs at offsets relative to the start of the heap.  The heap can contain
  /// unreachable garbage data.
  #[repr(transparent)]
  #[derive(Default, Clone, Copy)]
  pub struct GuidsHeap<'a>(pub(crate) &'a [u8]);

  impl<'a> GuidsHeap<'a> {
    /// Returns the guid from the given [GuidId].
    ///
    /// Will return `None` if the given id is out of bounds.
    pub fn get(&self, id: GuidId) -> Option<[u8; 16]> {
      let beg = id.0;
      let end = beg + 16;

      self.0.get(beg..end).and_then(|b| b.try_into().ok())
    }
  }

  impl fmt::Debug for GuidsHeap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_tuple("GuidsHeap").finish()
    }
  }

  impl FromBytes<'_, HeapSizes> for GuidId {
    #[inline]
    fn from_bytes(buf: &[u8], offset: &mut usize, heap_sizes: HeapSizes) -> Option<Self> {
      Some(Self(match Self::byte_size(heap_sizes) {
        4 => buf.read::<u32>(offset)? as _,
        2 => buf.read::<u16>(offset)? as _,
        _ => unreachable!(),
      }))
    }
  }

  impl ByteSize<HeapSizes> for GuidId {
    #[inline]
    fn byte_size(heap_sizes: HeapSizes) -> usize {
      match heap_sizes.contains(HeapSizes::WIDE_GUID_HEAP) {
        true => 4,
        false => 2,
      }
    }
  }
}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {
  // TODO: implement [GuidsHeapBuilder].
}
