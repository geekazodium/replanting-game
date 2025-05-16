use super::SimulationCell;
use super::EIGHT_CONNECTED_OFFSETS;
use super::MAX_HYDRATION;
use super::MOVE_FLAG_COPY;

use godot::global::randi_range;

use super::cell_update::CellUpdate;

#[derive(Clone, Copy, Debug)]
pub struct MossSpread {
    energy: u8,
}

impl CellUpdate for MossSpread {
    fn update(&mut self, neighbors: [&SimulationCell; 8], this: &mut SimulationCell) {
        let mut offset_indicies = [1, 3, 4, 6];
        for i in (0..offset_indicies.len()).rev() {
            let other_index = randi_range(i as i64, 0) as usize;
            offset_indicies.swap(i, other_index);
        }

        if (randi_range(2, 200) as u8) < self.energy {
            self.energy = 0;
            for neighbor_idx in offset_indicies {
                if neighbors[neighbor_idx].is_solid() {
                    this.set_velocity(EIGHT_CONNECTED_OFFSETS[neighbor_idx]);
                    this.set_velocity_mode_type(MOVE_FLAG_COPY);
                    return;
                }
            }
        } else {
            if this.get_hydration() > MAX_HYDRATION - 6 {
                self.energy += 1;
            } else {
                if self.energy >= 4 {
                    self.energy -= 4;
                } else {
                    self.energy = 0;
                }
            }
        }
    }
}

impl MossSpread{
    pub fn new() -> Self{
        Self { energy: 2 }
    }
}