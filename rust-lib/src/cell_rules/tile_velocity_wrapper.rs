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

#[cfg(test)]
mod tile_velocity_tests{
    use crate::cell_rules::{EIGHT_CONNECTED_OFFSETS, MOVE_FLAG_COPY, MOVE_FLAG_SWAP};

    use super::VelocityWrapper;

    #[test]
    fn test_write_vec_then_retrive(){
        for saved_velocity in EIGHT_CONNECTED_OFFSETS{
            let mut wrapper = VelocityWrapper::new();
            wrapper.set_velocity(saved_velocity);
            assert_eq!(wrapper.get_velocity(), saved_velocity);
        }
    }
    
    #[test]
    fn test_write_vec_then_mode_then_retrive(){
        for saved_velocity in EIGHT_CONNECTED_OFFSETS{
            for f in [MOVE_FLAG_COPY,MOVE_FLAG_SWAP]{
                let mut wrapper = VelocityWrapper::new();
                wrapper.set_velocity(saved_velocity);
                wrapper.set_velocity_mode_type(f);
                assert_eq!(wrapper.get_velocity(), saved_velocity);
                assert_eq!(wrapper.is_move_mode_copy(), MOVE_FLAG_COPY == f);
                assert_eq!(wrapper.is_move_mode_swap(), MOVE_FLAG_SWAP == f);
            }
        }
    }

    #[test]
    fn test_write_mode_then_vec_then_retrive(){
        for saved_velocity in EIGHT_CONNECTED_OFFSETS{
            for f in [MOVE_FLAG_COPY,MOVE_FLAG_SWAP]{
                let mut wrapper = VelocityWrapper::new();
                wrapper.set_velocity_mode_type(f);
                wrapper.set_velocity(saved_velocity);
                assert_eq!(wrapper.get_velocity(), saved_velocity);
                assert_eq!(wrapper.is_move_mode_copy(), MOVE_FLAG_COPY == f);
                assert_eq!(wrapper.is_move_mode_swap(), MOVE_FLAG_SWAP == f);
            }
        }
    }
}