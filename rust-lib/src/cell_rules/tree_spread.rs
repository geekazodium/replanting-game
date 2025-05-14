use godot::builtin::Vector2i;
use godot::global::randi_range;

use crate::cellular_automata_layer::CellDataWrapper;

use super::cell_update::CellUpdate;
use super::hydration::Hydration;
use super::CellRules;
use super::EIGHT_CONNECTED_OFFSETS;
use super::MAX_HYDRATION;

#[derive(Clone, Copy, Debug)]
pub struct TreeSpread{
    pub(crate) max_neigbours: u8
}

impl CellUpdate for TreeSpread{
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i) {
        let mut count: u8 = 0;
        let original_cell: &CellRules = data.get(position);
        if original_cell.get_hydration() < MAX_HYDRATION - 20{
            return;
        }
        for offset in EIGHT_CONNECTED_OFFSETS{
            if data.get(position + offset) == original_cell{
                count += 1;
            }
        }
        if count < self.max_neigbours{
            data.set(position + EIGHT_CONNECTED_OFFSETS[randi_range(5, 7) as usize], CellRules::TreeSpread { hydration: Hydration{hydration: 0}, tree: {TreeSpread { max_neigbours: 8 }} });
        }
    }
}
