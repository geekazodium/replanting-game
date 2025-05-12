use godot::{builtin::Vector2i, global::randi_range};

use crate::cellular_automata_layer::CellDataWrapper;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CellRules{
    Empty,
    ForceEmpty,
    StaticCell,
    Water{water_cell:WaterCell},
}

impl CellRules{
    pub fn from_tile(tile: Vector2i, id: i32) -> Self{
        match id{
            0 => CellRules::from_atlas_coords(tile),
            _default => CellRules::ForceEmpty
        }
    }
    #[allow(unused)]
    fn to_id(&self)->u16{
        match self{
            Self::ForceEmpty=>0,
            Self::Empty=>1,
            Self::StaticCell=>2,
            Self::Water{water_cell}=>3,
        }
    }
    pub fn can_set(&self)-> bool{
        match self {
            Self::ForceEmpty=> false,
            _default=>true
        }
    }
    pub fn to_atlas_coords(&self) -> Vector2i{
        match self{
            Self::ForceEmpty=>panic!("can't set forced empty cell"),
            Self::Empty=>Vector2i::new(0, 0),
            Self::StaticCell=>Vector2i::new(1, 0),
            Self::Water{water_cell: _}=>Vector2i::new(2, 0)
        }
    }
    pub fn from_atlas_coords(coord: Vector2i) -> Self{
        match coord.x {
            0 =>Self::Empty,
            1 =>Self::StaticCell,
            2 =>Self::Water{water_cell: WaterCell { velocity_x: 0, velocity_y: 1 }},
            _default=> Self::ForceEmpty
        }
    }
    pub fn update(&self, cell_data: &mut CellDataWrapper, position: Vector2i){
        match self{
            Self::Water{water_cell}=>{water_cell.update(cell_data, position)}
            _default => {}
        }
    }
}

trait CellUpdate {
    fn update(&self, data: &mut CellDataWrapper, position: Vector2i);
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct WaterCell{
    velocity_x: i32,
    velocity_y: i32,
}

impl CellUpdate for WaterCell{
    fn update(&self, data: &mut CellDataWrapper, position: Vector2i) {
        let mut dir_bias = randi_range(0,1) as i32;
        if dir_bias == 0{
            dir_bias = -1;
        }
        let offsets = vec![
            Vector2i::UP,
            Vector2i::UP + Vector2i::LEFT * dir_bias,
            Vector2i::UP + Vector2i::RIGHT * dir_bias,
            Vector2i::LEFT * dir_bias,
            Vector2i::RIGHT * dir_bias
        ];
        for offset in offsets{
            if *data.get(position - offset) == CellRules::Empty{
                data.set(position - offset, CellRules::Water{water_cell: *self});
                data.set(position, CellRules::Empty);
                return;
            }
        }
    }
}