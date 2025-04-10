use godot::{classes::Engine, prelude::*};
use selection::SelectionSignalBus;

pub mod selection;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {
    fn on_level_init(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }
        Engine::singleton()
            .register_singleton(SelectionSignalBus::NAME, &SelectionSignalBus::new_alloc());
    }

    fn on_level_deinit(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }

        let mut engine = Engine::singleton();
        let Some(singleton) = engine.get_singleton(SelectionSignalBus::NAME) else {
            godot_error!("Failed to get {} singleton", SelectionSignalBus::NAME);
            return;
        };
        engine.unregister_singleton(SelectionSignalBus::NAME);
        singleton.free();
    }
}
