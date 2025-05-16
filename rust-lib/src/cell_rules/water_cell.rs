use godot::global::randi_range;

use super::cell_update::CellUpdate;
use super::SimulationCell;
use super::EIGHT_CONNECTED_OFFSETS;

#[derive(Clone, Copy, Debug)]
pub struct WaterCell {
    pos_x_bias: bool,
}

impl CellUpdate for WaterCell {
    fn update(&mut self, neighbors: [&SimulationCell; 8], this: &mut SimulationCell) {
        let x_dir_pref = if self.pos_x_bias { 1 } else { -1 };
        let mut offset_indices = vec![6, 7, 5, 4, 3];
        if !self.pos_x_bias {
            offset_indices.swap(1, 2);
            offset_indices.swap(3, 4);
        }
        for offset_index in offset_indices {
            if neighbors[offset_index].get_weight() < this.get_weight() {
                let offset = EIGHT_CONNECTED_OFFSETS[offset_index];
                if x_dir_pref * offset.x == -1 {
                    self.pos_x_bias = !self.pos_x_bias;
                }
                this.set_velocity(offset);
                return;
            }
        }
    }
}

impl WaterCell {
    pub fn new() -> Self {
        Self {
            pos_x_bias: randi_range(0, 1) == 1,
        }
    }
}
