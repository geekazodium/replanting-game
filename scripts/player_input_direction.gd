extends Node
class_name PlayerInputDirection

@export var move_left_action: String;
@export var move_right_action: String;
@export var move_up_action: String;
@export var move_down_action: String;

var _direction: Vector2 = Vector2(0,0);

@export var character_body: PlatformerCharacterBody;

func _physics_process(_delta: float) -> void:
	if Input.is_action_just_released(self.move_left_action):
		self._direction.x = (Input.is_action_pressed(self.move_right_action) as float);
	if Input.is_action_just_released(self.move_right_action):
		self._direction.x = -(Input.is_action_pressed(self.move_left_action) as float);
	
	if Input.is_action_just_released(self.move_up_action):
		self._direction.y = (Input.is_action_pressed(self.move_down_action) as float);
	if Input.is_action_just_released(self.move_down_action):
		self._direction.y = -(Input.is_action_pressed(self.move_up_action) as float);
		
	if Input.is_action_just_pressed(self.move_left_action):
		self._direction.x = -1;
	if Input.is_action_just_pressed(self.move_right_action):
		self._direction.x = 1;
	
	if Input.is_action_just_pressed(self.move_up_action):
		self._direction.y = -1;
	if Input.is_action_just_pressed(self.move_down_action):
		self._direction.y = 1;
	
	self.character_body.set_input_direction(self._direction);

func get_direction() -> Vector2:
	return self._direction.normalized();
