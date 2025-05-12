extends Node
class_name PlayerJumpInput

@export var character_body: PlatformerCharacterBody;
@export var jump_input: String = "";

func _physics_process(_delta: float) -> void:
	if Input.is_action_just_pressed(self.jump_input):
		self.character_body.jump();
	
