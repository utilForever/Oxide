use crate::{matrix, vector3};

pub trait Rotate {
    // TODO: Can be implemented only after matrix implemented
    fn rotate(&self, rotation_matrix: matrix::Matrix) -> Self;
}

pub trait InnerProduct {
    fn inner_product(&self, _rhs: &Self) -> f32;
}

pub trait OuterProduct {
    fn outer_product(&self, _rhs: &Self) -> vector3::Vector3;
}

pub trait Scale {
    fn scale(&self, multiplier: f32) -> Self;
}

pub trait Negate {
    fn negate(&self) -> Self;
}

pub trait Length {
    fn get_squared_length(&self) -> f32;
    fn get_length(&self) -> f32;
    fn normalize(&self) -> Self;
}
