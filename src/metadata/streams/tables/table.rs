#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;
#[cfg(feature = "write")]
#[doc(inline)]
pub use write::*;

pub trait Row: Sized {}

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  use super::Row;
  use crate::metadata::errors::MetadataStreamReadError;
  use crate::metadata::headers::{HeapSizes, MetadataTablesHeader};
  use crate::metadata::streams::tables::id::RowId;
  use core::marker::PhantomData;

  /// A slice of bytes containing the data for a metadata table.
  #[repr(transparent)]
  pub struct TableBytes<'a, R> {
    row: PhantomData<R>,
    bytes: &'a [u8],
  }

  impl<'a, R: RowRead> TableBytes<'a, R> {
    /// Creates a new [TableReader] from the given bytes and [MetadataTablesHeader].
    pub fn from_bytes(
      bytes: &'a [u8],
      offset: &mut usize,
      header: &MetadataTablesHeader,
    ) -> Result<Self, MetadataStreamReadError<'a>> {
      let len = R::table_len(header);
      let size = R::row_size(header);
      let size = len.saturating_mul(size);
      let bytes = bytes
        .get(*offset..*offset + size)
        .ok_or(MetadataStreamReadError::NotEnough)?;

      *offset += size;

      Ok(Self {
        row: PhantomData,
        bytes,
      })
    }

    /// Creates a new [TableReader] from the given bytes and [MetadataTablesHeader].
    pub fn reader<'h>(&self, header: &'h MetadataTablesHeader) -> TableReader<'a, 'h, R> {
      TableReader {
        row: PhantomData,
        bytes: self.bytes,
        header,
      }
    }
  }

  impl<'a, R> Default for TableBytes<'a, R> {
    fn default() -> Self {
      Self {
        row: PhantomData,
        bytes: &[],
      }
    }
  }

  impl<'a, R> Clone for TableBytes<'a, R> {
    fn clone(&self) -> Self {
      *self
    }
  }

  impl<'a, R> Copy for TableBytes<'a, R> {}

  /// Reads rows from a metadata table.
  pub struct TableReader<'a, 'h, R> {
    row: PhantomData<R>,
    bytes: &'a [u8],
    header: &'h MetadataTablesHeader,
  }

  impl<'a, 'h, R: RowRead> TableReader<'a, 'h, R> {
    /// Gets the row from the given [RowId].
    pub fn get(&self, id: RowId<R>) -> Option<R> {
      let mut offset = id.index() * R::row_size(self.header);

      R::from_bytes(self.bytes, &mut offset, id, self.header)
    }
  }

  impl<'a, 'h, R> Clone for TableReader<'a, 'h, R> {
    fn clone(&self) -> Self {
      *self
    }
  }

  impl<'a, 'h, R> Copy for TableReader<'a, 'h, R> {}

  impl<'a, 'h, R: RowRead> IntoIterator for TableReader<'a, 'h, R> {
    type Item = R;
    type IntoIter = TableReaderIter<'a, 'h, R>;

    fn into_iter(self) -> Self::IntoIter {
      TableReaderIter {
        row: self.row,
        id: RowId::new(0),
        bytes: self.bytes,
        header: self.header,
      }
    }
  }

  /// Iterates over rows in a metadata table.
  pub struct TableReaderIter<'a, 'h, R> {
    row: PhantomData<R>,
    id: RowId<R>,
    bytes: &'a [u8],
    header: &'h MetadataTablesHeader,
  }

  impl<'a, 'h, R: RowRead> Iterator for TableReaderIter<'a, 'h, R> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
      let mut offset = self.id.index() * R::row_size(self.header);
      let row = R::from_bytes(self.bytes, &mut offset, self.id, self.header)?;

      self.id = self.id.next();

      Some(row)
    }
  }

  pub trait RowRead: Row {
    /// Returns the size of a row in bytes using the given [MetadataTablesHeader].
    fn row_size(header: &MetadataTablesHeader) -> usize;
    /// Returns the number of rows in the table using the given [MetadataTablesHeader].
    fn table_len(header: &MetadataTablesHeader) -> usize;
    /// Reads the row with the given [RowId] from the given buffer and [MetadataTablesHeader].
    fn from_bytes(
      buf: &[u8],
      offset: &mut usize,
      id: RowId<Self>,
      header: &MetadataTablesHeader,
    ) -> Option<Self>;
  }

  impl From<&MetadataTablesHeader> for HeapSizes {
    fn from(header: &MetadataTablesHeader) -> HeapSizes {
      header.heap_sizes
    }
  }

  impl From<&MetadataTablesHeader> for () {
    fn from(_: &MetadataTablesHeader) {}
  }
}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {}

/// Defines a metadata table row.
macro_rules! row {
  (
    $(#[$attr:meta])*
    struct $name:ident : $id:literal {
      $(
        $(#[$field_attr:meta])*
        $field:ident: $field_ty:ty,
      )+
    }
  ) => {
    $(#[$attr])*
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct $name {
      id: RowId<Self>,
      $($field: $field_ty,)+
    }

    impl $name {
      /// The numerical sequence of the table in the metadata stream.
      pub const ID: usize = $id;

      pub fn id(self) -> RowId<Self> {
        self.id
      }

      $(
        $(#[$field_attr])*
        pub fn $field(self) -> $field_ty {
          self.$field
        }
      )+
    }

    impl Row for $name {}

    #[cfg(feature = "read")]
    impl $crate::metadata::streams::tables::table::RowRead for $name {
      fn row_size(header: &$crate::metadata::headers::MetadataTablesHeader) -> usize {
        use $crate::bytes::ByteSize;

        let mut size = 0usize;

        $(
          size = size.saturating_add(<$field_ty>::byte_size(header.into()));
        )+

        size
      }

      #[inline]
      fn table_len(header: &$crate::metadata::headers::MetadataTablesHeader) -> usize {
        header.rows[$id] as _
      }

      fn from_bytes(
        buf: &[u8],
        offset: &mut usize,
        id: RowId<Self>,
        header: &$crate::metadata::headers::MetadataTablesHeader,
      ) -> Option<Self> {
        use $crate::bytes::FromBytes;

        Some(Self {
          id,
          $(
            $field: <$field_ty>::from_bytes(buf, offset, header.into())?,
          )+
        })
      }
    }
  };
}

pub(crate) use row;
