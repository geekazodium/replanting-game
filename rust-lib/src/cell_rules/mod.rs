use godot::builtin::GString;
use godot::builtin::Vector2i;
use godot::classes::TileData;
use godot::obj::Gd;

use crate::cellular_automata_layer::TILE_TYPE_DATA_LAYER;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CellRules{
    Empty,
    ForceEmpty,
    BasicFilled,
    PermaCell,
}

impl CellRules{
    pub fn from_tile(tile: Option<Gd<TileData>>) -> Self{
        let layer_name: GString = TILE_TYPE_DATA_LAYER.into();
        if tile.is_none(){
            return CellRules::ForceEmpty;
        }
        let id:u16 = tile.unwrap().get_custom_data(layer_name.clone()).to();
        CellRules::from_id(id)
    }
    #[allow(unused)]
    fn to_id(&self)->u16{
        match self{
            Self::ForceEmpty=>0,
            Self::Empty=>1,
            Self::BasicFilled=>2,
            Self::PermaCell=>3,
        }
    }
    fn to_cost(&self)->i32{
        match self{
            Self::Empty => 0,
            Self::BasicFilled => 6,
            Self::PermaCell => panic!("user probably shouldn't be able to place these, too op"),
            Self::ForceEmpty => 0
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
            Self::BasicFilled=>Vector2i::new(1, 0),
            Self::PermaCell=>Vector2i::new(2, 0)
        }
    }
    fn from_id(id: u16) -> Self{
        match id {
            0=>Self::ForceEmpty,
            1=>Self::Empty,
            2=>Self::BasicFilled,
            3=>Self::PermaCell,
            _default=> panic!("invalid id")
        }
    }
    pub fn next_cell(&self,neighbors: &Vec<CellRules>) -> Self{
        match self{
            Self::Empty=>{
                if Self::count_non_empty(neighbors)==3{
                    return Self::BasicFilled;
                }
                Self::Empty
            }
            Self::BasicFilled=>{
                let c = Self::count_non_empty(neighbors);
                if c<=3 && c>=2{
                    return Self::BasicFilled;
                }
                Self::Empty
            }
            Self::ForceEmpty=>Self::ForceEmpty,
            Self::PermaCell=>Self::PermaCell
        }
    }
    #[allow(unused)]
    fn user_replaceable(&self)-> bool{
        match self {
            Self::Empty=>true,
            Self::BasicFilled=>true,
            _default=>false
        }
    }
    fn count_non_empty(neighbors: &Vec<CellRules>) -> u8{
        let mut c:u8 = 8;
        for n in neighbors.iter(){
            if *n == Self::Empty || *n == Self::ForceEmpty{
                c -=1;
            }
        }
        c
    }
}

struct WaterCell{
    velocity_x: i32,
    velocity_y: i32,
}