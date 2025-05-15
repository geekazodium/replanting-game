use super::MOVE_FLAG_COPY;
use super::MOVE_FLAG_SWAP;

use godot::builtin::Vector2i;

#[derive(Clone, Copy)]
pub struct VelocityWrapper{
    velocity: i8
}

impl VelocityWrapper{
    pub fn new() -> Self{
        Self { velocity: 0b101|MOVE_FLAG_SWAP }
    }
    pub fn get_velocity(&mut self) -> Vector2i{
        let x_index = self.velocity & 0b11;
        let y_index = (self.velocity >> 2) & 0b11;

        let x: i32 = (x_index - 1).signum() as i32;
        let y: i32 = (y_index - 1).signum() as i32;

        Vector2i { x, y }
    }
    pub fn set_velocity(&mut self, velocity: Vector2i){
        let x = velocity.x.signum() as i8 + 1;
        let y  = velocity.y.signum() as i8 + 1;
        self.velocity &= 0b1110000;
        self.velocity |= x | (y << 2);
    }
    pub fn set_velocity_mode_type(&mut self, type_flag: i8){
        self.velocity &= 0b1111;
        self.velocity |= type_flag;
    }
    pub fn is_move_mode_copy(&self) -> bool{
        self.velocity & MOVE_FLAG_COPY != 0
    }
    pub fn is_move_mode_swap(&self) -> bool{
        self.velocity & MOVE_FLAG_SWAP != 0
    }
}
