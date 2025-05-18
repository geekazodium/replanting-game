extends VBoxContainer
class_name NewDayMenuContainer

func _ready() -> void:
	EventBus.day_end.connect(self.on_day_end);

func on_day_end() -> void:
	self.visible = true;
	self.get_tree().paused = true;
