use godot::builtin::Vector2i;

use crate::cellular_automata_layer::CellDataWrapper;

pub trait CellUpdate {
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i);
}
