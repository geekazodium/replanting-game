use super::cell_update::CellUpdate;
use super::SimulationCell;

#[derive(Clone, Copy, Debug)]
pub struct TreeSpread{
}

impl CellUpdate for TreeSpread{
    fn update(&mut self, _data: [&SimulationCell; 8], _position: &mut SimulationCell) {
        // let original_cell: CellRules = data.get(position).clone();
        // if original_cell.get_hydration() < MAX_HYDRATION - 20{
        //     return;
        // }
        // for i in 0..4{
        //     if i == 3{
        //         data.set(position + EIGHT_CONNECTED_OFFSETS[6], CellRules::TreeSpread { hydration: Hydration::new(), tree: {TreeSpread {}} });
        //         break;
        //     }
        //     if data.get(position + EIGHT_CONNECTED_OFFSETS[i]).to_id() == 6 && randi_range(0, 2) != 0{
        //         data.set(position + EIGHT_CONNECTED_OFFSETS[7 - i], CellRules::TreeSpread { hydration: Hydration::new(), tree: {TreeSpread {}} });
        //         break;
        //     }
        // }
        // if randi_range(0, 5) == 0{
        //     data.set(position + EIGHT_CONNECTED_OFFSETS[randi_range(5, 7) as usize], CellRules::TreeSpread { hydration: Hydration::new(), tree: {TreeSpread {}} });
        // }
        // let mut min_distance_h = u16::MAX;
        // for i in 0..5{
        //     let offset = EIGHT_CONNECTED_OFFSETS[i];
        //     let h_distance = data.get(position + offset).get_support_distance_h();
        //     min_distance_h = min_distance_h.min(if h_distance == u16::MAX {u16::MAX} else {h_distance + offset.x.abs() as u16});
        // }
        // data.set(position, CellRules::TreeTrunk { hydration: Hydration::new(), support: CellSupport{distance_from_solid_h: min_distance_h, strength: 15} });
    }
}
