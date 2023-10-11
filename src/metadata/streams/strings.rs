//! The `#Strings` metadata stream.

#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;
#[cfg(feature = "write")]
#[doc(inline)]
pub use write::*;

/// A handle to a string in the `#Strings` metadata stream.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringId(usize);

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  use super::StringId;
  use crate::bytes::{ByteSize, ByteSliceExt, FromBytes};
  use crate::metadata::headers::HeapSizes;
  use core::ffi::CStr;
  use core::fmt;

  /// A `#Strings` heap.
  ///
  /// Contains utf8 encoded, nul-terminated strings at offsets relative to the start of the heap.  
  /// The heap can contain garbage data provided it is not part of content reachable from any of the
  /// tables.
  #[repr(transparent)]
  #[derive(Default, Clone, Copy)]
  pub struct StringsHeap<'a>(pub(crate) &'a [u8]);

  impl<'a> StringsHeap<'a> {
    /// Returns the string for the given [StringId].
    ///
    /// Will return `None` if the given id is out of bounds.  Performs a scan for a `nul` byte to
    /// determine the length of the string.
    pub fn get(&self, id: StringId) -> Option<&'a CStr> {
      CStr::from_bytes_until_nul(self.0.get(id.0..)?).ok()
    }
  }

  impl<'a> IntoIterator for StringsHeap<'a> {
    type Item = &'a CStr;
    type IntoIter = StringsHeapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
      StringsHeapIter {
        data: self.0,
        index: 0,
      }
    }
  }

  impl fmt::Debug for StringsHeap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_tuple("StringsHeap").finish()
    }
  }

  /// An iterator over strings in the [StringsHeap].
  ///
  /// In reality this method for iterating strings in the heap isn't entirely accurate as garbage
  /// data can be present in the heap.  However, this iterator is useful for testing purposes.
  #[derive(Clone, Copy)]
  pub struct StringsHeapIter<'a> {
    data: &'a [u8],
    index: usize,
  }

  impl<'a> Iterator for StringsHeapIter<'a> {
    type Item = &'a CStr;

    fn next(&mut self) -> Option<Self::Item> {
      let data = self.data.get(self.index..)?;
      let next = CStr::from_bytes_until_nul(data).ok()?;

      self.index += next.to_bytes_with_nul().len();

      Some(next)
    }
  }

  impl FromBytes<'_, HeapSizes> for StringId {
    #[inline]
    fn from_bytes(buf: &[u8], offset: &mut usize, heap_sizes: HeapSizes) -> Option<Self> {
      Some(Self(match Self::byte_size(heap_sizes) {
        4 => buf.read::<u32>(offset)? as _,
        2 => buf.read::<u16>(offset)? as _,
        _ => unreachable!(),
      }))
    }
  }

  impl ByteSize<HeapSizes> for StringId {
    #[inline]
    fn byte_size(heap_sizes: HeapSizes) -> usize {
      match heap_sizes.contains(HeapSizes::WIDE_STRING_HEAP) {
        true => 4,
        false => 2,
      }
    }
  }
}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {
  // TODO: implement [StringHeapBuilder].
}
