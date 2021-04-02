use crate::matrix;
pub mod vector2;
pub mod vector3;

pub trait Length {
    fn get_squared_length(&self) -> f32;
    fn get_length(&self) -> f32;
}

pub trait Rotate {
    // TODO: Can be implemented only after matrix implemented
    fn rotate(&self, rotation_matrix: matrix::Matrix) -> Self;
}

pub trait InnerProduct {
    fn inner_product(&self, _rhs: Self) -> f32;
}

pub trait OuterProduct {
    // TODO: Can be implemented only after matrix implemented
    fn outer_product(&self, _rhs: Self) -> matrix::Matrix;
}

pub trait Scale {
    fn scale(&self, multiplier: f32) -> Self;
}
