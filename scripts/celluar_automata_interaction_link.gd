extends Node

@export var cell_layer: CellularAutomataLayer;

func _ready() -> void:
	EventBus.place_tile_attempt.connect(self.place_tile);

func place_tile(global_pos: Vector2, atlas_coords: Vector2i) -> void:
	var tile_pos: Vector2i = (global_pos as Vector2i)/ cell_layer.tile_set.tile_size;
	self.cell_layer.set_tile(tile_pos, atlas_coords, 0);
