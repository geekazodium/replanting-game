use godot::global::randi_range;

use super::cell_support::CellSupport;
use super::cell_update::CellUpdate;
use super::hydration::Hydration;
use super::CellRules;
use super::SimulationCell;
use super::EIGHT_CONNECTED_OFFSETS;
use super::MAX_HYDRATION;
use super::MOVE_FLAG_COPY;

const MAX_GROW_LENGTH: u8 = 28;

#[derive(Clone, Copy, Debug)]
pub struct TreeSpread {
    length: u8,
}

impl CellUpdate for TreeSpread {
    fn update(&mut self, neighbors: [&SimulationCell; 8], this: &mut SimulationCell) {
        if self.length > MAX_GROW_LENGTH {
            this.set_velocity(EIGHT_CONNECTED_OFFSETS[randi_range(0, 2) as usize]);
            this.replace(CellRules::TreeTrunk {
                hydration: Hydration::new(),
                support: CellSupport::new(10),
            });
            return;
        }
        if this.get_hydration() < MAX_HYDRATION - 50 {
            return;
        }
        let mut count: u8 = 0;
        for n in neighbors {
            if n.cell_type_eq(&this) {
                count += 1;
            }
        }

        if !(count <= 2) {
            return;
        }

        this.set_velocity(EIGHT_CONNECTED_OFFSETS[randi_range(0, 2) as usize]);
        self.length += 1;
        this.set_velocity_mode_type(MOVE_FLAG_COPY);
    }
}

impl TreeSpread {
    pub fn new() -> Self {
        Self { length: 0 }
    }
}
