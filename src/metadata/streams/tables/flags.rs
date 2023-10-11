use crate::bytes;

bytes::bitflags! {
  #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
  pub struct AssemblyHashAlgorithm : u32 {
    const MD5 = 0x8003;
    const SHA1 = 0x8004;
    const SHA256 = 0x800C;
    const SHA384 = 0x800D;
    const SHA512 = 0x800E;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct AssemblyFlags : u32 {
    /// The assembly reference holds the full (unhashed) public key.
    const PUBLIC_KEY = 0x0001;
    /// The implementation of this assembly used at runtime is not expected to match the version
    /// seen at compile time.
    const RETARGETABLE =  0x0100;
    /// Reserved (a conforming implementation of the CLI can ignore this setting on read; some
    /// implementations might use this bit to indicate that a CIL-to-native-code compiler should not
    /// should not generate optimized code)
    const DISABLE_JIT_COMPILE_OPTIMIZER = 0x4000;
    /// Reserved (a conforming implementation of the CLI can ignore this setting on read; some
    /// implementations might use this bit to indicate that a CIL-to-native-code compiler should
    /// generate CIL-to-native code map)
    const ENABLE_JIT_COMPILE_TRACKING = 0x8000;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct EventAttributes : u16 {
    /// The event is special.
    const SPECIAL_NAME = 0x0200;
    /// CLI provides 'special' behavior, depending upon the name of the event.
    const RTSPECIAL_NAME = 0x0400;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct FieldAttributes : u16 {
    // These 3 bits contain one of the following values:
    const FIELD_ACCESS_MASK = 0x0007;
    // Member not referenceable
    const COMPILER_CONTROLLED = 0x0000;
    // Accessible only by the parent type
    const PRIVATE = 0x0001;
    // Accessible by sub-types only in this Assembly
    const FAM_AND_ASSEM = 0x0002;
    // Accessibly by anyone in the Assembly
    const ASSEMBLY = 0x0003;
    // Accessible only by type and sub-types
    const FAMILY = 0x0004;
    // Accessibly by sub-types anywhere, plus anyone in assembly
    const FAM_OR_ASSEM = 0x0005;
    // Accessibly by anyone who has visibility to this scope field contract attributes
    const PUBLIC = 0x0006;
    // Defined on type, else per instance
    const STATIC = 0x0010;
    // Field can only be initialized, not written to after init
    const INIT_ONLY = 0x0020;
    // Value is compile time constant
    const LITERAL = 0x0040;
    // Reserved (to indicate this field should not be serialized when type is remoted)
    const NOT_SERIALIZED = 0x0080;

    // Field is special
    const SPECIAL_NAME = 0x0200;
    // Implementation is forwarded through PInvoke.
    const P_INVOKE_IMPL = 0x2000;

    // CLI provides 'special' behavior, depending upon the name of the field
    const RT_SPECIAL_NAME = 0x0400;
    // Field has marshalling information
    const HAS_FIELD_MARSHAL = 0x1000;
    // Field has default
    const HAS_DEFAULT = 0x8000;
    // Field has RVA
    const HAS_FIELD_RVA = 0x0100;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct FileAttributes: u32 {
    const CONTAINS_METADATA = 0x0000;
    const CONTAINS_NO_METADATA = 0x0001;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct GenericParamAttributes : u16 {
  // These 2 bits contain one of the following values:
    const VARIANCE_MASK = 0x0003;
    // The generic parameter is non-variant and has no special constraints
    const NONE = 0x0000;
    // The generic parameter is covariant
    const COVARIANT = 0x0001;
    // The generic parameter is contravariant
    const CONTRAVARIANT = 0x0002;
    // These 3 bits contain one of the following values:
    const SPECIAL_CONSTRAINT_MASK = 0x001C;
    // The GENERIC parameter has the class special constraint
    const REFERENCE_TYPE_CONSTRAINT = 0x0004;
    // The generic parameter has the valuetype special constraint
    const NOT_NULLABLE_VALUE_TYPE_CONSTRAINT = 0x0008;
    // The generic parameter has the .ctor special constraint
    const DEFAULT_CONSTRUCTOR_CONSTRAINT = 0x0010;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct PInvokeAttributes : u16 {
    /// PInvoke is to use the member name as specified
    const NO_MANGLE = 0x0001;
    /// This is a resource file or other non-metadata-containing file.
    const CHAR_SET_MASK = 0x0006;
    const CHAR_SET_NOT_SPEC = 0x0000;
    const CHAR_SET_ANSI = 0x0002;
    const CHAR_SET_UNICODE = 0x0004;
    const CHAR_SET_AUTO = 0x0006;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct ManifestResourceAttributes : u32 {
    /// The resource is exported from the assembly.
    const PUBLIC = 0x0001;
    /// The resource is private to the assembly.
    const PRIVATE = 0x0002;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct MethodImplAttributes : u16 {
    /// These 3 bits contain one of the following values:
    const MEMBER_ACCESS_MASK = 0x0007;
    /// Member not referenceable
    const COMPILER_CONTROLLED = 0x0000;
    /// Accessible only by the parent type
    const PRIVATE = 0x0001;
    /// Accessible by sub-types only in this Assembly
    const FAM_AND_ASSEM = 0x0002;
    /// Accessibly by anyone in the Assembly
    const ASSEM = 0x0003;
    /// Accessible only by type and sub-types
    const FAMILY = 0x0004;
    /// Accessibly by sub-types anywhere, plus anyone in assembly
    const FAM_OR_ASSEM = 0x0005;
    /// Accessibly by anyone who has visibility to this scope
    const PUBLIC = 0x0006;
    /// Defined on type, else per instance
    const STATIC = 0x0010;
    /// Method cannot be overridden
    const FINAL = 0x0020;
    /// Method is virtual
    const VIRTUAL = 0x0040;
    /// Method hides by name+sig, else just by name
    const HIDE_BY_SIG = 0x0080;
    /// Use this mask to retrieve vtable attributes. This bit contains one of the following values:
    const VTABLE_LAYOUT_MASK = 0x0100;
    /// Method reuses existing slot in vtable
    const REUSE_SLOT = 0x0000;
    /// Method always gets a new slot in the vtable
    const NEW_SLOT = 0x0100;
    /// Method can only be overriden if also accessible
    const STRICT = 0x0200;
    /// Method does not provide an implementation
    const ABSTRACT = 0x0400;
    /// Method is special
    const SPECIAL_NAME = 0x0800;

    // Interop attributes

    /// Implementation is forwarded through PInvoke
    const P_INVOKE_IMPL = 0x2000;
    /// Reserved: shall be zero for conforming implementations
    const UNMANAGED_EXPORT = 0x0008;

    // Additional flags

    /// CLI provides 'special' behavior, depending upon the name of the method
    const RT_SPECIAL_NAME = 0x1000;
    /// Method has security associate with it
    const HAS_SECURITY = 0x4000;
    /// Method calls another method containing security code.
    const REQUIRE_SEC_OBJECT = 0x8000;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct MethodAttributes : u16 {
    /// These 3 bits contain one of the following values:
    const MEMBER_ACCESS_MASK = 0x0007;
    /// Member not referenceable
    const COMPILER_CONTROLLED = 0x0000;
    /// Accessible only by the parent type
    const PRIVATE = 0x0001;
    /// Accessible by sub-types only in this Assembly
    const FAM_AND_ASSEM = 0x0002;
    /// Accessibly by anyone in the Assembly
    const ASSEM = 0x0003;
    /// Accessible only by type and sub-types
    const FAMILY = 0x0004;
    /// Accessibly by sub-types anywhere, plus anyone in assembly
    const FAM_OR_ASSEM = 0x0005;
    /// Accessibly by anyone who has visibility to this scope
    const PUBLIC = 0x0006;
    /// Defined on type, else per instance
    const STATIC = 0x0010;
    /// Method cannot be overridden
    const FINAL = 0x0020;
    /// Method is virtual
    const VIRTUAL = 0x0040;
    /// Method hides by name+sig, else just by name
    const HIDE_BY_SIG = 0x0080;
    /// Use this mask to retrieve vtable attributes. This bit contains one of the following values:
    const VTABLE_LAYOUT_MASK = 0x0100;
    /// Method reuses existing slot in vtable
    const REUSE_SLOT = 0x0000;
    /// Method always gets a new slot in the vtable
    const NEW_SLOT = 0x0100;
    /// Method can only be overriden if also accessible
    const STRICT = 0x0200;
    /// Method does not provide an implementation
    const ABSTRACT = 0x0400;
    /// Method is special
    const SPECIAL_NAME = 0x0800;

    // Interop attributes

    /// Implementation is forwarded through PInvoke
    const P_INVOKE_IMPL = 0x2000;
    /// Reserved: shall be zero for conforming implementations
    const UNMANAGED_EXPORT = 0x0008;

    // Additional flags

    /// CLI provides 'special' behavior, depending upon the name of the method
    const RT_SPECIAL_NAME = 0x1000;
    /// Method has security associate with it
    const HAS_SECURITY = 0x4000;
    /// Method calls another method containing security code
    const REQUIRE_SEC_OBJECT = 0x8000;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct MethodSemanticsAttributes : u16 {
    /// Setter for property
    const SETTER = 0x0001;
    /// Getter for property
    const GETTER = 0x0002;
    /// Other method for property or event
    const OTHER = 0x0004;
    /// AddOn method for event. This refers to the required add_ method for events.
    const ADD_ON = 0x0008;
    /// RemoveOn method for event. . This refers to the required remove_ method for events.
    const REMOVE_ON = 0x0010;
    /// Fire method for event. This refers to the optional raise_ method for events.
    const FIRE = 0x0020;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct ParamAttributes : u16 {
    /// Param is [In]
    const IN = 0x0001;
    /// Param is [out]
    const OUT = 0x0002;
    /// Param is optional
    const OPTIONAL = 0x0010;
    /// Param has default value
    const HAS_DEFAULT = 0x1000;
    /// Param has FieldMarshal
    const HAS_FIELD_MARSHAL = 0x2000;
    /// Reserved: shall be zero in a conforming implementation
    const UNUSED = 0xcfe0;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct PropertyAttributes : u16 {
    /// Property is special
    const SPECIAL_NAME = 0x0200;
    /// Runtime(metadata internal APIs) should check name encoding
    const RT_SPECIAL_NAME = 0x0400;
    /// Property has default
    const HAS_DEFAULT = 0x1000;
    /// Reserved: shall be zero in a conforming implementation
    const UNUSED = 0xe9ff;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct TypeAttributes : u32 {
    // Visibility attributes

    /// Use this mask to retrieve visibility information. These 3 bits contain one of the following values:
    const VISIBILITY_MASK = 0x00000007;
    /// Class has no public scope
    const NOT_PUBLIC = 0x00000000;
    /// Class has public scope
    const PUBLIC = 0x00000001;
    /// Class is nested with public visibility
    const NESTED_PUBLIC = 0x00000002;
    /// Class is nested with private visibility
    const NESTED_PRIVATE = 0x00000003;
    /// Class is nested with family visibility
    const NESTED_FAMILY = 0x00000004;
    /// Class is nested with assembly visibility
    const NESTED_ASSEMBLY = 0x00000005;
    /// Class is nested with family and assembly visibility
    const NESTED_FAM_AND_ASSEM = 0x00000006;
    /// Class is nested with family or assembly visibility
    const NESTED_FAM_OR_ASSEM = 0x00000007;

    // Class layout attributes

    /// Use this mask to retrieve class layout information. These 2 bits contain one of the following values:
    const LAYOUT_MASK = 0x00000018;
    /// Class fields are auto-laid out
    const AUTO_LAYOUT = 0x00000000;
    /// Class fields are laid out sequentially
    const SEQUENTIAL_LAYOUT = 0x00000008;
    /// Layout is supplied explicitly
    const EXPLICIT_LAYOUT = 0x00000010;

    // Class semantics attributes

    /// Use this mask to retrive class semantics information. This bit contains one of the following values:
    const CLASS_SEMANTICS_MASK = 0x00000020;
    /// Type is a class
    const CLASS = 0x00000000;
    /// Type is an interface
    const INTERFACE = 0x00000020;

    /// Class is abstract
    const ABSTRACT = 0x00000080;
    /// Class cannot be extended
    const SEALED = 0x00000100;
    /// Class name is special
    const SPECIAL_NAME = 0x00000400;

    // Implementation Attributes

    /// Class/Interface is imported
    const IMPORT = 0x00001000;
    /// Reserved (Class is serializable)
    const SERIALIZABLE = 0x00002000;

    /// Use this mask to retrieve string information for native interop. These 2 bits contain one of the following values:
    const STRING_FORMAT_MASK = 0x00030000;
    /// LPSTR is interpreted as ANSI
    const ANSI_CLASS = 0x00000000;
    /// LPSTR is interpreted as Unicode
    const UNICODE_CLASS = 0x00010000;
    /// LPSTR is interpreted automatically
    const AUTO_CLASS = 0x00020000;
    /// A non-standard encoding specified by
    const CUSTOM_FORMAT_CLASS = 0x00030000;

    // CustomStringFormatMask

    /// Use this mask to retrieve non-standard encoding information for native interop. The meaning of the values of these 2 bits is unspecified.
    const CUSTOM_STRING_FORMAT_MASK = 0x00C00000;

    // Class Initialization Attributes

    /// Initialize the class before first static field access
    const BEFORE_FIELD_INIT = 0x00100000;

    // Additional Flags

    /// CLI provides 'special' behavior, depending upon the name of the Type
    const RT_SPECIAL_NAME = 0x00000800;
    /// Type has security associate with it
    const HAS_SECURITY = 0x00040000;
    /// This ExportedType entry is a type forwarder
    const IS_TYPE_FORWARDER = 0x00200000;
  }
}

bytes::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct ElementType : u8 {
    const ELEMENT_TYPE_END =  0x00; //  Marks end of a list
    const ELEMENT_TYPE_VOID =  0x01; //
    const ELEMENT_TYPE_BOOLEAN =  0x02; //
    const ELEMENT_TYPE_CHAR =  0x03; //
    const ELEMENT_TYPE_I1 =  0x04; //
    const ELEMENT_TYPE_U1 =  0x05; //
    const ELEMENT_TYPE_I2 =  0x06; //
    const ELEMENT_TYPE_U2 =  0x07; //
    const ELEMENT_TYPE_I4 =  0x08; //
    const ELEMENT_TYPE_U4 =  0x09; //
    const ELEMENT_TYPE_I8 =  0x0a; //
    const ELEMENT_TYPE_U8 =  0x0b; //
    const ELEMENT_TYPE_R4 =  0x0c; //
    const ELEMENT_TYPE_R8 =  0x0d; //
    const ELEMENT_TYPE_STRING =  0x0e; //
    const ELEMENT_TYPE_PTR =  0x0f; //  Followed by type
    const ELEMENT_TYPE_BYREF =  0x10; //  Followed by type
    const ELEMENT_TYPE_VALUETYPE =  0x11; //  Followed by TypeDef or TypeRef token
    const ELEMENT_TYPE_CLASS =  0x12; //  Followed by TypeDef or TypeRef token
    const ELEMENT_TYPE_VAR =  0x13; //  Generic parameter in a generic type definition, represented as number (compressed unsigned integer)
    const ELEMENT_TYPE_ARRAY =  0x14; //  type rank boundsCount bound1 … loCount lo1 …
    const ELEMENT_TYPE_GENERICINST =  0x15; //  Generic type instantiation. Followed by type type-arg-count type-1 ... type-n
    const ELEMENT_TYPE_TYPEDBYREF =  0x16; //
    const ELEMENT_TYPE_I =  0x18; //  System.IntPtr
    const ELEMENT_TYPE_U =  0x19; //  System.UIntPtr
    const ELEMENT_TYPE_FNPTR =  0x1b; //  Followed by full method signature
    const ELEMENT_TYPE_OBJECT =  0x1c; //  System.Object
    const ELEMENT_TYPE_SZARRAY =  0x1d; //  Single-dim array with 0 lower bound
    const ELEMENT_TYPE_MVAR =  0x1e; //  Generic parameter in a generic method definition, represented as number (compressed unsigned integer)
    const ELEMENT_TYPE_CMOD_REQD =  0x1f; //  Required modifier : followed by a TypeDef or TypeRef token
    const ELEMENT_TYPE_CMOD_OPT =  0x20; //  Optional modifier : followed by a TypeDef or TypeRef token
    const ELEMENT_TYPE_INTERNAL =  0x21; //  Implemented within the CLI
    const ELEMENT_TYPE_MODIFIER =  0x40; //  Or’d with following element types
    const ELEMENT_TYPE_SENTINEL =  0x41; //  Sentinel for vararg method signature
    const ELEMENT_TYPE_PINNED =  0x45; //  Denotes a local variable that points at a pinned object
  }
}
