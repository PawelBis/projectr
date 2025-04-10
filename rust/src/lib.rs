use godot::prelude::*;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

#[derive(GodotClass)]
#[class(init, base=Node3D)]
struct TestNode {
    base: Base<Node3D>,
}
