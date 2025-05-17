extends Sprite2D

@export var place_tile_button: StringName = "";
@export var break_tile_button: StringName = "";

func _physics_process(_delta: float) -> void:
	self.global_position = floor(self.get_global_mouse_position() / 8) * 8 + Vector2.ONE * 4;
	if Input.is_action_pressed(self.place_tile_button):
		EventBus.place_tile_attempt.emit(self.global_position, Vector2i(2,0));
	elif Input.is_action_pressed(self.break_tile_button):
		EventBus.break_tile_attempt.emit(self.global_position);
