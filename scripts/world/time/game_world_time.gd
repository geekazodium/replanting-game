extends Node
class_name GameWorldTime

@export var day_duration_seconds: float = 0;
@export var current_time_seconds: float = 0;
@export var current_day: float = 0;

func _physics_process(delta: float) -> void:
	self.current_time_seconds += delta;
	if self.current_time_seconds > self.day_duration_seconds:
		self.current_time_seconds -= self.day_duration_seconds;
		self.current_day += 1;
		EventBus.day_end.emit();
