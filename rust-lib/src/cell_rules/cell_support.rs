use std::u16;

use godot::builtin::Vector2i;
use crate::cellular_automata_layer::CellDataWrapper;

use super::cell_update::CellUpdate; 
use super::CellRules;
use super::EIGHT_CONNECTED_OFFSETS;

#[derive(Clone, Copy, Debug)]
pub struct CellSupport{
    pub(crate) distance_from_solid_h: u16,
    pub(crate) strength: u16,
}

impl CellUpdate for CellSupport{
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i) {
        let mut min_distance_h: u16 = u16::MAX;
        for i in 0..5{
            let offset = EIGHT_CONNECTED_OFFSETS[i];
            let h_distance = data.get(position + offset).get_support_distance_h();
            min_distance_h = min_distance_h.min(if h_distance == u16::MAX {u16::MAX} else {h_distance + offset.x.abs() as u16});
        }
        self.distance_from_solid_h = min_distance_h;
        if self.distance_from_solid_h > self.strength{
            let dir_bias = 1;
            let offsets = vec![
                Vector2i::UP,
                Vector2i::UP + Vector2i::LEFT * dir_bias,
                Vector2i::UP + Vector2i::RIGHT * dir_bias,
            ];
            for offset in offsets{
                if *data.get(position - offset) == CellRules::Empty{
                    data.set(position - offset, data.get(position).clone());
                    data.set(position, CellRules::Empty);
                    return;
                }
            }
        }
    }
}
