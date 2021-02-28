use crate::matrix;
use crate::vector;

pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {}

impl vector::Length for Vector2 {
    fn get_length(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

impl vector::Rotate for Vector2 {
    fn rotate(&self, rotationMatrix: matrix::Matrix) -> Vector2 {
        Vector2 { x: 0, y: 0 }
    }
}

impl vector::InnerProduct for Vector2 {
    fn inner_product(&self, _rhs: Vector2) -> i32 {
        self.x * _rhs.x + self.y * _rhs.y
    }
}