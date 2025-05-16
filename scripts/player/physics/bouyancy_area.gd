extends Area2D

@export var character_body: PlatformerCharacterBody;

@export var jump_input: String;

@export var swim_up_force: float = 200;
@export var up_force: float = 200;
@export var friction: float = 2;

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _physics_process(_delta: float) -> void:
	if self.has_overlapping_bodies():
		if Input.is_action_pressed(self.jump_input):
			self.character_body.add_acceleration(Vector2.UP * self.swim_up_force);
		self.character_body.add_acceleration(Vector2.UP * self.up_force);
		self.character_body.add_acceleration(-self.friction * self.character_body.velocity);
