use super::hydration::Hydration;
use super::MAX_HYDRATION;

use super::CellRules;

use godot::global::randi_range;

use godot::builtin::Vector2i;

use crate::cellular_automata_layer::CellDataWrapper;

use super::cell_update::CellUpdate;

#[derive(Clone, Copy, Debug)]
pub struct MossSpread{
    pub(crate) energy: u8
}

impl CellUpdate for MossSpread{
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i) {
        let mut offsets = vec![
            Vector2i::UP,
            Vector2i::DOWN,
            Vector2i::LEFT,
            Vector2i::RIGHT
        ];
        for i in (0..offsets.len()).rev(){
            offsets.swap(i, randi_range(i as i64, 0) as usize);
        }
    
        if (randi_range(2, 200) as u8) < self.energy{
            self.energy = 0;
            for offset in offsets{
                if data.get(position - offset).is_solid(){
                    data.set(position - offset, CellRules::Moss { hydration: Hydration { hydration: 2 }, moss: MossSpread { energy: 0 } });
                    return;
                }
            }
        }else{
            if data.get(position).get_hydration() > MAX_HYDRATION - 6{
                self.energy += 1;
            }else{
                if self.energy >= 4{
                    self.energy -= 4;
                }else{
                    self.energy = 0;
                }
            }
        }
    }
}
