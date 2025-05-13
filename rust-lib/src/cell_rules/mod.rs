use godot::builtin::Vector2i;
use godot::global::randi_range;

use crate::cell_rules::cell_update::CellUpdate;
use crate::cellular_automata_layer::CellDataWrapper;

pub const MAX_HYDRATION: u8 = 32;

#[derive(Clone, Debug, Copy)]
pub enum CellRules{
    Empty,
    ForceEmpty,
    StaticCell{hydration: hydration::Hydration},
    Water{water_cell:water_cell::WaterCell},
    Moss{hydration: hydration::Hydration, moss: moss_spread::MossSpread},
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
            Self::StaticCell{hydration} =>2,
            Self::Water{water_cell}=>3,
            Self::Moss { hydration, moss } => 4,
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
            Self::StaticCell{hydration: _}=>Vector2i::new(1, 0),
            Self::Water{water_cell: _}=>Vector2i::new(2, 0),
            Self::Moss { hydration:_, moss: _}=>Vector2i { x: 3, y: 0 }
        }
    }
    pub fn from_atlas_coords(coord: Vector2i) -> Self{
        match coord.x {
            0 =>Self::Empty,
            1 =>Self::StaticCell{hydration: hydration::Hydration { hydration: 0 }},
            2 =>Self::Water{water_cell: water_cell::WaterCell{pos_x_bias: randi_range(0,1) == 1}},
            3 =>Self::Moss { hydration: hydration::Hydration { hydration: 2 }, moss: moss_spread::MossSpread { energy: 8 } },
            _default=> Self::ForceEmpty
        }
    }
    pub fn is_solid(&self) -> bool{
        match self{
            Self::StaticCell { hydration: _} => true,
            Self::Moss { hydration: _, moss: _} => true,
            _default => false
        }
    }
    pub fn update(&mut self, cell_data: &mut CellDataWrapper, position: Vector2i){
        match self{
            Self::StaticCell { hydration }=>{
                hydration.update(cell_data, position);
                cell_data.set(position, self.clone());
            },
            Self::Water{water_cell}=>{water_cell.update(cell_data, position)},
            Self::Moss { hydration, moss } => {
                hydration.update(cell_data, position);
                moss.update(cell_data, position);
                cell_data.set(position, self.clone());
            },
            _default => {return;}
        };
    }
    pub fn get_hydration(&self) -> u8{
        match self {
            Self::Water { water_cell: _ } => MAX_HYDRATION,
            Self::StaticCell { hydration } => hydration.hydration,
            Self::Moss { hydration, moss: _ } => if hydration.hydration > 2 { hydration.hydration - 2}else{0},
            _default => 0
        }
    }
}

impl PartialEq for CellRules{
    fn eq(&self, other: &Self) -> bool {
        self.to_id() == other.to_id()
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

mod cell_update;
mod water_cell;
mod hydration;
mod moss_spread;