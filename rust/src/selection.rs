use godot::{classes::Engine, prelude::*};

/// A list of `Selectable` nodes that were added to the scene tree.
#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct SelectionSignalBus {
    base: Base<Object>,
}

/// Simple singleton allowing for `Selectable` node registration.
#[godot_api]
impl SelectionSignalBus {
    pub const NAME: &'static str = "SelectionSignalBus";

    /// Emit a signal when a `Selectable` node enters the scene tree.
    #[signal]
    pub fn selectable_entered_tree(selectable: i32);

    /// Emit a signal when a `Selectable` node exits the scene tree.
    #[signal]
    pub fn selectable_exited_tree(selectable: i32);

    /// Get singleton instance
    ///
    /// Panics if singleton is not registered.
    /// TODO: Return Result instead of panicking
    #[func]
    pub fn singleton() -> Gd<SelectionSignalBus> {
        let engine = Engine::singleton();
        let Some(singleton) = engine.get_singleton(Self::NAME) else {
            godot_error!("Failed to get {} singleton", Self::NAME);
            panic!("Failed to get {} singleton", Self::NAME);
        };

        match singleton.try_cast::<SelectionSignalBus>() {
            Ok(s) => s,
            Err(err) => {
                panic!("Failed to cast {} singleton, err: {}", Self::NAME, err);
            }
        }
    }
}

/// Add `select` capability to parent node.
#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct Selectable {
    /// Override node that should be selected when `shapes` is clicked.
    ///
    /// Parent node will be selected if unset.
    #[export]
    pub select_node_override: Option<Gd<Node3D>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for Selectable {
    fn enter_tree(&mut self) {
        // FIXME: Investigate why cannot emit this via `selectable_singleton.signals()`
        let mut selectable_singleton = SelectionSignalBus::singleton();
        selectable_singleton.emit_signal("selectable_entered_tree", &[Variant::from(self.to_gd())]);
    }

    fn exit_tree(&mut self) {
        // FIXME: Investigate why cannot emit this via `selectable_singleton.signals()`
        let mut selectable_singleton = SelectionSignalBus::singleton();
        selectable_singleton.emit_signal("selectable_exited_tree", &[Variant::from(self.to_gd())]);
    }
}
