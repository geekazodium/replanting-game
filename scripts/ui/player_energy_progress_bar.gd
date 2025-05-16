extends ProgressBar
class_name PlayerEnergyProgressBar

func _ready() -> void:
	EventBus.player_energy_changed.connect(self._on_energy_changed);

func _on_energy_changed(current: float) -> void:
	self.value = current;
