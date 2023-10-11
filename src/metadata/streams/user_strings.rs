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
pub struct UserStringId(usize);

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  use super::UserStringId;
  use crate::bytes::{ByteSliceExt, CompressedLength};
  use core::fmt;

  /// A `#US` heap.
  ///
  /// Contains UTF-16 strings prefixed with a variable length integer indicating the length of the
  /// string.
  #[repr(transparent)]
  #[derive(Default, Clone, Copy)]
  pub struct UserStringsHeap<'a>(pub(crate) &'a [u8]);

  impl<'a> UserStringsHeap<'a> {
    /// Returns the UTF-16 encoded string from the given [UserStringId].
    ///
    /// Will return `None` if the given id is out of bounds.
    pub fn get(&self, id: UserStringId) -> Option<&'a [u8]> {
      let mut offset = id.0;

      let len = self.0.read_with(&mut offset, CompressedLength)?;
      let data = self.0.read_with(&mut offset, len)?;

      Some(data)
    }
  }

  impl fmt::Debug for UserStringsHeap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_tuple("UserStringsHeap").finish()
    }
  }
}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {
  // TODO: implement [UserStringsHeapBuilder].
}
