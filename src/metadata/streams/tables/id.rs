use super::rows::*;
use core::fmt;
use core::marker::PhantomData;
#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;
#[cfg(feature = "write")]
#[doc(inline)]
pub use write::*;

/// A handle to a row in a table with the given row type.
#[repr(transparent)]
pub struct RowId<R> {
  row: PhantomData<R>,
  index: usize,
}

impl<R> RowId<R> {
  /// Creates a new [RowId] with the given index.
  #[inline]
  pub(crate) fn new(index: usize) -> Self {
    Self {
      row: PhantomData,
      index,
    }
  }

  /// Returns the index of the row.
  #[inline]
  pub fn index(self) -> usize {
    self.index
  }

  /// Returns the next row id.
  #[inline]
  pub fn next(self) -> Self {
    Self::new(self.index.saturating_add(1))
  }
}

impl<R> Clone for RowId<R> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<R> Copy for RowId<R> {}

impl<R> fmt::Debug for RowId<R> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("RowId").field(&self.index).finish()
  }
}

impl<R> PartialEq for RowId<R> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.index == other.index
  }
}

impl<R> Eq for RowId<R> {}

/// Defines a metadata coded id type.
macro_rules! coded_id {
  (
    $(#[$attr:meta])*
    enum $name:ident : $bits:literal {
      $(
        $(#[$variant_attr:meta])*
        $variant:ident($table:ident) = $tag:literal
      ),* $(,)?
    }
  ) => {
    $(#[$attr])*
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum $name {
      $(
        $(#[$variant_attr])*
        $variant(RowId<$table>)
      ),*
    }

    impl $name {
      /// Creates a coded id from the given coded id value and [MetadataTablesHeader].
      ///
      /// Returns `None` if the index is out of bounds.
      #[inline]
      pub fn from_tag(val: usize) -> Option<Self> {
        let tag = val & ((1 << $bits) - 1);
        let index = val >> $bits;

        match tag {
          $(
            $tag => Some(Self::$variant(RowId::new(index))),
          )*
          _ => None
        }
      }
    }

    #[cfg(feature = "read")]
    impl $crate::bytes::FromBytes<'_, &$crate::metadata::headers::MetadataTablesHeader> for $name {
      fn from_bytes(buf: &[u8], offset: &mut usize, header: &$crate::metadata::headers::MetadataTablesHeader) -> Option<Self> {
        use $crate::bytes::{ByteSize, ByteSliceExt};

        let tag = match Self::byte_size(header) {
          4 => buf.read::<u32>(offset)? as usize,
          2 => buf.read::<u16>(offset)? as usize,
          _ => unreachable!(),
        };

        Self::from_tag(tag)
      }
    }

    #[cfg(feature = "read")]
    impl $crate::bytes::ByteSize<&$crate::metadata::headers::MetadataTablesHeader> for $name {
      fn byte_size(header: &$crate::metadata::headers::MetadataTablesHeader) -> usize {
        use $crate::metadata::streams::tables::table::RowRead;

        $(
          if $table::table_len(header) as u32 >= (1u32 << (16 - $bits)) {
            return 4;
          }
        )+

        2
      }
    }
  };
}

coded_id! {
  enum TypeDefOrRef : 2 {
    TypeDef(TypeDefRow) = 0,
    TypeRef(TypeRefRow) = 1,
    TypeSpec(TypeSpecRow) = 2
  }
}

coded_id! {
  enum HasConstant : 2 {
    Field(FieldRow) = 0,
    Param(ParamRow) = 1,
    Property(PropertyRow) = 2
  }
}

coded_id! {
  enum HasCustomAttribute : 5 {
    MethodDef(MethodDefRow) = 0,
    Field(FieldRow) = 1,
    TypeRef(TypeRefRow) = 2,
    TypeDef(TypeDefRow) = 3,
    Param(ParamRow) = 4,
    InterfaceImpl(InterfaceImplRow) = 5,
    MemberRef(MemberRefRow) = 6,
    Module(ModuleRow) = 7,
    DeclSecurity(DeclSecurityRow) = 8,
    Property(PropertyRow) = 9,
    Event(EventRow) = 10,
    StandAloneSig(StandAloneSigRow) = 11,
    ModuleRef(ModuleRefRow) = 12,
    TypeSpec(TypeSpecRow) = 13,
    Assembly(AssemblyRow) = 14,
    AssemblyRef(AssemblyRefRow) = 15,
    File(FileRow) = 16,
    ExportedType(ExportedTypeRow) = 17,
    ManifestResource(ManifestResourceRow) = 18,
    GenericParam(GenericParamRow) = 19,
    GenericParamConstraint(GenericParamConstraintRow) = 20,
    MethodSpec(MethodSpecRow) = 21
  }
}

coded_id! {
  enum HasFieldMarshal : 1 {
    Field(FieldRow) = 0,
    Param(ParamRow) = 1
  }
}

coded_id! {
  enum HasDeclSecurity : 2 {
    TypeDef(TypeDefRow) = 0,
    MethodDef(MethodDefRow) = 1,
    Assembly(AssemblyRow) = 2
  }
}

coded_id! {
  enum MemberRefParent : 3 {
    TypeDef(TypeDefRow) = 0,
    TypeRef(TypeRefRow) = 1,
    ModuleRef(ModuleRefRow) = 2,
    MethodDef(MethodDefRow) = 3,
    TypeSpec(TypeSpecRow) = 4
  }
}

coded_id! {
  enum HasSemantics : 1 {
    Event(EventRow) = 0,
    Property(PropertyRow) = 1
  }
}

coded_id! {
  enum MethodDefOrRef : 1 {
    MethodDef(MethodDefRow) = 0,
    MemberRef(MemberRefRow) = 1
  }
}

coded_id! {
  enum MemberForwarded : 1 {
    Field(FieldRow) = 0,
    MethodDef(MethodDefRow) = 1
  }
}

coded_id! {
  enum Implementation : 2 {
    File(FileRow) = 0,
    AssemblyRef(AssemblyRefRow) = 1,
    ExportedType(ExportedTypeRow) = 2
  }
}

coded_id! {
  enum CustomAttributeType : 3 {
    MethodDef(MethodDefRow) = 2,
    MemberRef(MemberRefRow) = 3
  }
}

coded_id! {
  enum ResolutionScope : 2 {
    Module(ModuleRow) = 0,
    ModuleRef(ModuleRefRow) = 1,
    AssemblyRef(AssemblyRefRow) = 2,
    TypeRef(TypeRefRow) = 3
  }
}

coded_id! {
  enum TypeOrMethodDef : 1 {
    TypeDef(TypeDefRow) = 0,
    MethodDef(MethodDefRow) = 1
  }
}

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  use super::RowId;
  use crate::bytes::{ByteSize, ByteSliceExt, FromBytes};
  use crate::metadata::headers::MetadataTablesHeader;
  use crate::metadata::streams::tables::table::RowRead;

  impl<R: RowRead> FromBytes<'_, &MetadataTablesHeader> for RowId<R> {
    fn from_bytes(buf: &[u8], offset: &mut usize, header: &MetadataTablesHeader) -> Option<Self> {
      Some(Self::new(match Self::byte_size(header) {
        2 => buf.read::<u16>(offset)? as usize,
        4 => buf.read::<u32>(offset)? as usize,
        _ => unreachable!(),
      }))
    }
  }

  impl<R: RowRead> ByteSize<&MetadataTablesHeader> for RowId<R> {
    fn byte_size(header: &MetadataTablesHeader) -> usize {
      let len = R::table_len(header);
      match len < 1 << 16 {
        true => 2,
        false => 4,
      }
    }
  }
}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {}
