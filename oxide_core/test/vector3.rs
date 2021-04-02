#[cfg(test)]
mod test {
    use crate::vector::*;

    #[test]
    fn get_squared_length() {
        let new_vector = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let res = 50.0;
        assert_eq!(res, new_vector.get_squared_length());
    }

    #[test]
    fn get_length() {
        let new_vector = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let res = 7.0710678118654752440084436210485;
        assert_eq!(res, new_vector.get_length());
    }

    #[test]
    fn inner_product() {
        let new_vector_1 = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let new_vector_2 = vector3::Vector3 {
            x: 3.0,
            y: 7.0,
            z: 11.0,
        };
        let res = 92.0;
        assert_eq!(res, new_vector_1.inner_product(new_vector_2));
    }
}
