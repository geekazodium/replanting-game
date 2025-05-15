use super::SimulationCell;

pub trait CellUpdate {
    fn update(&mut self, neighbors: [&SimulationCell; 8], this: &mut SimulationCell);
}
