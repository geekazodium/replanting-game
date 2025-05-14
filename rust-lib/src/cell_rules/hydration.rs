use godot::builtin::Vector2i;

use crate::cellular_automata_layer::CellDataWrapper;

use super::cell_update::CellUpdate;
use super::EIGHT_CONNECTED_OFFSETS;

#[derive(Clone, Copy, Debug)]
pub struct Hydration{
    pub(crate) hydration: u8
}

impl CellUpdate for Hydration{
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i) {
        let mut hydration_max: u8 = 0;
        for offset in EIGHT_CONNECTED_OFFSETS{
            hydration_max = hydration_max.max(data.get(position + offset).get_hydration());
        }
        if hydration_max > 0{
            self.hydration = hydration_max - 1;
        }else{
            self.hydration = 0;
        }
    }
}
