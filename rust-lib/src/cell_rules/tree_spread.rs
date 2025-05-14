use godot::{builtin::Vector2i, global::randi_range};

use crate::cellular_automata_layer::CellDataWrapper;

use super::{cell_update::CellUpdate, hydration::Hydration, CellRules, MAX_HYDRATION};

#[derive(Clone, Copy, Debug)]
pub struct TreeSpread{
    pub(crate) max_neigbours: u8
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

impl CellUpdate for TreeSpread{
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i) {
        let mut count: u8 = 0;
        let original_cell: &CellRules = data.get(position);
        if original_cell.get_hydration() < MAX_HYDRATION - 20{
            return;
        }
        for offset in OFFSETS{
            if data.get(position + offset) == original_cell{
                count += 1;
            }
        }
        if count < self.max_neigbours{
            data.set(position + OFFSETS[randi_range(5, 7) as usize], CellRules::TreeRoot { hydration: Hydration{hydration: 0}, tree: {TreeSpread { max_neigbours: 8 }} });
        }
    }
}
