use crate::matrix;

pub trait Length {
    fn get_length(&self) -> i32;
}

pub trait Rotate {
    // TODO: Can be implemented only after matrix implemented
    fn rotate(&self, rotationMatrix: matrix::Matrix) -> Self;
}

pub trait InnerProduct {
    fn inner_product(&self, _rhs: Self) -> i32;
}

pub trait OuterProduct {
    // TODO: Can be implemented only after matrix implemented
    fn outer_product(&self, _rhs: Self) -> matrix::Matrix;
}