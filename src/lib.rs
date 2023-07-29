/*
** ------------------------------------------------------------ TYPES
*/
pub struct Matrix<K> {
    // 
}

//-------------------- Utility functions
impl Matrix<K> {
    // fn get_shape {}
    // print_matrix {}
    // reshape_into_vector {}
}

pub struct Vector<K> {
    //
}

//-------------------- Utility functions
impl Vector<K> {
    // fn get_size {}
    // print_vector {}
    // reshape_into_matrix {}
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
