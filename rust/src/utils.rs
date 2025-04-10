use godot::prelude::*;

pub fn get_child_of_class<T: Inherits<Node>>(node: Gd<Node>) -> Option<Gd<T>> {
    for child in node.get_children().iter_shared() {
        if let Ok(t) = child.try_cast::<T>() {
            return Some(t);
        }
    }

    None
}
