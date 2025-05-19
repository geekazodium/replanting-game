@warning_ignore_start("unused_signal")
extends Node
#region Tile Set Events
signal break_tile_attempt(global_position: Vector2);
signal place_tile_attempt(global_position: Vector2, atlas_coords: Vector2i);
signal energy_generated(amount: float);
#endregion

#region Player Stats Events
signal player_energy_changed(current_energy: float);
signal player_out_of_energy();
#endregion

#region World Time Events
signal day_end();
#endregion

#region Player New Day Choice Event
signal day_end_player_choice_made(attempt_resurrect: bool);
signal plant_resurrect_success(plant_name: StringName, plant_type: PlantType);
#endregion

@warning_ignore_restore("unused_signal")
