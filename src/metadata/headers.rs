//! Support types for the ECMA-335 metadata format.

#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;
#[cfg(feature = "write")]
#[doc(inline)]
pub use write::*;

/// The magic signature for the CLI metadata header.
pub const METADATA_MAGIC: u32 = 0x424A5342;

/// Contains the table stream header information.
#[derive(Copy, Clone, Debug)]
pub struct MetadataTablesHeader {
  // Reserved, always 0.
  pub _reserved_0: u32,
  /// Major version of table schemata; shall be 2.
  pub major_version: u8,
  /// Minor version of table schemata; shall be 0.
  pub minor_version: u8,
  /// Bit vector for heap sizes.
  pub heap_sizes: HeapSizes,
  /// Reserved, always 1.
  pub _reserved_1: u8,
  /// Bit vector of present tables, let n be the number of bits that are 1.
  pub valid: u64,
  /// Bit vector of sorted tables.
  pub sorted: u64,
  /// The array containing the values representing the number of rows in a table, indexed by the id
  /// of the table.
  pub rows: [u32; 64],
}

bitflags::bitflags! {
  /// The bit flags indicating which heaps should have 4 bit wide indexes or 2 bit wide indexes.
  #[derive(Default, Copy, Clone, Debug)]
  pub struct HeapSizes : u8 {
    /// If set indicates the `#Strings` heap index should be `4` bytes wide, otherwise `2`.
    const WIDE_STRING_HEAP = 0x01;
    /// If set indicates the `#GUID` heap index should be `4` bytes wide, otherwise `2`.
    const WIDE_GUID_HEAP = 0x02;
    /// If set indicates the `#Blob` heap index should be `4` bytes wide, otherwise `2`.
    const WIDE_BLOB_HEAP = 0x04;
  }
}

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  use super::{HeapSizes, MetadataTablesHeader};
  use crate::bytes::{ByteSliceExt, FourByteBoundaryPadded, FromBytes, LengthPrefixed};
  use core::ffi::CStr;

  /// The [MetadataHeader] contains information about the root of the physical metadata.
  #[derive(Debug)]
  pub struct MetadataHeader<'a> {
    /// The magic signature for the metadata header. This should always be `0x424A5342`.
    pub signature: u32,
    /// The major version of the metadata header. This should always be `1`.
    pub major_version: u16,
    /// The minor version of the metadata header. This should always be `1`.
    pub minor_version: u16,
    /// The reserved bytes. This should always be `0`.
    pub reserved: u32,
    /// The version string.
    pub version: &'a CStr,
    /// The metadata flags.  This should always be `0`.
    pub flags: u16,
    /// The number of streams in the metadata.
    pub streams: u16,
  }

  impl<'a> FromBytes<'a> for MetadataHeader<'a> {
    /// Creates the [MetadataHeader] from the given metadata bytes and advances the offset.
    fn from_bytes(buf: &'a [u8], offset: &mut usize, _: ()) -> Option<Self> {
      Some(Self {
        signature: buf.read(offset)?,
        major_version: buf.read(offset)?,
        minor_version: buf.read(offset)?,
        reserved: buf.read(offset)?,
        version: buf.read_with(offset, LengthPrefixed)?,
        flags: buf.read(offset)?,
        streams: buf.read(offset)?,
      })
    }
  }

  /// The [MetadataStreamHeader] contains information about a single stream in the metadata.
  #[derive(Debug)]
  pub struct MetadataStreamHeader<'a> {
    /// The offset to the start of the stream data from the start of the metadata root.
    pub offset: u32,
    /// The size of the stream data in bytes.
    pub size: u32,
    /// The name of the stream.
    pub name: &'a CStr,
  }

  impl<'a> MetadataStreamHeader<'a> {
    /// Gets the data for this stream from the given metadata bytes.
    pub fn data(&self, metadata: &'a [u8]) -> Option<&'a [u8]> {
      let beg = self.offset as usize;
      let end = beg.saturating_add(self.size as usize);

      metadata.get(beg..end)
    }
  }

  impl<'a> FromBytes<'a> for MetadataStreamHeader<'a> {
    fn from_bytes(buf: &'a [u8], offset: &mut usize, _: ()) -> Option<Self> {
      Some(Self {
        offset: buf.read(offset)?,
        size: buf.read(offset)?,
        name: buf.read_with(offset, FourByteBoundaryPadded)?,
      })
    }
  }

  impl<'a> FromBytes<'a> for [u32; 64] {
    fn from_bytes(buf: &'a [u8], offset: &mut usize, _: ()) -> Option<Self> {
      let mut arr = [0; 64];
      let mut i = 0;

      while i < arr.len() {
        let val = buf.read::<u32>(offset)?;

        if val == 0 {
          break;
        }

        arr[i] = val;
        i += 1;
      }

      Some(arr)
    }
  }

  impl FromBytes<'_> for MetadataTablesHeader {
    fn from_bytes(buf: &[u8], offset: &mut usize, _: ()) -> Option<Self> {
      let _reserved_0 = buf.read(offset)?;
      let major_version = buf.read(offset)?;
      let minor_version = buf.read(offset)?;
      let heap_sizes = buf.read(offset)?;
      let _reserved_1 = buf.read(offset)?;
      let valid = buf.read(offset)?;
      let sorted = buf.read(offset)?;
      let mut rows = [0; 64];

      for (i, row) in rows.iter_mut().enumerate() {
        if valid & (1 << i) != 0 {
          *row = buf.read(offset)?;
        }
      }

      Some(Self {
        _reserved_0,
        major_version,
        minor_version,
        heap_sizes,
        _reserved_1,
        valid,
        sorted,
        rows,
      })
    }
  }

  impl FromBytes<'_> for HeapSizes {
    fn from_bytes(buf: &[u8], offset: &mut usize, _: ()) -> Option<Self> {
      Some(Self::from_bits_truncate(buf.read::<u8>(offset)?))
    }
  }
}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {}
