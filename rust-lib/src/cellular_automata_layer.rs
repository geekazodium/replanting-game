use core::f64;
use std::collections::HashSet;

use godot::builtin::Rect2i;
use godot::builtin::Vector2i;
use godot::classes::Camera2D;
use godot::classes::ITileMapLayer;
use godot::classes::TileMapLayer;
use godot::global::godot_error;
use godot::global::randi_range;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

use crate::cell_rules;
use crate::cell_rules::CellRules;
use crate::cell_rules::SimulationCell;

#[derive(GodotClass)]
#[class(base = TileMapLayer)]
struct CellularAutomataLayer {
    base: Base<TileMapLayer>,
    cell_data: CellDataWrapper,
    #[export]
    camera: Option<Gd<Camera2D>>,
    last_visible_area: Rect2i,
}

#[godot_api]
impl ITileMapLayer for CellularAutomataLayer {
    fn init(base: Base<TileMapLayer>) -> Self {
        Self {
            base,
            cell_data: CellDataWrapper::new(
                vec![],
                Rect2i::new(Vector2i::new(0, 0), Vector2i::new(0, 0)),
            ),
            camera: None,
            last_visible_area: Rect2i::new(Vector2i::ZERO, Vector2i::ZERO),
        }
    }
    fn ready(&mut self) {
        self.load_tiles_to_wrapper();
    }
    fn physics_process(&mut self, _delta: f64) {
        self.update_tiles();
    }
}

#[godot_api]
impl CellularAutomataLayer {
    #[func]
    fn set_tile(&mut self, position: Vector2i, tile_src_pos: Vector2i, tile_src_id: i32) {
        if tile_src_id != 0 {
            godot_error!("multuple tilemap texture sources not implemented!");
        }
        self.cell_data.set(
            position,
            SimulationCell::new(CellRules::from_atlas_coords(tile_src_pos)),
        );
    }
    #[func]
    fn is_tile_solid(&mut self, position: Vector2i) -> bool {
        self.cell_data.get(position).is_solid()
    }
    #[func]
    fn get_energy_generation(&mut self) -> f64{
        let mut amount: f64 = 0.;
        for c in self.cell_data.into_iter(){
            amount += c.get_energy_generation();
        }
        amount
    }
}

impl CellularAutomataLayer {
    fn update_tiles(&mut self) {
        let (range_x, range_y) = self.cell_data.get_range();
        let min_position = self.cell_data.get_min_vec2i();

        let mut update_order = [
            Vector2i::ZERO,
            Vector2i::ONE,
            Vector2i::RIGHT,
            Vector2i::DOWN,
        ];
        if randi_range(0, 1) == 0 {
            update_order.swap(0, 1);
        }
        if randi_range(0, 1) == 0 {
            update_order.swap(2, 3);
        }
        update_order.rotate_left(randi_range(0, 3) as usize);
        for offset in update_order {
            for y in 0..range_y / 2 {
                for x in 0..range_x / 2 {
                    self.update_cell(
                        x * 2 + min_position.x + offset.x,
                        y * 2 + min_position.y + offset.y,
                    );
                }
            }
        }

        for offset in update_order {
            for y in 0..range_y / 2 {
                let r = 0..range_x / 2;
                if randi_range(0, 1) == 1 {
                    for x in r {
                        self.move_cell(
                            x * 2 + min_position.x + offset.x,
                            y * 2 + min_position.y + offset.y,
                        );
                    }
                } else {
                    for x in r.rev() {
                        self.move_cell(
                            x * 2 + min_position.x + offset.x,
                            y * 2 + min_position.y + offset.y,
                        );
                    }
                }
            }
        }

        let vis_area = self.get_visible_tile_area();
        for y in 0..vis_area.size.y {
            for x in 0..vis_area.size.x {
                let tile_pos = Vector2i::new(x, y) + vis_area.position;
                if !self.cell_data.is_position_hash_updated(tile_pos)
                    && self.last_visible_area.contains_point(tile_pos)
                {
                    continue;
                }
                let cell = self.cell_data.get(tile_pos);
                if !cell.cell_type_eq_rules(CellRules::ForceEmpty) {
                    let atlas_coords = cell.to_atlas_coords();
                    self.set_cell(tile_pos, atlas_coords, 0);
                }
            }
        }
        self.last_visible_area = vis_area;
        self.cell_data.reset_hashes_updated();
    }

    fn set_cell(&mut self, pos: Vector2i, atlas_coords: Vector2i, id: i32) {
        if self.base().get_cell_atlas_coords(pos) == atlas_coords {
            return;
        }
        self.base_mut()
            .set_cell_ex(pos)
            .atlas_coords(atlas_coords)
            .source_id(id)
            .done();
    }

    fn load_tiles_to_wrapper(&mut self) {
        let rect = self.base().get_used_rect();
        let rect_position = rect.position;
        let range = rect.size;

        let mut cells_data_vec = vec![];
        for y in 0..range.y {
            for x in 0..range.x {
                let tile_pos = Vector2i::new(x, y) + rect_position;
                let rules = CellRules::from_tile(
                    self.base().get_cell_atlas_coords(tile_pos),
                    self.base().get_cell_source_id(tile_pos),
                );
                cells_data_vec.push(SimulationCell::new(rules));
            }
        }
        self.cell_data = CellDataWrapper::new(cells_data_vec, rect);
    }

    fn move_cell(&mut self, x: i32, y: i32) {
        let tile_pos = Vector2i::new(x, y);
        let mut simulation_cell = self.cell_data.get(tile_pos).clone();
        let v = simulation_cell.get_velocity();
        if v == Vector2i::ZERO {
            return;
        }
        let mut move_to = self.cell_data.get(tile_pos + v).clone();
        if move_to.get_weight() > simulation_cell.get_weight()
            && !simulation_cell.is_move_ignoring_weight()
        {
            return;
        }
        move_to.set_velocity(Vector2i::ZERO);
        if simulation_cell.is_move_mode_swap() {
            self.cell_data.set(tile_pos, move_to);
        }
        simulation_cell.set_velocity(Vector2i::ZERO);
        self.cell_data.set(tile_pos + v, simulation_cell);
    }
    fn update_cell(&mut self, x: i32, y: i32) {
        let tile_pos = Vector2i::new(x, y);
        let mut simulation_cell = self.cell_data.get(tile_pos).clone();
        simulation_cell.update(self.get_cell_neighbors(x, y));
        self.cell_data.set(tile_pos, simulation_cell);
    }
    fn get_cell_neighbors(&self, x: i32, y: i32) -> [&SimulationCell; 8] {
        cell_rules::EIGHT_CONNECTED_OFFSETS
            .map(|offset| self.cell_data.get(Vector2i::new(x, y) + offset))
    }

    fn get_visible_tile_area(&self) -> Rect2i {
        return Rect2i::new(
            Vector2i::new(0,0),
            Vector2i::new(30, 30),
        );
        let camera= self.camera.as_ref().expect("no camera set");
        let camera_rect = camera.get_viewport_rect();
        let mut position = camera.get_global_position();
        let mut size = camera_rect.size;
        size /= camera.get_zoom();
        position -= size / 2.;

        let tile_size = self.base().get_tile_set().unwrap().get_tile_size();
        let mut rect = [
            position.x as i32,
            position.y as i32,
            size.x as i32,
            size.y as i32,
        ];
        for i in 0..4 {
            //assume tiles are square
            rect[i] /= tile_size.x;
        }
        return Rect2i::new(
            Vector2i::new(rect[0] - 1, rect[1] - 1),
            Vector2i::new(rect[2] + 3, rect[3] + 3),
        );
    }
}

pub const TILE_TYPE_DATA_LAYER: &str = "tile_type";

pub struct CellDataWrapper {
    data: Vec<SimulationCell>,
    boundary: SimulationCell,
    updated_hashes: HashSet<i32>,
    width: i32,
    height: i32,
    min_x: i32,
    min_y: i32,
}

impl CellDataWrapper {
    fn new(data: Vec<SimulationCell>, rect: Rect2i) -> Self {
        Self {
            data,
            updated_hashes: HashSet::new(),
            boundary: SimulationCell::new(CellRules::ForceEmpty),
            width: rect.size.x,
            height: rect.size.y,
            min_x: rect.position.x,
            min_y: rect.position.y,
        }
    }
    pub fn get(&self, position: Vector2i) -> &SimulationCell {
        let pos = self.map_global_pos_to_grid(position);
        if pos.x < 0 || pos.x >= self.width {
            return &self.boundary;
        }
        if pos.y < 0 || pos.y >= self.height {
            return &self.boundary;
        }

        let index = self.map_vec_to_index(pos);
        &self.data[index]
    }
    pub fn set(&mut self, position: Vector2i, cell: SimulationCell) {
        let pos = self.map_global_pos_to_grid(position);
        if pos.x < 0 || pos.x >= self.width {
            return;
        }
        if pos.y < 0 || pos.y >= self.height {
            return;
        }

        if !cell.cell_type_eq(self.get(position)) {
            self.set_position_hash_updated(position);
        }
        let index = self.map_vec_to_index(pos);
        self.data[index] = cell;
    }
    fn map_vec_to_index(&self, vec: Vector2i) -> usize {
        let x = (vec.x) as usize;
        let cy = ((vec.y) * self.width) as usize;
        x + cy
    }
    fn map_global_pos_to_grid(&self, position: Vector2i) -> Vector2i {
        position
            - Vector2i {
                x: self.min_x,
                y: self.min_y,
            }
    }
    fn get_range(&self) -> (i32, i32) {
        (self.width, self.height)
    }
    fn get_min_vec2i(&self) -> Vector2i {
        return Vector2i::new(self.min_x, self.min_y);
    }
    fn set_position_hash_updated(&mut self, pos: Vector2i) {
        self.updated_hashes.insert(Self::get_position_hash(pos));
    }
    fn reset_hashes_updated(&mut self) {
        self.updated_hashes.clear();
    }
    fn is_position_hash_updated(&self, pos: Vector2i) -> bool {
        self.updated_hashes.contains(&Self::get_position_hash(pos))
    }
    fn get_position_hash(pos: Vector2i) -> i32 {
        pos.x ^ (pos.y << 16)
    }
}

impl<'a> IntoIterator for &'a CellDataWrapper{
    type IntoIter = std::slice::Iter<'a,SimulationCell>;
    type Item = &'a SimulationCell;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}