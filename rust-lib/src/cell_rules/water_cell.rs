use super::{cell_update::CellUpdate, SimulationCell, EIGHT_CONNECTED_OFFSETS};

#[derive(Clone, Copy, Debug)]
pub struct WaterCell{
    pub(crate) pos_x_bias: bool
}

impl CellUpdate for WaterCell{
    fn update(&mut self, neighbors: [&SimulationCell; 8], this: &mut SimulationCell) {
        let dir_bias:i8 = if self.pos_x_bias {-1} else {1};

        let offset_indices = vec![
            6,
            6 + dir_bias,
            6 - dir_bias,
            3 + dir_bias.max(0),
            3 - dir_bias.min(0)
        ];
        for offset_index in offset_indices{
            if neighbors[offset_index as usize].get_weight() > this.get_weight(){
                let offset = EIGHT_CONNECTED_OFFSETS[offset_index as usize];
                if dir_bias * offset.x as i8 == 1{
                    self.pos_x_bias ^= true;
                }
                this.set_velocity(offset);
                return;
            }
        }
    }
}
