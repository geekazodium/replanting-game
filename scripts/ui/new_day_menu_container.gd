extends VBoxContainer
class_name NewDayMenuContainer

func _ready() -> void:
	EventBus.day_end.connect(self.on_day_end);

func on_day_end() -> void:
	self.visible = true;
	self.get_tree().paused = true;

func _on_keep_planting_pressed() -> void:
	self.on_exit();
	EventBus.day_end_player_choice_made.emit(false);

func _on_attempt_resurrection_pressed() -> void:
	self.on_exit();
	EventBus.day_end_player_choice_made.emit(true);

func on_exit() -> void:
	self.visible = false;
	self.get_tree().paused = false;
