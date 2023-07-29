/*
** ------------------------------------------------------------ TYPES
*/
pub struct Vector<K> {
    x: K,
    y: K,
    z: K,
}

//-------------------- Utility functions
impl<K> Vector<K> {
    // fn get_size {}
    // print_vector {}
    // reshape_into_matrix {}
}

pub struct Matrix<K> {
    vectors: Vec<Vector<K>>
}

//-------------------- Utility functions
impl<K> Matrix<K> {
    // fn get_shape {}
    // print_matrix {}
    // reshape_into_vector {}
}

/*
** ------------------------------------------------------------ TRAITS
*/
// T: type of structure (Vector or Matrix)
// K: number type
pub trait AddSubScl<T, K> {
    fn add(&mut self, t: &T);
    fn sub(&mut self, t: &T);
    fn scl(&mut self, a: K);
}

/*
** ---------------------------------------------------------- MODULES
*/
mod add_sub_scl;
