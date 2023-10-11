use core::ffi::CStr;

/// An extension trait for reading data from a slice of bytes.
///
/// Primarily uses the [Decode] trait to read data from the slice.
pub trait ByteSliceExt<'a> {
  fn remaining(&self, offset: &usize) -> &'a [u8];

  /// Reads a value of type `R` from the slice at the given offset and increments the offset.
  ///
  /// Returns `None` if the offset is out of bounds or if the value could not be read.
  fn read<R: FromBytes<'a>>(&self, offset: &mut usize) -> Option<R>;

  /// Reads a value of type `R` from the slice at the given offset with the given context and
  /// increments the offset.
  ///
  /// Returns `None` if the offset is out of bounds or if the value could not be read.
  fn read_with<R: FromBytes<'a, C>, C>(&self, offset: &mut usize, ctx: C) -> Option<R>;

  /// Peeks a value of type `R` from the slice at the given offset.
  ///
  /// Returns `None` if the offset is out of bounds or if the value could not be read.
  fn peek<R: FromBytes<'a>>(&self, offset: &usize) -> Option<R> {
    let mut offset = *offset;

    self.read(&mut offset)
  }

  /// Peeks a value of type `R` from the slice at the given offset with the given context.
  ///
  /// Returns `None` if the offset is out of bounds or if the value could not be peeked.
  fn peek_with<R: FromBytes<'a, C>, C>(&self, offset: &usize, ctx: C) -> Option<R> {
    let mut offset = *offset;

    self.read_with(&mut offset, ctx)
  }
}

impl<'a> ByteSliceExt<'a> for &'a [u8] {
  fn remaining(&self, offset: &usize) -> &'a [u8] {
    self.get(*offset..).unwrap_or_default()
  }

  fn read<F: FromBytes<'a>>(&self, offset: &mut usize) -> Option<F> {
    F::from_bytes(self, offset, ())
  }

  fn read_with<F: FromBytes<'a, C>, C>(&self, offset: &mut usize, ctx: C) -> Option<F> {
    F::from_bytes(self, offset, ctx)
  }
}

/// A trait that can be used to decode a value from a slice of bytes.
///
/// The `C` type parameter is a context that can be used to provide additional information to the
/// decoding process.
///
/// Decoding a value that could be erroneous outside of a lack of bytes should be avoided as the
/// return value `Option` doesn't provide much context as to why the value could not be read.
///
/// The reason behind using `Option` was to try to limit the amount of error types needed and to
/// generalize a `NotEnoughBytes` error into a `None` return value to allow the consumer to
/// accurately represent the error.
pub trait FromBytes<'a, C = ()>: Sized {
  /// Decodes a value of type `Self` from the given slice of bytes at the given offset with the
  /// given context.
  ///
  /// Returns `None` if the offset is out of bounds or if the value could not be read.
  fn from_bytes(buf: &'a [u8], offset: &mut usize, ctx: C) -> Option<Self>;
}

impl<'a> FromBytes<'a, usize> for &'a [u8] {
  #[inline]
  fn from_bytes(buf: &'a [u8], offset: &mut usize, len: usize) -> Option<Self> {
    let beg = *offset;
    let end = beg.saturating_add(len);
    let val = buf.get(beg..end)?;

    *offset = end;

    Some(val)
  }
}

impl<'a, const L: usize> FromBytes<'a> for [u8; L] {
  #[inline]
  fn from_bytes(buf: &'a [u8], offset: &mut usize, _: ()) -> Option<Self> {
    let beg = *offset;
    let end = beg.saturating_add(L);
    let val = buf.get(beg..end)?;
    if val.len() != L {
      return None;
    }

    let mut arr = [0u8; L];

    arr.copy_from_slice(val);
    *offset = end;

    Some(arr)
  }
}

/// A context for reading a `&CStr` that is nul-terminated.
#[derive(Clone, Copy)]
pub struct NulTerminated;

impl<'a> FromBytes<'a, NulTerminated> for &'a CStr {
  #[inline]
  fn from_bytes(buf: &'a [u8], offset: &mut usize, _: NulTerminated) -> Option<Self> {
    let rem = buf.get(*offset..)?;
    let val = CStr::from_bytes_until_nul(rem).ok()?;

    *offset = offset.saturating_add(val.to_bytes_with_nul().len());

    Some(val)
  }
}

/// A context for reading a `&CStr` that is prefixed with a `u32` length.
#[derive(Clone, Copy)]
pub struct LengthPrefixed;

impl<'a> FromBytes<'a, LengthPrefixed> for &'a CStr {
  #[inline]
  fn from_bytes(buf: &'a [u8], offset: &mut usize, _: LengthPrefixed) -> Option<Self> {
    let len = buf.read::<u32>(offset)? as usize;
    let beg = *offset;
    let val = buf.read_with::<&CStr, _>(offset, NulTerminated)?;

    *offset = beg.saturating_add(len);

    Some(val)
  }
}

/// A context for reading a `&CStr` that is padded to a 4-byte boundary.
#[derive(Clone, Copy)]
pub struct FourByteBoundaryPadded;

impl<'a> FromBytes<'a, FourByteBoundaryPadded> for &'a CStr {
  #[inline]
  fn from_bytes(buf: &'a [u8], offset: &mut usize, _: FourByteBoundaryPadded) -> Option<Self> {
    let cstr = buf.read_with::<&CStr, _>(offset, NulTerminated)?;

    let len = cstr.to_bytes_with_nul().len();
    let pad = (len.saturating_add(3)) & !3;
    let pad = pad.saturating_sub(len);

    *offset = offset.saturating_add(pad);

    Some(cstr)
  }
}

/// A context for reading compressed length values.
pub struct CompressedLength;

impl<'a> FromBytes<'a, CompressedLength> for usize {
  #[inline]
  fn from_bytes(buf: &'a [u8], offset: &mut usize, _: CompressedLength) -> Option<Self> {
    let rem = buf.get(*offset..)?;
    let val = *rem.first()? as usize;

    if val & 0x80 == 0 {
      *offset += 1;

      return Some(val);
    }

    if val & 0x40 == 0 {
      let val = val & 0x3f << 8 | *rem.get(1)? as usize;

      *offset += 2;

      return Some(val);
    }

    if val & 0x20 == 0 {
      let val = val & 0x1f << 24;
      let val = val | (*rem.get(1)? as usize) << 16;
      let val = val | (*rem.get(2)? as usize) << 8;
      let val = val | *rem.get(3)? as usize;

      *offset += 4;

      return Some(val);
    }

    None
  }
}

/// A trait that can be used to determine the number of bytes necessary to decode a value.
///
/// The `C` type parameter is a context that can be used to provide additional information to the
/// sizing process.
pub trait ByteSize<C = ()>: Sized {
  /// Returns the size of the type in bytes.
  fn byte_size(ctx: C) -> usize;
}

macro_rules! int {
  ($int:ident) => {
    impl<'a> FromBytes<'a, ()> for $int {
      #[inline]
      fn from_bytes(buf: &'a [u8], offset: &mut usize, _: ()) -> Option<Self> {
        Some($int::from_le_bytes(buf.read(offset)?))
      }
    }

    impl ByteSize<()> for $int {
      #[inline]
      fn byte_size(_: ()) -> usize {
        core::mem::size_of::<$int>()
      }
    }
  };
}

int!(i8);
int!(u8);
int!(i16);
int!(u16);
int!(i32);
int!(u32);
int!(i64);
int!(u64);

macro_rules! bitflags {
  (
    $(#[$outer:meta])*
    $vis:vis struct $BitFlags:ident: $T:ty {
        $(
            $(#[$inner:ident $($args:tt)*])*
            const $Flag:ident = $value:expr;
        )*
    }

    $($t:tt)*
  ) => {
    bitflags::bitflags! {
      $(#[$outer])*
      $vis struct $BitFlags: $T {
        $(
          $(#[$inner $($args)*])*
          const $Flag = $value;
        )*
      }
    }

    #[cfg(feature = "read")]
    impl<'a> $crate::bytes::FromBytes<'a, ()> for $BitFlags {
      #[inline]
      fn from_bytes(buf: &'a [u8], offset: &mut usize, _: ()) -> Option<Self> {
        use $crate::bytes::ByteSliceExt;

        Some(Self::from_bits_truncate(buf.read(offset)?))
      }
    }

    #[cfg(feature = "read")]
    impl $crate::bytes::ByteSize<()> for $BitFlags {
      #[inline]
      fn byte_size(_: ()) -> usize {
        core::mem::size_of::<$BitFlags>()
      }
    }
  };
}

pub(crate) use bitflags;

#[cfg(test)]
mod tests {
  use super::ByteSliceExt;
  use crate::bytes::NulTerminated;
  use core::ffi::CStr;

  #[test]
  fn test_read_u64() {
    let offset = &mut 0;
    let expected = 0xdeadbeefdeadbeefu64;
    let actual = (&expected.to_le_bytes()[..]).read::<u64>(offset).unwrap();

    assert_eq!(expected, actual);
    assert_eq!(8, *offset);
  }

  #[test]
  fn test_read_cstr_nul_terminated() {
    let offset = &mut 0;
    let expected = &b"hello world\0"[..];
    let actual = expected
      .read_with::<&CStr, _>(offset, NulTerminated)
      .unwrap();

    assert_eq!(expected, actual.to_bytes_with_nul());
    assert_eq!(expected.len(), *offset);
  }
}
