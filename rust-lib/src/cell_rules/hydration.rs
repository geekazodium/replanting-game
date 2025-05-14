use godot::builtin::Vector2i;

use crate::cellular_automata_layer::CellDataWrapper;

use super::cell_update::CellUpdate;

#[derive(Clone, Copy, Debug)]
pub struct Hydration{
    pub(crate) hydration: u8
}

const OFFSETS: [Vector2i; 8] = [
            Vector2i::new(-1, 1),
            Vector2i::DOWN,
            Vector2i::new(1, 1),
            Vector2i::LEFT,
            Vector2i::RIGHT,
            Vector2i::new(-1, -1),
            Vector2i::UP,
            Vector2i::new(1, -1)
        ];

impl CellUpdate for Hydration{
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i) {
        let mut hydration_max: u8 = 0;
        for offset in OFFSETS{
            hydration_max = hydration_max.max(data.get(position + offset).get_hydration());
        }
        if hydration_max > 0{
            self.hydration = hydration_max - 1;
        }else{
            self.hydration = 0;
        }
    }
}
