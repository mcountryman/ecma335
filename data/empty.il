.assembly extern mscorlib {}
.assembly 'empty' {}

.method static public void main() cil managed
{ .entrypoint
 .maxstack 1
 ldstr "Hello world!"
 call void [mscorlib]System.Console::WriteLine(class System.String)
 ret
}