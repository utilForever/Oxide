#[cfg(test)]
mod test {
    use crate::vector::vector2;
    use crate::vector::*;

    #[test]
    fn test_vector2_get_squared_length() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        assert_eq!(25.0, new_vector.get_squared_length());
    }

    #[test]
    fn test_vector2_get_length() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        assert_eq!(5.0, new_vector.get_length());
    }

    #[test]
    fn test_vector2_inner_product() {
        let new_vector_1 = vector2::Vector2 { x: 3.0, y: 4.0 };
        let new_vector_2 = vector2::Vector2 { x: 3.0, y: 7.0 };
        assert_eq!(37.0, new_vector_1.inner_product(new_vector_2));
    }
}
