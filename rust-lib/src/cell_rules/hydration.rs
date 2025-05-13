use godot::builtin::Vector2i;

use crate::cellular_automata_layer::CellDataWrapper;

use super::cell_update::CellUpdate;

#[derive(Clone, Copy, Debug)]
pub struct Hydration{
    pub(crate) hydration: u8
}

impl CellUpdate for Hydration{
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i) {
        let offsets = vec![
            Vector2i::UP,
            Vector2i::DOWN,
            Vector2i::LEFT,
            Vector2i::RIGHT
        ];
        let mut hydration_max: u8 = 0;
        for offset in offsets{
            hydration_max = hydration_max.max(data.get(position + offset).get_hydration());
        }
        if hydration_max > 0{
            self.hydration = hydration_max - 1;
        }else{
            self.hydration = 0;
        }
    }
}
