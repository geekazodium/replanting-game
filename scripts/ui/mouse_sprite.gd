extends Sprite2D
class_name PlaceTileCursor

@export var place_tile_button: StringName = "";
@export var break_tile_button: StringName = "";
@export var reachable_raycast: PlayerReachableRaycast;
@export var tile_size: float = 8;
@export var minimum_distance: float = 20;

func _physics_process(_delta: float) -> void:
	self.global_position = floor(self.get_global_mouse_position() / tile_size) * tile_size + Vector2.ONE * 4;
	
	self.global_position = self.reachable_raycast.get_reachable_global_pos(self.position,true);
	
	if Input.is_action_pressed(self.place_tile_button):
		var _position: Vector2 = self.reachable_raycast.get_reachable_global_pos(self.position);
		if _position.distance_to(self.global_position - self.position) < minimum_distance:
			return;
		for x in range(-1,2):
			for y in range(-1,2):
				EventBus.place_tile_attempt.emit(_position + Vector2(x,y) * tile_size, Vector2i(3,0));
	elif Input.is_action_pressed(self.break_tile_button):
		var _position: Vector2 = self.reachable_raycast.get_reachable_global_pos(self.position,true);
		for x in range(-1,2):
			for y in range(-1,2):
				EventBus.break_tile_attempt.emit(_position + Vector2(x,y) * tile_size);
