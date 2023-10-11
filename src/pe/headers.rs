#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;
#[cfg(feature = "write")]
#[doc(inline)]
pub use write::*;

/// The [CliHeader] contains all of the runtime-specific data entries and other information. The
/// header should be placed in a read-only, sharable section of the image.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CliHeader {
  /// The size of the header, currently 72.
  pub cb: u32,
  /// The minimum version of the runtime required to run this program, currently 2.
  pub major_runtime_version: u16,
  /// The minor portion of the version, currently 0.
  pub minor_runtime_version: u16,
  /// RVA and size of the physical metadata.
  pub metadata: DataDirectory,
  /// Flags describing this runtime image.
  pub flags: CliRuntimeFlags,
  /// Token for the MethodDef or File of the entry point for the image.
  pub entry_point_token: u32,
  /// RVA and size of implementation-specific resources.
  pub resources: DataDirectory,
  /// RVA of the hash data for this PE file used by the CLI loader for binding and versioning.
  pub strong_name_signature: DataDirectory,
  /// Always 0.
  pub code_manager_table: DataDirectory,
  /// RVA of an array of locations in the file that contain an array of function pointers (e.g., vtable slots).
  pub vtable_fixups: DataDirectory,
  /// Always 0.
  pub export_address_table_jumps: DataDirectory,
  /// Always 0.
  pub managed_native_header: DataDirectory,
}

/// Represents a data directory.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DataDirectory {
  /// The relative virtual address of the table.
  pub virtual_address: u32,
  /// The size of the table, in bytes.
  pub size: u32,
}

bitflags::bitflags! {
  /// The following [CliRuntimeFlags] describe this runtime image and are used by the loader. All
  /// unspecified bits should be zero.
  #[derive(Copy, Clone)]
  pub struct CliRuntimeFlags : u32 {
    const COMIMAGE_FLAGS_ILONLY = 0x00000001;
    /// Image can only be loaded into a 32-bit process, for instance if there are 32-bit vtable
    /// fixups, or casts from native integers to int32. CLI implementations that have 64-bit native
    /// integers shall refuse loading binaries with this flag set.
    const COMIMAGE_FLAGS_32BITREQUIRED = 0x00000002;
    /// Image has a strong name signature.
    const COMIMAGE_FLAGS_STRONGNAMESIGNED = 0x00000008;
    const COMIMAGE_FLAGS_NATIVE_ENTRYPOINT = 0x00000010;
    const COMIMAGE_FLAGS_TRACKDEBUGDATA = 0x00010000;
  }
}

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  use super::{CliHeader, CliRuntimeFlags, DataDirectory};
  use crate::bytes::{ByteSliceExt, FromBytes};

  impl<'a> FromBytes<'a> for CliHeader {
    fn from_bytes(buf: &'a [u8], offset: &mut usize, _: ()) -> Option<Self> {
      Some(Self {
        cb: buf.read(offset)?,
        major_runtime_version: buf.read(offset)?,
        minor_runtime_version: buf.read(offset)?,
        metadata: buf.read(offset)?,
        flags: buf.read(offset)?,
        entry_point_token: buf.read(offset)?,
        resources: buf.read(offset)?,
        strong_name_signature: buf.read(offset)?,
        code_manager_table: buf.read(offset)?,
        vtable_fixups: buf.read(offset)?,
        export_address_table_jumps: buf.read(offset)?,
        managed_native_header: buf.read(offset)?,
      })
    }
  }

  impl<'a> FromBytes<'a> for DataDirectory {
    fn from_bytes(buf: &'a [u8], offset: &mut usize, _: ()) -> Option<Self> {
      Some(Self {
        virtual_address: buf.read(offset)?,
        size: buf.read(offset)?,
      })
    }
  }

  impl<'a> FromBytes<'a> for CliRuntimeFlags {
    fn from_bytes(buf: &'a [u8], offset: &mut usize, _: ()) -> Option<Self> {
      Some(Self::from_bits_truncate(buf.read(offset)?))
    }
  }
}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {}
