use godot::builtin::Vector2i;
use godot::global::randi_range;

use crate::cellular_automata_layer::CellDataWrapper;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CellRules{
    Empty,
    ForceEmpty,
    StaticCell{hydration: Hydration},
    Water{water_cell:WaterCell},
    Moss{hydration: Hydration, moss: MossSpread},
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
            1 =>Self::StaticCell{hydration: Hydration { hydration: 0 }},
            2 =>Self::Water{water_cell: WaterCell{}},
            3 =>Self::Moss { hydration: Hydration { hydration: 2 }, moss: MossSpread { energy: 8 } },
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
            Self::Water { water_cell: _ } => 16,
            Self::StaticCell { hydration } => hydration.hydration,
            Self::Moss { hydration, moss: _ } => if hydration.hydration > 3 { hydration.hydration - 3}else{0},
            _default => 0
        }
    }
}

trait CellUpdate {
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i);
}

#[derive(Eq, Clone, Copy, Debug)]
pub struct WaterCell{
}

impl CellUpdate for WaterCell{
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i) {
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
                data.set(position - offset, CellRules::Water { water_cell: *self });
                data.set(position, CellRules::Empty);
                return;
            }
        }
    }
}

impl PartialEq for WaterCell{
    fn eq(&self, _other: &Self) -> bool {
        true
    }
    fn ne(&self, _other: &Self) -> bool {
        false
    }
}

#[derive(Eq, Clone, Copy, Debug)]
pub struct Hydration{
    hydration: u8
}

impl CellUpdate for Hydration{
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i) {
        let offsets = vec![
            Vector2i::UP,
            Vector2i::DOWN,
            Vector2i::LEFT,
            Vector2i::RIGHT
        ];
        let mut hydration_max: u8 = 0;
        for offset in offsets{
            hydration_max = hydration_max.max(data.get(position + offset).get_hydration());
        }
        if hydration_max > 0{
            self.hydration = hydration_max - 1;
        }else{
            self.hydration = 0;
        }
    }
}

impl PartialEq for Hydration{
    fn eq(&self, _other: &Self) -> bool {
        true
    }
    fn ne(&self, _other: &Self) -> bool {
        false
    }
}

#[derive(Eq, Clone, Copy, Debug)]
pub struct MossSpread{
    energy: u8
}

impl CellUpdate for MossSpread{
    fn update(&mut self, data: &mut CellDataWrapper, position: Vector2i) {
        let mut offsets = vec![
            Vector2i::UP,
            Vector2i::DOWN,
            Vector2i::LEFT,
            Vector2i::RIGHT
        ];
        for i in (0..offsets.len()).rev(){
            offsets.swap(i, randi_range(i as i64, 0) as usize);
        }
        
        if (randi_range(2, 200) as u8) < self.energy{
            self.energy = 0;
            for offset in offsets{
                if data.get(position - offset).is_solid(){
                    data.set(position - offset, CellRules::Moss { hydration: Hydration { hydration: 2 }, moss: MossSpread { energy: 0 } });
                    return;
                }
            }
        }else{
            if data.get(position).get_hydration() > 4{
                self.energy += 1;
            }else{
                if self.energy >= 4{
                    self.energy -= 4;
                }else{
                    self.energy = 0;
                }
            }
        }
    }
}

impl PartialEq for MossSpread{
    fn eq(&self, _other: &Self) -> bool {
        true
    }
    fn ne(&self, _other: &Self) -> bool {
        false
    }
}

