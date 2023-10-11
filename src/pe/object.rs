//! PE utilities for the [object] crate.

#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;
#[cfg(feature = "write")]
#[doc(inline)]
pub use write::*;

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  use crate::bytes::ByteSliceExt;
  use crate::metadata::errors::MetadataReadError;
  use crate::metadata::MetadataReader;
  use crate::pe::errors::ReadManagedPeError;
  use crate::pe::headers::CliHeader;
  use object::pe::{ImageNtHeaders32, ImageNtHeaders64};
  use object::read::pe::{ImageNtHeaders, PeFile};

  pub type ManagedPeFile32<'a> = ManagedPeFile<'a, ImageNtHeaders32>;
  pub type ManagedPeFile64<'a> = ManagedPeFile<'a, ImageNtHeaders64>;

  /// A PE object file with a CIL metadata data directory.
  pub struct ManagedPeFile<'a, Pe>
  where
    Pe: ImageNtHeaders,
  {
    pe: PeFile<'a, Pe, &'a [u8]>,
    header: CliHeader,
  }

  impl<'a, Pe> ManagedPeFile<'a, Pe>
  where
    Pe: ImageNtHeaders,
  {
    /// Returns the [ManagedPeFile] from the given data.
    pub fn from_data(data: &'a [u8]) -> Result<Self, ReadManagedPeError> {
      Self::from_pe(PeFile::parse(data)?)
    }

    /// Returns the [ManagedPeFile] from the given [PeFile].
    pub fn from_pe(pe: PeFile<'a, Pe, &'a [u8]>) -> Result<Self, ReadManagedPeError> {
      let directory = pe
        .data_directories()
        .get(14)
        .ok_or(ReadManagedPeError::MissingCliHeader)?;

      let data = directory
        .data(pe.data(), &pe.section_table())
        .map_err(|_| ReadManagedPeError::MissingCliHeader)?;

      let header = data
        .read::<CliHeader>(&mut 0)
        .ok_or(ReadManagedPeError::MissingCliHeader)?;

      Ok(Self { pe, header })
    }

    /// Returns the [MetadataReader] for this PE file.
    pub fn metadata(&self) -> Result<MetadataReader, MetadataReadError> {
      let metadata = self.header.metadata;
      let data = self
        .pe
        .section_table()
        .pe_data_at(self.pe.data(), metadata.virtual_address)
        .ok_or(MetadataReadError::NotEnough)?
        .get(..metadata.size as _)
        .ok_or(MetadataReadError::NotEnough)?;

      MetadataReader::from_bytes(data)
    }
  }
}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {}
