use godot::builtin::Vector2i;
use hydration::Hydration;
use moss_spread::MossSpread;
use tile_velocity_wrapper::VelocityWrapper;
use tree_spread::TreeSpread;
use water_cell::WaterCell;

use crate::cell_rules::cell_update::CellUpdate;

mod cell_support;
mod cell_update;
mod hydration;
mod moss_spread;
mod tree_spread;
mod water_cell;

pub const MAX_HYDRATION: u8 = 128;

pub const MOVE_FLAG_SWAP: i8 = 0b0010000;
pub const MOVE_FLAG_COPY: i8 = 0b0100000;
pub const MOVE_FLAG_IGNORE_WEIGHT: i8 = 0b1000000;

pub const EIGHT_CONNECTED_OFFSETS: [Vector2i; 8] = [
    Vector2i::new(-1, -1),
    Vector2i::UP,
    Vector2i::new(1, -1),
    Vector2i::LEFT,
    Vector2i::RIGHT,
    Vector2i::new(-1, 1),
    Vector2i::DOWN,
    Vector2i::new(1, 1),
];

#[derive(Clone, Copy)]
pub struct SimulationCell {
    rules: CellRules,
    velocity: VelocityWrapper,
}

impl SimulationCell {
    pub fn new(rules: CellRules) -> Self {
        Self {
            rules,
            velocity: VelocityWrapper::new(),
        }
    }
    pub fn update(&mut self, neighbors: [&SimulationCell; 8]) {
        let mut rules = self.rules.clone();
        rules.update(neighbors, self);
        if self.rules.to_id() == rules.to_id() {
            self.rules = rules;
        }
    }
    pub fn is_solid(&self) -> bool {
        self.rules.is_solid()
    }
    pub fn get_weight(&self) -> u8 {
        self.rules.get_weight()
    }
    fn get_h_distance(&self) -> u16 {
        self.rules.get_support_distance_h()
    }
    fn get_hydration(&self) -> u8 {
        self.rules.get_hydration()
    }
    pub fn cell_type_eq(&self, other: &Self) -> bool {
        self.cell_type_eq_rules(other.rules)
    }
    pub fn cell_type_eq_rules(&self, other: CellRules) -> bool {
        self.rules.to_id() == other.to_id()
    }
    pub fn get_velocity(&mut self) -> Vector2i {
        self.velocity.get_velocity()
    }
    pub fn set_velocity(&mut self, velocity: Vector2i) {
        self.velocity.set_velocity(velocity);
    }
    pub fn set_velocity_mode_type(&mut self, type_flag: i8) {
        self.velocity.set_velocity_mode_type(type_flag);
    }
    pub fn is_move_ignoring_weight(&self) -> bool {
        self.velocity.is_move_ignoring_weight()
    }
    pub fn replace(&mut self, rules: CellRules) {
        self.rules = rules;
    }
    pub fn is_move_mode_copy(&self) -> bool {
        self.velocity.is_move_mode_copy()
    }
    pub fn is_move_mode_swap(&self) -> bool {
        self.velocity.is_move_mode_swap()
    }
    pub fn to_atlas_coords(&self) -> Vector2i {
        self.rules.to_atlas_coords()
    }
    pub fn get_energy_generation(&self) -> f64 {
        self.rules.get_energy_generation()
    }
}

mod tile_velocity_wrapper;

#[derive(Clone, Debug, Copy)]
pub enum CellRules {
    Empty,
    ForceEmpty,
    StaticCell {
        hydration: Hydration,
    },
    Water {
        water_cell: WaterCell,
    },
    Moss {
        hydration: Hydration,
        moss: MossSpread,
    },
    TreeSpread {
        hydration: Hydration,
        tree: TreeSpread,
    },
    TreeLeaves {
        hydration: Hydration,
    },
}

impl CellRules {
    pub fn from_tile(tile: Vector2i, id: i32) -> Self {
        match id {
            0 => CellRules::from_atlas_coords(tile),
            _default => CellRules::ForceEmpty,
        }
    }
    fn to_id(&self) -> u16 {
        match self {
            Self::ForceEmpty => 0,
            Self::Empty => 1,
            Self::StaticCell { hydration: _ } => 2,
            Self::Water { water_cell: _ } => 3,
            Self::Moss {
                hydration: _,
                moss: _,
            } => 4,
            Self::TreeSpread {
                hydration: _,
                tree: _,
            } => 5,
            Self::TreeLeaves { hydration: _ } => 6,
        }
    }
    pub fn can_set(&self) -> bool {
        match self {
            Self::ForceEmpty => false,
            _default => true,
        }
    }
    pub fn get_energy_generation(&self) -> f64{
        match self{
            Self::Moss { hydration: _, moss: _ } => 1.,
            Self::TreeLeaves { hydration: _ } => 1.5,
            _default => 0.
        }
    }
    pub fn get_weight(&self) -> u8 {
        match self {
            Self::ForceEmpty => u8::MAX,
            Self::StaticCell { hydration: _ } => 128,
            Self::Moss {
                hydration: _,
                moss: _,
            } => 100,
            Self::Water { water_cell: _ } => 70,
            Self::TreeSpread {
                hydration: _,
                tree: _,
            } => 127,
            Self::TreeLeaves { hydration: _ } => 120,
            Self::Empty => 50,
        }
    }
    pub fn to_atlas_coords(&self) -> Vector2i {
        match self {
            Self::ForceEmpty => panic!("can't set forced empty cell"),
            Self::Empty => Vector2i::new(0, 0),
            Self::StaticCell { hydration: _ } => Vector2i::new(1, 0),
            Self::Water { water_cell: _ } => Vector2i::new(2, 0),
            Self::Moss {
                hydration: _,
                moss: _,
            } => Vector2i { x: 3, y: 0 },
            Self::TreeSpread {
                hydration: _,
                tree: _,
            } => Vector2i { x: 4, y: 0 },
            Self::TreeLeaves { hydration: _ } => Vector2i { x: 5, y: 0 },
        }
    }
    pub fn from_atlas_coords(coord: Vector2i) -> Self {
        match coord.x {
            0 => Self::Empty,
            1 => Self::StaticCell {
                hydration: Hydration::new(),
            },
            2 => Self::Water {
                water_cell: WaterCell::new(),
            },
            3 => Self::Moss {
                hydration: Hydration::new(),
                moss: MossSpread::new(),
            },
            4 => Self::TreeSpread {
                hydration: Hydration::new(),
                tree: TreeSpread::new(),
            },
            5 => Self::TreeLeaves {
                hydration: Hydration::new(),
            },
            _default => Self::ForceEmpty,
        }
    }
    pub fn is_solid(&self) -> bool {
        match self {
            Self::StaticCell { hydration: _ } => true,
            Self::TreeLeaves { hydration: _ } => true,
            Self::Moss {
                hydration: _,
                moss: _,
            } => true,
            Self::TreeSpread {
                hydration: _,
                tree: _,
            } => true,
            _default => false,
        }
    }
    pub fn update(&mut self, neighbors: [&SimulationCell; 8], this_cell: &mut SimulationCell) {
        match self {
            Self::StaticCell { hydration } => {
                hydration.update(neighbors, this_cell);
            }
            Self::Water { water_cell } => water_cell.update(neighbors, this_cell),
            Self::Moss { hydration, moss } => {
                hydration.update(neighbors, this_cell);
                moss.update(neighbors, this_cell);
            }
            Self::TreeSpread { hydration, tree } => {
                hydration.update(neighbors, this_cell);
                tree.update(neighbors, this_cell);
            }
            Self::TreeLeaves { hydration } => {
                hydration.update(neighbors, this_cell);
            }
            _default => {
                return;
            }
        };
    }
    pub fn get_hydration(&self) -> u8 {
        match self {
            Self::Water { water_cell: _ } => MAX_HYDRATION,
            Self::StaticCell { hydration } => hydration.get(),
            Self::Moss { hydration, moss: _ } => {
                if hydration.get() > 2 {
                    hydration.get() - 2
                } else {
                    0
                }
            }
            Self::TreeSpread { hydration, tree: _ } => hydration.get(),
            Self::TreeLeaves { hydration } => hydration.get(),
            _default => 0,
        }
    }
    pub fn get_support_distance_h(&self) -> u16 {
        match self {
            Self::StaticCell { hydration: _ } => 0,
            _default => u16::MAX,
        }
    }
}

impl PartialEq for CellRules {
    fn eq(&self, other: &Self) -> bool {
        self.to_id() == other.to_id()
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
