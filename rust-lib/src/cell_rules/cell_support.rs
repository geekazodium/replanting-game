use godot::builtin::Vector2i;
use crate::cellular_automata_layer::CellDataWrapper;

use super::cell_update::CellUpdate; 
use super::EIGHT_CONNECTED_OFFSETS;

#[derive(Clone, Copy, Debug)]
pub struct CellSupport{
    pub(crate) distance_from_solid: u16,
    pub(crate) strength: u16,
}

impl CellUpdate for CellSupport{
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i) {
        for i in EIGHT_CONNECTED_OFFSETS{
            panic!("not implemented")
        }
    }
}
