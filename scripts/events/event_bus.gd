@warning_ignore_start("unused_signal")
extends Node
#region Tile Set Events
signal break_tile_attempt(global_position: Vector2);
signal place_tile_attempt(global_position: Vector2, atlas_coords: Vector2i);
#endregion

#region Player Stats Events
signal player_energy_changed(current_energy: float);
signal player_out_of_energy();
#endregion

#region World Time Events
signal day_end();
#endregion
@warning_ignore_restore("unused_signal")
