use core::f64;
use std::collections::HashSet;

use godot::builtin::Rect2i;
use godot::builtin::Vector2i;
use godot::classes::Camera2D;
use godot::classes::ITileMapLayer;
use godot::classes::TileMapLayer;
use godot::global::randf_range;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

use crate::cell_rules::CellRules;

#[derive(GodotClass)]
#[class(base = TileMapLayer)]
struct CellularAutomataLayer{
    base: Base<TileMapLayer>,
    cell_data: CellDataWrapper,
    #[export]
    camera: Option<Gd<Camera2D>>,
    last_visible_area: Rect2i
}

#[godot_api]
impl ITileMapLayer for CellularAutomataLayer{
    fn init(base: Base<TileMapLayer>) -> Self{
        Self{
            base,
            cell_data: CellDataWrapper::new(vec![],  Rect2i::new(Vector2i::new(0, 0), Vector2i::new(0, 0))),
            camera:None,
            last_visible_area: Rect2i::new(Vector2i::ZERO, Vector2i::ZERO)
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

        let vis_area = self.get_visible_tile_area();
        for y in 0..vis_area.size.y{
            for x in 0..vis_area.size.x{
                let tile_pos = Vector2i::new(x, y) + vis_area.position;
                if !self.cell_data.is_position_hash_updated(tile_pos) && self.last_visible_area.contains_point(tile_pos){
                    continue;
                }
                self.cell_data.add_chunk_updating(tile_pos);
                let cell = self.cell_data.get(tile_pos);
                if *cell != CellRules::ForceEmpty{
                    let atlas_coords = cell.to_atlas_coords();
                    self.set_cell(tile_pos, atlas_coords, 0);
                }
            }
        }
        self.last_visible_area = vis_area;
        self.cell_data.reset_hashes_updated();
        self.cell_data.update_chunks_updating();
    }

    fn set_cell(&mut self, pos: Vector2i, atlas_coords: Vector2i, id: i32){
        if self.base().get_cell_atlas_coords(pos) == atlas_coords && self.base().get_cell_source_id(pos) == id{
            return;
        }
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
                let new_tile = CellRules::from_tile(self.base().get_cell_atlas_coords(tile_pos), self.base().get_cell_source_id(tile_pos));
                cells_data_vec.push(new_tile);
            }
        }
        self.cell_data = CellDataWrapper::new(cells_data_vec, rect);
        self.cell_data.add_chunk_updating(Vector2i::new(0, 0));
    }
    
    fn update_cell(&mut self, x: i32, y: i32) {
        if !self.cell_data.is_chunk_updating(Vector2i::new(x,y)){
            return;
        }
        let mut cell_rules = self.cell_data.get(Vector2i::new(x, y)).clone();
        let tile_pos = Vector2i::new(x, y);
        cell_rules.update(&mut self.cell_data, tile_pos);
    }
    
    fn get_visible_tile_area(&self) -> Rect2i{        
        let camera = self.camera.as_ref().expect("no camera set");
        let camera_rect = camera.get_viewport_rect();
        let mut position = camera.get_global_position();
        let mut size = camera_rect.size;
        size /= camera.get_zoom();
        position -= size / 2.;
        
        let tile_size = self.base().get_tile_set().unwrap().get_tile_size();
        let mut rect = [position.x as i32, position.y as i32, size.x as i32, size.y as i32];
        for i in 0..4{
            //assume tiles are square
            rect[i]/=tile_size.x;
        }
        return Rect2i::new(Vector2i::new(rect[0] - 1, rect[1] - 1), Vector2i::new(rect[2] + 3, rect[3] + 3));
    }
}

pub const TILE_TYPE_DATA_LAYER: &str = "tile_type";
pub const CHUNK_SIZE: i32 = 64;

pub struct CellDataWrapper{
    data: Vec<CellRules>,
    updated_hashes: HashSet<i32>,
    processing_chunks: UpdatingChunkDoubleBuffer,
    width: i32,
    height: i32,
    min_x: i32,
    min_y: i32
}

impl CellDataWrapper{
    fn new(data: Vec<CellRules>, rect: Rect2i) -> Self{
        Self{
            data,
            updated_hashes: HashSet::new(),
            processing_chunks: UpdatingChunkDoubleBuffer::new(),
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
        self.get_unchecked_local_space(pos)
    }
    fn get_unchecked_local_space(&self, pos: Vector2i) -> &CellRules{
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

        if cell != *self.get_unchecked_local_space(pos){
            self.set_position_hash_updated(position);
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
    fn set_position_hash_updated(&mut self, position: Vector2i){
        self.updated_hashes.insert(Self::get_position_hash(position));
    }
    fn reset_hashes_updated(&mut self){
        self.updated_hashes.clear();
    }
    fn is_position_hash_updated(&self, position: Vector2i) -> bool{
        self.updated_hashes.contains(&Self::get_position_hash(position))
    }
    fn get_position_hash(global_position: Vector2i) -> i32{
        global_position.x ^ (global_position.y<<16)
    }
    fn get_chunk_key(global_position: Vector2i) -> u64{
        global_position.x.div_euclid(CHUNK_SIZE) as u64 | ((global_position.y.div_euclid(CHUNK_SIZE) as u64) << 32)
    }
    fn is_chunk_updating(&self, global_position: Vector2i) -> bool{
        self.processing_chunks.contains(&Self::get_chunk_key(global_position))
    }
    fn add_chunk_updating(&mut self, global_position: Vector2i){
        self.processing_chunks.insert(Self::get_chunk_key(global_position));
    }
    fn update_chunks_updating(&mut self){
        self.processing_chunks.swap();
    }
}

struct UpdatingChunkDoubleBuffer{
    a:HashSet<u64>,
    b:HashSet<u64>, 
    a_used:bool
}

impl UpdatingChunkDoubleBuffer{
    fn new() -> Self{
        Self{a:HashSet::new(), b:HashSet::new(), a_used:false}
    }
    fn contains(&self, val: &u64) -> bool{
        if self.a_used {&self.a} else {&self.b}.contains(val)
    }
    fn insert(&mut self, val: u64){
        if !self.a_used {&mut self.a} else {&mut self.b}.insert(val);
    }
    fn swap(&mut self){
        if self.a_used {&mut self.a} else {&mut self.b}.clear();
        self.a_used ^= true;
    }
}