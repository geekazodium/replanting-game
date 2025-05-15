use std::u16;

use super::cell_update::CellUpdate;
use super::SimulationCell;
use super::EIGHT_CONNECTED_OFFSETS;

#[derive(Clone, Copy, Debug)]
pub struct CellSupport {
    pub(crate) distance_from_solid_h: u16,
    pub(crate) strength: u16,
}

impl CellUpdate for CellSupport {
    fn update(&mut self, neighbors: [&SimulationCell; 8], this: &mut SimulationCell) {
        let mut min_distance_h: u16 = u16::MAX;
        for i in 0..5 {
            let offset = EIGHT_CONNECTED_OFFSETS[i];
            let neighbor = neighbors[i];
            let h_distance = neighbor.get_h_distance();
            min_distance_h = min_distance_h.min(if h_distance == u16::MAX {
                u16::MAX
            } else {
                h_distance + offset.x.abs() as u16
            });
        }
        self.distance_from_solid_h = min_distance_h;
        if self.distance_from_solid_h > self.strength {
            let dir_bias = 1;
            let offset_indicies = [6, 6 + dir_bias, 6 - dir_bias];
            for index in offset_indicies {
                if neighbors[index].get_weight() < this.get_weight() {
                    this.set_velocity(EIGHT_CONNECTED_OFFSETS[index]);
                    return;
                }
            }
        }
    }
}
