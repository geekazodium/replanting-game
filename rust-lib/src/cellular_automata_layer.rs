use core::f64;

use godot::builtin::Rect2i;
use godot::builtin::Vector2i;
use godot::classes::ITileMapLayer;
use godot::classes::TileMapLayer;
use godot::global::randf_range;
use godot::obj::Base;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

use crate::cell_rules::CellRules;

#[derive(GodotClass)]
#[class(base = TileMapLayer)]
struct CellularAutomataLayer{
    base: Base<TileMapLayer>,
    cell_data: CellDataWrapper
}

#[godot_api]
impl ITileMapLayer for CellularAutomataLayer{
    fn init(base: Base<TileMapLayer>) -> Self{
        Self{
            base,
            cell_data: CellDataWrapper::new(vec![],  Rect2i::new(Vector2i::new(0, 0), Vector2i::new(0, 0)))
        }
    }
    fn ready(&mut self){
        self.load_tiles_to_wrapper();
    }
    fn physics_process(&mut self, _delta: f64){
        self.update_tiles();
    }
}

impl CellularAutomataLayer{
    fn update_tiles(&mut self){
        let (range_x, range_y) = self.cell_data.get_range();
        let min_position = self.cell_data.get_min_vec2i();

        for y in (0..range_y).rev(){
            let iter = 0..range_x;
            if randf_range(0., 2.) > 1.0{
                for x in iter.rev(){
                    self.update_cell(x+min_position.x, y + min_position.y);
                }
            }else{
                for x in iter{
                    self.update_cell(x+min_position.x, y + min_position.y);
                }
            }
        }

        for y in 0..range_y{
            for x in 0..range_x{
                let tile_pos = Vector2i::new(x, y) + min_position;
                let cell = self.cell_data.get(tile_pos);
                if *cell != CellRules::ForceEmpty{
                    let atlas_coords = cell.to_atlas_coords();
                    if randf_range(0., 1.) < 0.01f64{
                        self.set_cell(tile_pos, atlas_coords, 0);
                    }
                }
            }
        }
    }

    fn set_cell(&mut self, pos: Vector2i, atlas_coords: Vector2i, id: i32){
        self.base_mut().set_cell_ex(pos).atlas_coords(atlas_coords).source_id(id).done();
    }

    fn load_tiles_to_wrapper(&mut self){
        let rect = self.base().get_used_rect();
        let rect_position = rect.position;
        let range = rect.size;
    
        let mut cells_data_vec = vec![];
        for y in 0..range.y{
            for x in 0..range.x{
                let tile_pos = Vector2i::new(x, y)+rect_position;
                cells_data_vec.push(CellRules::from_tile(self.base().get_cell_atlas_coords(tile_pos), self.base().get_cell_source_id(tile_pos)));
            }
        }
        self.cell_data = CellDataWrapper::new(cells_data_vec, rect);
    }
    
    fn update_cell(&mut self, x: i32, y: i32) {
        let cell_rules = self.cell_data.get(Vector2i::new(x, y)).clone();
        let tile_pos = Vector2i::new(x, y);
        cell_rules.update(&mut self.cell_data, tile_pos);
    }
}

pub const TILE_TYPE_DATA_LAYER: &str = "tile_type";
pub const TILE_SIZE: f32 = 64.;

pub struct CellDataWrapper{
    data: Vec<CellRules>,
    width: i32,
    height: i32,
    min_x: i32,
    min_y: i32
}

impl CellDataWrapper{
    fn new(data: Vec<CellRules>, rect: Rect2i) -> Self{
        Self{
            data,
            width: rect.size.x,
            height: rect.size.y,
            min_x: rect.position.x,
            min_y: rect.position.y
        }
    }
    pub fn get(&self, position: Vector2i) -> &CellRules{
        let pos = self.map_global_pos_to_grid(position);
        if pos.x < 0 || pos.x >= self.width{
            return &CellRules::ForceEmpty;
        }
        if pos.y < 0 || pos.y >= self.height{
            return &CellRules::ForceEmpty;
        }

        let index = self.map_vec_to_index(pos);
        &self.data[index]
    }
    pub fn set(&mut self, position: Vector2i, cell: CellRules){
        let pos = self.map_global_pos_to_grid(position);
        if pos.x < 0 || pos.x >= self.width{
            return;
        }
        if pos.y < 0 || pos.y >= self.height{
            return;
        }

        let index = self.map_vec_to_index(pos);
        self.data[index] = cell;
    }
    fn map_vec_to_index(&self, vec: Vector2i)-> usize{
        let x = (vec.x) as usize;
        let cy = ((vec.y) * self.width) as usize;
        x + cy
    }
    fn map_global_pos_to_grid(&self, position: Vector2i) -> Vector2i{
        position - Vector2i{ x:self.min_x, y:self.min_y }
    }
    fn get_range(&self) -> (i32, i32){
        (self.width, self.height)
    }
    fn get_min_vec2i(&self) -> Vector2i{
        return Vector2i::new(self.min_x, self.min_y);
    }
}