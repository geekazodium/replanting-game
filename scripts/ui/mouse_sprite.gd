extends Sprite2D

@export var place_tile_button: StringName = "";
var last_place_pos: Vector2;
var last_placing: bool;

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	self.global_position = floor(self.get_global_mouse_position() / 8) * 8 + Vector2.ONE * 4;

func _physics_process(_delta: float) -> void:
	self.global_position = floor(self.get_global_mouse_position() / 8) * 8;
	if Input.is_action_pressed(self.place_tile_button):
		if !self.last_placing:
			self.last_place_pos = self.global_position;
		var moved_by: Vector2 = self.last_place_pos - self.global_position;
		var distance: float = moved_by.length();
		while distance != 0:
			distance = move_toward(distance, 0, 4);
			EventBus.place_tile_attempt.emit(self.global_position + distance * moved_by.normalized(), Vector2i(2,0));
		self.last_place_pos = self.global_position;
		self.last_placing = true;
	else:
		self.last_placing = false;
