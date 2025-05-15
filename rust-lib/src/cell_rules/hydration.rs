use super::cell_update::CellUpdate;
use super::SimulationCell;

#[derive(Clone, Copy, Debug)]
pub struct Hydration{
    hydration: u8
}

impl CellUpdate for Hydration{
    fn update(&mut self, neighbors: [&SimulationCell; 8], _this: &mut SimulationCell) {
        let mut hydration_max: u8 = 0;
        for n in neighbors{
            hydration_max = n.get_hydration();
        }
        if hydration_max > 0{
            self.hydration = hydration_max - 1;
        }else{
            self.hydration = 0;
        }
    }
}

impl Hydration{
    pub fn new() -> Self{
        Self { hydration: 0 }
    }
    pub fn get(&self) -> u8{
        self.hydration
    }
}