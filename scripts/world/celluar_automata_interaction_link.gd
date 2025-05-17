extends Node
class_name CelluarAutomataInteractionLink

@export var cell_layer: CellularAutomataLayer;

func _ready() -> void:
	EventBus.place_tile_attempt.connect(self.place_tile);
	EventBus.break_tile_attempt.connect(self.break_tile);

func place_tile(global_pos: Vector2, atlas_coords: Vector2i) -> void:
	var tile_pos: Vector2i = self.tile_from_global_pos(global_pos);
	if !self.cell_layer.is_tile_solid(tile_pos):
		self.cell_layer.set_tile(tile_pos, atlas_coords, 0);

func break_tile(global_pos: Vector2) -> void:
	var tile_pos: Vector2i = self.tile_from_global_pos(global_pos);
	if self.cell_layer.is_tile_solid(tile_pos):
		self.cell_layer.set_tile(tile_pos, Vector2i(0,0), 0);

func tile_from_global_pos(global_pos: Vector2) -> Vector2i:
	var rounded_position: Vector2 = floor(
		global_pos/(self.cell_layer.tile_set.tile_size as Vector2) - 
		self.cell_layer.global_position
	);
	return rounded_position as Vector2i;
