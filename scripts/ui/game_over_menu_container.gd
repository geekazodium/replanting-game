extends VBoxContainer
class_name GameOverMenuContainer

func _ready() -> void:
	self.visible = false;
	EventBus.player_out_of_energy.connect(self._on_player_out_of_energy);

func _on_player_out_of_energy() -> void:
	self.get_tree().paused = true;
	self.visible = true;

func _on_retry() -> void:
	self.close();
	self.get_tree().reload_current_scene();

func close() -> void:
	self.visible = false;
	self.get_tree().paused = false;
