extends Sprite2D
class_name PlaceTileCursor

@export var place_tile_button: StringName = "";
@export var break_tile_button: StringName = "";
@export var reachable_raycast: PlayerReachableRaycast;
@export var tile_size: float = 8;
@export var minimum_distance: float = 20;

var selected_tile: Vector2i = Vector2i(3,0);

func _ready() -> void:
	EventBus.player_plant_type_chosen.connect(self.on_place_tile_chosen)

func on_place_tile_chosen(plant_type: PlantType) -> void:
	self.selected_tile = plant_type.tileset_coords;

func _physics_process(_delta: float) -> void:
	var mouse_pos: Vector2 = self.get_global_mouse_position() - self.reachable_raycast.global_position;
	self.global_position = self.reachable_raycast.get_reachable_global_pos(mouse_pos,true);
	
	if Input.is_action_pressed(self.place_tile_button):
		var _position: Vector2 = self.reachable_raycast.get_reachable_global_pos(self.position);
		if _position.distance_to(self.global_position - self.position) < minimum_distance:
			return;
		for x in range(-1,2):
			for y in range(-1,2):
				EventBus.place_tile_attempt.emit(_position + Vector2(x,y) * tile_size, self.selected_tile);
	elif Input.is_action_pressed(self.break_tile_button):
		var _position: Vector2 = self.reachable_raycast.get_reachable_global_pos(self.position,true);
		for x in range(-1,2):
			for y in range(-1,2):
				EventBus.break_tile_attempt.emit(_position + Vector2(x,y) * tile_size);
