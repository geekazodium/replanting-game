use godot::init::gdextension;
use godot::init::ExtensionLibrary;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

pub mod cell_rules;
pub mod cellular_automata_layer;
