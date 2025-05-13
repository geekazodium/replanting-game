use super::CellRules;
use super::cell_update::CellUpdate;
use godot::builtin::Vector2i;

use crate::cellular_automata_layer::CellDataWrapper;

#[derive(Clone, Copy, Debug)]
pub struct WaterCell{
    pub(crate) pos_x_bias: bool
}

impl CellUpdate for WaterCell{
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i) {
        let dir_bias = if self.pos_x_bias {1} else {-1};
        let offsets = vec![
            Vector2i::UP,
            Vector2i::UP + Vector2i::LEFT * dir_bias,
            Vector2i::UP + Vector2i::RIGHT * dir_bias,
            Vector2i::LEFT * dir_bias,
            Vector2i::RIGHT * dir_bias
        ];
        for offset in offsets{
            if *data.get(position - offset) == CellRules::Empty{
                if dir_bias * offset.x == 1{
                    self.pos_x_bias ^= true;
                }
                data.set(position - offset, CellRules::Water { water_cell: *self });
                data.set(position, CellRules::Empty);
                return;
            }
        }
    }
}
