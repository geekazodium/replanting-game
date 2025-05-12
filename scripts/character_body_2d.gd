extends CharacterBody2D
class_name PlatformerCharacterBody

@export var horizontal_accel_speed: float = 1500;
@export var max_walk_speed: float = 300;
@export var jump_y_vel: float = 500;
@export var reverse_boost: float = 10;
@export var air_friction: float = .1;
@export var leading_jump_buffer: float = .125;
var jump_timer: float;
@export var trailing_jump_buffer: float = .1;
var ground_timer: float;

@export var lock_acceleration_timer: Timer;

var left_direction: Vector2: get = get_left;
var input_direction: Vector2 = Vector2.ZERO;
var instant_acceleration: Vector2 = Vector2.ZERO;
var acceleration: Vector2 = Vector2.ZERO;

signal jumped();

func set_input_direction(direction: Vector2) -> void:
	self.input_direction = direction;

func jump() -> void:
	self.jump_timer = self.leading_jump_buffer;

func add_instant_acceleration(acceleration: Vector2) -> void:
	self.instant_acceleration += acceleration;

func add_acceleration(accel: Vector2) -> void:
	self.acceleration += accel;

func _physics_process(delta: float) -> void:
	if self.jump_timer > 0.:
		self.jump_timer -= delta;
		
	if self.ground_timer > 0.:
		self.ground_timer -= delta;
	
	if self.is_on_floor():
		self.ground_timer = self.trailing_jump_buffer;
	
	if !self.lock_acceleration_timer.is_stopped():
		self.move_and_slide();
		self.set_velocity(self.velocity + self.instant_acceleration);
		self.instant_acceleration = Vector2.ZERO;
		return;
	
	var temp_velocity = self.get_velocity();
	
	var delta_v = Vector2(0., 0.);
	var x_axis = self.input_direction.x;
	if x_axis != 0:
		delta_v += self.get_walk_force(temp_velocity, x_axis, delta);
	
	delta_v += self.get_reverse_boost(temp_velocity, x_axis);
	
	delta_v += self.get_gravity();
	delta_v += -temp_velocity * self.air_friction;
	delta_v += self.acceleration;

	if (self.jump_timer > 0.) && (self.ground_timer > 0.):
		self.apply_jump(temp_velocity);
	
	self.set_velocity(temp_velocity + delta_v * delta + self.instant_acceleration);
	self.move_and_slide();
	self.instant_acceleration = Vector2.ZERO;
	self.acceleration = Vector2.ZERO;

func get_walk_force(temp_velocity: Vector2, x_input: float, delta: float) -> Vector2:
	var delta_v: Vector2 = Vector2.ZERO;
	var distance_from_max_v = self.max_walk_speed * abs(x_input) 
	distance_from_max_v += self.left_direction.dot(temp_velocity) * sign(x_input);
	
	if distance_from_max_v > 0.:
		if distance_from_max_v < self.horizontal_accel_speed * delta :
			delta_v += -sign(x_input) * (distance_from_max_v / delta) * self.left_direction;
		else:
			delta_v += -sign(x_input) * self.horizontal_accel_speed * self.left_direction;
	return delta_v;

func get_reverse_boost(temp_velocity: Vector2, x_input: float) -> Vector2:
	if x_input == 0 || self.left_direction.dot(temp_velocity) * sign(x_input) > 0.:
		return -temp_velocity.project(self.left_direction) * self.reverse_boost;
	return Vector2.ZERO;

func apply_jump(temp_velocity: Vector2) -> void:
	self.jumped.emit();
	self.jump_timer = 0.;
	self.ground_timer = 0.;
	self.add_instant_acceleration(
		up_direction * self.jump_y_vel 
		- temp_velocity.project(up_direction)
	);

func get_left() -> Vector2:
	return Vector2(self.up_direction.y, -self.up_direction.x);
