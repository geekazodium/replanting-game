extends Node
class_name PlayerPlantableTypes

@export var plant_types: Dictionary[StringName, PlantType];
@export var player_selected_type: int;
@export var player_unlocked_types: Array[StringName] = [];

@export var player_energy: PlayerEnergy;

func _ready() -> void:
	EventBus.day_end_player_choice_made.connect(self.on_choice_made);

func on_choice_made(attempt_resurrect: bool) -> void:
	if !attempt_resurrect:
		return;
	
	var locked: Array[StringName] = self.plant_types.keys();
	locked.sort();
	for k in self.player_unlocked_types: 
		#tragic, the O(n^2) algorithm that could have been avoided.
		#that is, if I had enough time to make this optimized.
		#which uhh... no.
		locked.remove_at(locked.find(k));
	
	var random_index: int = randi_range(0,locked.size() - 1);
	var type_name: StringName = locked[random_index];
	var plant_type: PlantType = self.plant_types[type_name];
	if self.player_energy.get_current_energy() > plant_type.energy_cost:
		self.player_energy.add_to_energy(-plant_type.energy_cost);
		self.player_unlocked_types.append(type_name)
		EventBus.plant_resurrect_success.emit(type_name,plant_type);
