@warning_ignore_start("unused_signal")
extends Node

signal place_tile_attempt(global_position: Vector2, atlas_coords: Vector2i);

signal player_energy_changed(current_energy: float);
signal player_out_of_energy();

signal day_end();
@warning_ignore_restore("unused_signal")
