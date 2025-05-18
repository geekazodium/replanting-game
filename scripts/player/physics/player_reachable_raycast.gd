extends RayCast2D
class_name PlayerReachableRaycast

@export var max_distance: float = 128;

func get_reachable_global_pos(target: Vector2, inside: bool = false) -> Vector2:
	self.target_position = Vector2(min(max_distance, target.length()),0);
	var r: float = atan2(target.y, target.x);
	self.rotation = r;
	self.force_raycast_update();
	if self.is_colliding():
		return self.get_collision_point() + self.get_collision_normal() * (-1 if inside else 1);
	else:
		return self.target_position.rotated(r) + self.global_position;
