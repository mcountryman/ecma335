//! The `#~` metadata stream.

pub mod flags;
pub mod id;
pub mod rows;
pub mod signatures;
pub mod table;

#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;
#[cfg(feature = "write")]
#[doc(inline)]
pub use write::*;

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {
  use super::rows::{ModuleRow, *};
  use super::table::{TableBytes, TableReader};
  use crate::bytes::ByteSliceExt;
  use crate::metadata::errors::MetadataStreamReadError;
  use crate::metadata::headers::MetadataTablesHeader;
  use core::fmt;

  /// The `#~` metadata stream.
  ///
  /// This stream contains the metadata tables.  It is the only stream that is required to be present
  /// in the metadata.
  #[derive(Clone, Copy)]
  pub struct TablesStream<'a> {
    bytes: &'a [u8],
    header: MetadataTablesHeader,
    assemblies: TableBytes<'a, AssemblyRow>,
    assembly_oses: TableBytes<'a, AssemblyOsRow>,
    assembly_processors: TableBytes<'a, AssemblyProcessorRow>,
    assembly_refs: TableBytes<'a, AssemblyRefRow>,
    assembly_ref_oses: TableBytes<'a, AssemblyRefOsRow>,
    assembly_ref_processors: TableBytes<'a, AssemblyRefProcessorRow>,
    class_layouts: TableBytes<'a, ClassLayoutRow>,
    constants: TableBytes<'a, ConstantRow>,
    custom_attributes: TableBytes<'a, CustomAttributeRow>,
    decl_securities: TableBytes<'a, DeclSecurityRow>,
    events: TableBytes<'a, EventRow>,
    event_maps: TableBytes<'a, EventMapRow>,
    exported_types: TableBytes<'a, ExportedTypeRow>,
    fields: TableBytes<'a, FieldRow>,
    field_layouts: TableBytes<'a, FieldLayoutRow>,
    field_marshals: TableBytes<'a, FieldMarshalRow>,
    field_rvas: TableBytes<'a, FieldRvaRow>,
    files: TableBytes<'a, FileRow>,
    generic_params: TableBytes<'a, GenericParamRow>,
    generic_param_constraints: TableBytes<'a, GenericParamConstraintRow>,
    impl_maps: TableBytes<'a, ImplMapRow>,
    interface_impls: TableBytes<'a, InterfaceImplRow>,
    manifest_resources: TableBytes<'a, ManifestResourceRow>,
    member_refs: TableBytes<'a, MemberRefRow>,
    method_defs: TableBytes<'a, MethodDefRow>,
    method_impls: TableBytes<'a, MethodImplRow>,
    method_semantics: TableBytes<'a, MethodSemanticsRow>,
    method_specs: TableBytes<'a, MethodSpecRow>,
    modules: TableBytes<'a, ModuleRow>,
    module_refs: TableBytes<'a, ModuleRefRow>,
    nested_classes: TableBytes<'a, NestedClassRow>,
    params: TableBytes<'a, ParamRow>,
    properties: TableBytes<'a, PropertyRow>,
    property_maps: TableBytes<'a, PropertyMapRow>,
    stand_alone_sigs: TableBytes<'a, StandAloneSigRow>,
    type_defs: TableBytes<'a, TypeDefRow>,
    type_refs: TableBytes<'a, TypeRefRow>,
    type_specs: TableBytes<'a, TypeSpecRow>,
  }

  impl<'a> TablesStream<'a> {
    /// Creates a [TablesStream] from the given bytes.
    ///
    /// Attempts to parse and verify the tables stream header before returning.
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, MetadataStreamReadError<'a>> {
      let offset = &mut 0;
      let header = bytes
        .read::<MetadataTablesHeader>(offset)
        .ok_or(MetadataStreamReadError::NotEnough)?;
      let mut assemblies = TableBytes::default();
      let mut assembly_oses = TableBytes::default();
      let mut assembly_processors = TableBytes::default();
      let mut assembly_refs = TableBytes::default();
      let mut assembly_ref_oses = TableBytes::default();
      let mut assembly_ref_processors = TableBytes::default();
      let mut class_layouts = TableBytes::default();
      let mut constants = TableBytes::default();
      let mut custom_attributes = TableBytes::default();
      let mut decl_securities = TableBytes::default();
      let mut events = TableBytes::default();
      let mut event_maps = TableBytes::default();
      let mut exported_types = TableBytes::default();
      let mut fields = TableBytes::default();
      let mut field_layouts = TableBytes::default();
      let mut field_marshals = TableBytes::default();
      let mut field_rvas = TableBytes::default();
      let mut files = TableBytes::default();
      let mut generic_params = TableBytes::default();
      let mut generic_param_constraints = TableBytes::default();
      let mut impl_maps = TableBytes::default();
      let mut interface_impls = TableBytes::default();
      let mut manifest_resources = TableBytes::default();
      let mut member_refs = TableBytes::default();
      let mut method_defs = TableBytes::default();
      let mut method_impls = TableBytes::default();
      let mut method_semantics = TableBytes::default();
      let mut method_specs = TableBytes::default();
      let mut modules = TableBytes::default();
      let mut module_refs = TableBytes::default();
      let mut nested_classes = TableBytes::default();
      let mut params = TableBytes::default();
      let mut properties = TableBytes::default();
      let mut property_maps = TableBytes::default();
      let mut stand_alone_sigs = TableBytes::default();
      let mut type_defs = TableBytes::default();
      let mut type_refs = TableBytes::default();
      let mut type_specs = TableBytes::default();

      for table in 0..header.rows.len() {
        match table {
          AssemblyRow::ID => assemblies = TableBytes::from_bytes(bytes, offset, &header)?,
          AssemblyOsRow::ID => assembly_oses = TableBytes::from_bytes(bytes, offset, &header)?,
          AssemblyProcessorRow::ID => {
            assembly_processors = TableBytes::from_bytes(bytes, offset, &header)?
          }
          AssemblyRefRow::ID => assembly_refs = TableBytes::from_bytes(bytes, offset, &header)?,
          AssemblyRefOsRow::ID => {
            assembly_ref_oses = TableBytes::from_bytes(bytes, offset, &header)?
          }
          AssemblyRefProcessorRow::ID => {
            assembly_ref_processors = TableBytes::from_bytes(bytes, offset, &header)?
          }
          ClassLayoutRow::ID => class_layouts = TableBytes::from_bytes(bytes, offset, &header)?,
          ConstantRow::ID => constants = TableBytes::from_bytes(bytes, offset, &header)?,
          CustomAttributeRow::ID => {
            custom_attributes = TableBytes::from_bytes(bytes, offset, &header)?
          }
          DeclSecurityRow::ID => decl_securities = TableBytes::from_bytes(bytes, offset, &header)?,
          EventRow::ID => events = TableBytes::from_bytes(bytes, offset, &header)?,
          EventMapRow::ID => event_maps = TableBytes::from_bytes(bytes, offset, &header)?,
          ExportedTypeRow::ID => exported_types = TableBytes::from_bytes(bytes, offset, &header)?,
          FieldRow::ID => fields = TableBytes::from_bytes(bytes, offset, &header)?,
          FieldLayoutRow::ID => field_layouts = TableBytes::from_bytes(bytes, offset, &header)?,
          FieldMarshalRow::ID => field_marshals = TableBytes::from_bytes(bytes, offset, &header)?,
          FieldRvaRow::ID => field_rvas = TableBytes::from_bytes(bytes, offset, &header)?,
          FileRow::ID => files = TableBytes::from_bytes(bytes, offset, &header)?,
          GenericParamRow::ID => generic_params = TableBytes::from_bytes(bytes, offset, &header)?,
          GenericParamConstraintRow::ID => {
            generic_param_constraints = TableBytes::from_bytes(bytes, offset, &header)?
          }
          ImplMapRow::ID => impl_maps = TableBytes::from_bytes(bytes, offset, &header)?,
          InterfaceImplRow::ID => interface_impls = TableBytes::from_bytes(bytes, offset, &header)?,
          ManifestResourceRow::ID => {
            manifest_resources = TableBytes::from_bytes(bytes, offset, &header)?
          }
          MemberRefRow::ID => member_refs = TableBytes::from_bytes(bytes, offset, &header)?,
          MethodDefRow::ID => method_defs = TableBytes::from_bytes(bytes, offset, &header)?,
          MethodImplRow::ID => method_impls = TableBytes::from_bytes(bytes, offset, &header)?,
          MethodSemanticsRow::ID => {
            method_semantics = TableBytes::from_bytes(bytes, offset, &header)?
          }
          MethodSpecRow::ID => method_specs = TableBytes::from_bytes(bytes, offset, &header)?,
          ModuleRow::ID => modules = TableBytes::from_bytes(bytes, offset, &header)?,
          ModuleRefRow::ID => module_refs = TableBytes::from_bytes(bytes, offset, &header)?,
          NestedClassRow::ID => nested_classes = TableBytes::from_bytes(bytes, offset, &header)?,
          ParamRow::ID => params = TableBytes::from_bytes(bytes, offset, &header)?,
          PropertyRow::ID => properties = TableBytes::from_bytes(bytes, offset, &header)?,
          PropertyMapRow::ID => property_maps = TableBytes::from_bytes(bytes, offset, &header)?,
          StandAloneSigRow::ID => {
            stand_alone_sigs = TableBytes::from_bytes(bytes, offset, &header)?
          }
          TypeDefRow::ID => type_defs = TableBytes::from_bytes(bytes, offset, &header)?,
          TypeRefRow::ID => type_refs = TableBytes::from_bytes(bytes, offset, &header)?,
          TypeSpecRow::ID => type_specs = TableBytes::from_bytes(bytes, offset, &header)?,
          _ => {}
        }
      }

      Ok(Self {
        bytes,
        header,
        assemblies,
        assembly_oses,
        assembly_processors,
        assembly_refs,
        assembly_ref_oses,
        assembly_ref_processors,
        class_layouts,
        constants,
        custom_attributes,
        decl_securities,
        events,
        event_maps,
        exported_types,
        fields,
        field_layouts,
        field_marshals,
        field_rvas,
        files,
        generic_params,
        generic_param_constraints,
        impl_maps,
        interface_impls,
        manifest_resources,
        member_refs,
        method_defs,
        method_impls,
        method_semantics,
        method_specs,
        modules,
        module_refs,
        nested_classes,
        params,
        properties,
        property_maps,
        stand_alone_sigs,
        type_defs,
        type_refs,
        type_specs,
      })
    }

    /// Returns the bytes used to create this [TablesStream].
    #[inline]
    pub fn bytes(&self) -> &'a [u8] {
      self.bytes
    }

    /// Returns a reader for [AssemblyRow]s.
    #[inline]
    pub fn assemblies(&self) -> TableReader<'a, '_, AssemblyRow> {
      self.assemblies.reader(&self.header)
    }

    /// Returns a reader for [AssemblyOsRow]s.
    #[inline]
    pub fn assembly_oses(&self) -> TableReader<'a, '_, AssemblyOsRow> {
      self.assembly_oses.reader(&self.header)
    }

    /// Returns a reader for [AssemblyProcessorRow]s.
    #[inline]
    pub fn assembly_processors(&self) -> TableReader<'a, '_, AssemblyProcessorRow> {
      self.assembly_processors.reader(&self.header)
    }

    /// Returns a reader for [AssemblyRefRow]s.
    #[inline]
    pub fn assembly_refs(&self) -> TableReader<'a, '_, AssemblyRefRow> {
      self.assembly_refs.reader(&self.header)
    }

    /// Returns a reader for [AssemblyRefOsRow]s.
    #[inline]
    pub fn assembly_ref_oses(&self) -> TableReader<'a, '_, AssemblyRefOsRow> {
      self.assembly_ref_oses.reader(&self.header)
    }

    /// Returns a reader for [AssemblyRefProcessorRow]s.
    #[inline]
    pub fn assembly_ref_processors(&self) -> TableReader<'a, '_, AssemblyRefProcessorRow> {
      self.assembly_ref_processors.reader(&self.header)
    }

    /// Returns a reader for [ClassLayoutRow]s.
    #[inline]
    pub fn class_layouts(&self) -> TableReader<'a, '_, ClassLayoutRow> {
      self.class_layouts.reader(&self.header)
    }

    /// Returns a reader for [ConstantRow]s.
    #[inline]
    pub fn constants(&self) -> TableReader<'a, '_, ConstantRow> {
      self.constants.reader(&self.header)
    }

    /// Returns a reader for [CustomAttributeRow]s.
    #[inline]
    pub fn custom_attributes(&self) -> TableReader<'a, '_, CustomAttributeRow> {
      self.custom_attributes.reader(&self.header)
    }

    /// Returns a reader for [DeclSecurityRow]s.
    #[inline]
    pub fn decl_securities(&self) -> TableReader<'a, '_, DeclSecurityRow> {
      self.decl_securities.reader(&self.header)
    }

    /// Returns a reader for [EventRow]s.
    #[inline]
    pub fn events(&self) -> TableReader<'a, '_, EventRow> {
      self.events.reader(&self.header)
    }

    /// Returns a reader for [EventMapRow]s.
    #[inline]
    pub fn event_maps(&self) -> TableReader<'a, '_, EventMapRow> {
      self.event_maps.reader(&self.header)
    }

    /// Returns a reader for [ExportedTypeRow]s.
    #[inline]
    pub fn exported_types(&self) -> TableReader<'a, '_, ExportedTypeRow> {
      self.exported_types.reader(&self.header)
    }

    /// Returns a reader for [FieldRow]s.
    #[inline]
    pub fn fields(&self) -> TableReader<'a, '_, FieldRow> {
      self.fields.reader(&self.header)
    }

    /// Returns a reader for [FieldLayoutRow]s.
    #[inline]
    pub fn field_layouts(&self) -> TableReader<'a, '_, FieldLayoutRow> {
      self.field_layouts.reader(&self.header)
    }

    /// Returns a reader for [FieldMarshalRow]s.
    #[inline]
    pub fn field_marshals(&self) -> TableReader<'a, '_, FieldMarshalRow> {
      self.field_marshals.reader(&self.header)
    }

    /// Returns a reader for [FieldRvaRow]s.
    #[inline]
    pub fn field_rvas(&self) -> TableReader<'a, '_, FieldRvaRow> {
      self.field_rvas.reader(&self.header)
    }

    /// Returns a reader for [FileRow]s.
    #[inline]
    pub fn files(&self) -> TableReader<'a, '_, FileRow> {
      self.files.reader(&self.header)
    }

    /// Returns a reader for [GenericParamRow]s.
    #[inline]
    pub fn generic_params(&self) -> TableReader<'a, '_, GenericParamRow> {
      self.generic_params.reader(&self.header)
    }

    /// Returns a reader for [GenericParamConstraintRow]s.
    #[inline]
    pub fn generic_param_constraints(&self) -> TableReader<'a, '_, GenericParamConstraintRow> {
      self.generic_param_constraints.reader(&self.header)
    }

    /// Returns a reader for [ImplMapRow]s.
    #[inline]
    pub fn impl_maps(&self) -> TableReader<'a, '_, ImplMapRow> {
      self.impl_maps.reader(&self.header)
    }

    /// Returns a reader for [InterfaceImplRow]s.
    #[inline]
    pub fn interface_impls(&self) -> TableReader<'a, '_, InterfaceImplRow> {
      self.interface_impls.reader(&self.header)
    }

    /// Returns a reader for [ManifestResourceRow]s.
    #[inline]
    pub fn manifest_resources(&self) -> TableReader<'a, '_, ManifestResourceRow> {
      self.manifest_resources.reader(&self.header)
    }

    /// Returns a reader for [MemberRefRow]s.
    #[inline]
    pub fn member_refs(&self) -> TableReader<'a, '_, MemberRefRow> {
      self.member_refs.reader(&self.header)
    }

    /// Returns a reader for [MethodDefRow]s.
    #[inline]
    pub fn method_defs(&self) -> TableReader<'a, '_, MethodDefRow> {
      self.method_defs.reader(&self.header)
    }

    /// Returns a reader for [MethodImplRow]s.
    #[inline]
    pub fn method_impls(&self) -> TableReader<'a, '_, MethodImplRow> {
      self.method_impls.reader(&self.header)
    }

    /// Returns a reader for [MethodSemanticsRow]s.
    #[inline]
    pub fn method_semantics(&self) -> TableReader<'a, '_, MethodSemanticsRow> {
      self.method_semantics.reader(&self.header)
    }

    /// Returns a reader for [MethodSpecRow]s.
    #[inline]
    pub fn method_specs(&self) -> TableReader<'a, '_, MethodSpecRow> {
      self.method_specs.reader(&self.header)
    }

    /// Returns a reader for [ModuleRow]s.
    #[inline]
    pub fn modules(&self) -> TableReader<'a, '_, ModuleRow> {
      self.modules.reader(&self.header)
    }

    /// Returns a reader for [ModuleRefRow]s.
    #[inline]
    pub fn module_refs(&self) -> TableReader<'a, '_, ModuleRefRow> {
      self.module_refs.reader(&self.header)
    }

    /// Returns a reader for [NestedClassRow]s.
    #[inline]
    pub fn nested_classes(&self) -> TableReader<'a, '_, NestedClassRow> {
      self.nested_classes.reader(&self.header)
    }

    /// Returns a reader for [ParamRow]s.
    #[inline]
    pub fn params(&self) -> TableReader<'a, '_, ParamRow> {
      self.params.reader(&self.header)
    }

    /// Returns a reader for [PropertyRow]s.
    #[inline]
    pub fn properties(&self) -> TableReader<'a, '_, PropertyRow> {
      self.properties.reader(&self.header)
    }

    /// Returns a reader for [PropertyMapRow]s.
    #[inline]
    pub fn property_maps(&self) -> TableReader<'a, '_, PropertyMapRow> {
      self.property_maps.reader(&self.header)
    }

    /// Returns a reader for [StandAloneSigRow]s.
    #[inline]
    pub fn stand_alone_sigs(&self) -> TableReader<'a, '_, StandAloneSigRow> {
      self.stand_alone_sigs.reader(&self.header)
    }

    /// Returns a reader for [TypeDefRow]s.
    #[inline]
    pub fn type_defs(&self) -> TableReader<'a, '_, TypeDefRow> {
      self.type_defs.reader(&self.header)
    }

    /// Returns a reader for [TypeRefRow]s.
    #[inline]
    pub fn type_refs(&self) -> TableReader<'a, '_, TypeRefRow> {
      self.type_refs.reader(&self.header)
    }

    /// Returns a reader for [TypeSpecRow]s.
    #[inline]
    pub fn type_specs(&self) -> TableReader<'a, '_, TypeSpecRow> {
      self.type_specs.reader(&self.header)
    }
  }

  impl fmt::Debug for TablesStream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("TablesStream")
        .field("header", &self.header)
        .finish()
    }
  }
}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {}
