extends Node
class_name PlayerEnergy

@export var current_energy: float = 0;
@export var energy_loss_rate: float = 0;
@export var max_energy: float = 0;
@export var energy_generation_rate: float = 0.001;

func _ready() -> void:
	EventBus.energy_generated.connect(self.on_energy_generated);

func _physics_process(delta: float) -> void:
	self.current_energy -= self.energy_loss_rate * delta;
	self.current_energy = maxf(self.current_energy, 0);
	if self.current_energy == 0:
		EventBus.player_out_of_energy.emit();
	EventBus.player_energy_changed.emit(self.current_energy);

func on_energy_generated(amount: float) -> void:
	self.current_energy += amount * self.energy_generation_rate;
