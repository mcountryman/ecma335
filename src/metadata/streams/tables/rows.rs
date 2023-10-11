use super::flags::*;
use super::id::*;
use super::table::{self, Row};
use crate::metadata::streams::blobs::BlobId;
use crate::metadata::streams::guids::GuidId;
use crate::metadata::streams::strings::StringId;

table::row! {
  struct ModuleRow : 0x00 {
    /// Reserved, shall be 0.
    generation: u16,
    name: StringId,
    /// The module identity.
    mvid: GuidId,
    enc_id: GuidId,
    enc_base_id: GuidId,
  }
}

table::row! {
  struct TypeRefRow : 0x01 {
    resolution_scope: ResolutionScope,
    name: StringId,
    namespace: StringId,
  }
}

table::row! {
  struct TypeDefRow : 0x02 {
    flags: TypeAttributes,
    name: StringId,
    namespace: StringId,
    extends: TypeDefOrRef,
    field_list: RowId<FieldRow>, // List
    method_list: RowId<MethodDefRow>, // List
  }
}

table::row! {
  struct FieldRow : 0x04 {
    flags: FieldAttributes,
    name: StringId,
    signature: BlobId,
  }
}

table::row! {
  struct MethodDefRow : 0x06 {
    rva: u32,
    impl_flags: MethodImplAttributes,
    flags: MethodAttributes,
    name: StringId,
    signature: BlobId,
    param_list: RowId<ParamRow>, // List
  }
}

table::row! {
  struct AssemblyRow : 0x20 {
    hash_alg: AssemblyHashAlgorithm,
    major_version: u16,
    minor_version: u16,
    build_number: u16,
    revision_number: u16,
    flags: AssemblyFlags,
    public_key: BlobId,
    name: StringId,
    culture: StringId,
  }
}

table::row! {
  /// This row should not be emitted into any PE file. However, if present in a PE file, it shall be
  /// treated as if all it's fields were zero.  It shall be ignored by the CLI.
  struct AssemblyOsRow : 0x22 {
    os_platform_id: u32,
    os_major_version: u32,
    os_minor_version: u32,
  }
}

table::row! {
  /// This row should not be emitted into any PE file. However, if present in a PE file, it shall be
  /// treated as if all it's fields were zero.  It shall be ignored by the CLI.
  struct AssemblyProcessorRow : 0x21 {
    processor: u32,
  }
}

table::row! {
  struct AssemblyRefRow : 0x23 {
    major_version: u16,
    minor_version: u16,
    build_number: u16,
    revision_number: u16,
    flags: AssemblyFlags,
    public_key_or_token: BlobId,
    name: StringId,
    culture: StringId,
    hash_value: BlobId,
  }
}

table::row! {
  /// This row should not be emitted into any PE file. However, if present in a PE file, it shall be
  /// treated as if all it's fields were zero.  It shall be ignored by the CLI.
  struct AssemblyRefOsRow : 0x25 {
    os_platform_id: u32,
    os_major_version: u32,
    os_minor_version: u32,
    assembly_ref: RowId<AssemblyRefRow>,
  }
}

table::row! {
  /// This row should not be emitted into any PE file. However, if present in a PE file, it shall be
  /// treated as if all it's fields were zero.  It shall be ignored by the CLI.
  struct AssemblyRefProcessorRow : 0x24 {
    processor: u32,
    assembly_ref: RowId<AssemblyRefRow>,
  }
}

table::row! {
  /// Defines how the fields of a class or value type are laid out in memory.
  struct ClassLayoutRow : 0x0f {
    packing_size: u16,
    class_size: u32,
    parent: RowId<TypeDefRow>,
  }
}

table::row! {
  struct ConstantRow : 0x0B {
    kind: ElementType,
    _padding: u8,
    parent: HasConstant,
    value: BlobId,
  }
}

table::row! {
  struct CustomAttributeRow : 0x0C {
    parent: HasCustomAttribute,
    attribute_type: CustomAttributeType,
    value: BlobId,
  }
}

table::row! {
  struct DeclSecurityRow : 0x0e {
    action: u16,
    parent: HasDeclSecurity,
    permission_set: BlobId,
  }
}

table::row! {
  struct EventRow : 0x14 {
    flags: EventAttributes,
    name: StringId,
    event_type: TypeDefOrRef,
  }
}

table::row! {
  struct EventMapRow : 0x12 {
    parent: RowId<TypeDefRow>,
    event_list: RowId<EventRow>, // List
  }
}

table::row! {
  struct ExportedTypeRow : 0x27 {
    flags: TypeAttributes,
    type_def_id: RowId<TypeDefRow>,
    type_name: StringId,
    type_namespace: StringId,
    implementation: Implementation,
  }
}

table::row! {
  struct FieldLayoutRow : 0x10 {
     offset: u32,
     field: RowId<FieldRow>,
  }
}

table::row! {
  struct FieldMarshalRow : 0x0d {
    parent: HasFieldMarshal,
    native_type: BlobId,
  }
}

table::row! {
  struct FieldRvaRow : 0x1d {
    rva: u32,
    field: RowId<FieldRow>,
  }
}

table::row! {
  struct FileRow : 0x26 {
    flags: FileAttributes,
    name: StringId,
    hash_value: BlobId,
  }
}

table::row! {
  struct GenericParamRow : 0x2a {
    number: u16,
    flags: GenericParamAttributes,
    owner: TypeOrMethodDef,
    name: StringId,
  }
}

table::row! {
  struct GenericParamConstraintRow : 0x2c {
    owner: RowId<GenericParamRow>,
    constraint: TypeDefOrRef,
  }
}

table::row! {
  /// Holds information about un-managed methods that can be reached from managed code, using
  /// PInvoke dispatch.
  struct ImplMapRow : 0x1c {
    mapping_flags: PInvokeAttributes,
    member_forwarded: MemberForwarded,
    import_name: StringId,
    import_scope: RowId<ModuleRefRow>,
  }
}

table::row! {
  /// Contains interface implementation information.
  struct InterfaceImplRow : 0x09 {
    class: RowId<TypeDefRow>,
    interface: TypeDefOrRef,
  }
}

table::row! {
  struct ManifestResourceRow : 0x28 {
    offset: u32,
    flags: ManifestResourceAttributes,
    name: StringId,
    implementation: Implementation,
  }
}

table::row! {
  /// Contains a reference to a member of a type.
  struct MemberRefRow : 0x0a {
    class: MemberRefParent,
    name: StringId,
    signature: BlobId,
  }
}

table::row! {
  struct MethodImplRow : 0x19 {
    class: RowId<TypeDefRow>,
    method_body: MethodDefOrRef,
    method_declaration: MethodDefOrRef,
  }
}

table::row! {
  struct MethodSemanticsRow : 0x18 {
    semantics: MethodSemanticsAttributes,
    method: RowId<MethodDefRow>,
    association: HasSemantics,
  }
}

table::row! {
  struct MethodSpecRow : 0x2b {
    method: MethodDefOrRef,
    instantiation: BlobId,
  }
}

table::row! {
  struct ModuleRefRow : 0x1a {
    name: StringId,
  }
}

table::row! {
  struct NestedClassRow : 0x29 {
    nested_class: RowId<TypeDefRow>,
    enclosing_class: RowId<TypeDefRow>,
  }
}

table::row! {
  struct ParamRow : 0x08 {
    flags: ParamAttributes,
    sequence: u16,
    name: StringId,
  }
}

table::row! {
  struct PropertyRow : 0x17 {
    flags: PropertyAttributes,
    name: StringId,
    signature: BlobId,
  }
}

table::row! {
  struct PropertyMapRow : 0x15 {
    parent: RowId<TypeDefRow>,
    property_list: RowId<PropertyRow>, // List
  }
}

table::row! {
  struct StandAloneSigRow : 0x11 {
    signature: BlobId,
  }
}

table::row! {
  struct TypeSpecRow : 0x1b {
    signature: BlobId,
  }
}
