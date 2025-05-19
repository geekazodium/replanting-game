extends Node
class_name PlayerEnergy

@export var current_energy: float = 0;
@export var max_energy: float = 0;
@export var energy_generation_rate: float = 0.001;
@export var daily_energy_cost: float = 1;

func _ready() -> void:
	EventBus.energy_generated.connect(self.on_energy_generated);
	EventBus.day_end_player_choice_made.connect(self.on_choice_made);
	self.call_deferred("add_to_energy",0);

func on_energy_generated(amount: float) -> void:
	self.add_to_energy(amount * self.energy_generation_rate);

func on_choice_made(attempt_resurrect: bool) -> void:
	if !attempt_resurrect:
		self.add_to_energy(-self.daily_energy_cost);

func get_current_energy() -> float:
	return self.current_energy;

func add_to_energy(value: float) -> void:
	self.current_energy += value;
	self.current_energy = maxf(self.current_energy, 0);
	if self.current_energy == 0:
		EventBus.player_out_of_energy.emit();
	EventBus.player_energy_changed.emit(self.current_energy);
