use crate::vector::Vector;
use crate::traits::AddSubScl;

/// Matrix structure
pub struct Matrix<K> {
    vectors: Vec<Vector<K>>
}

//-------------------- Utility functions
impl<K> Matrix<K> {
    // fn get_shape {}
    // print_matrix {}
    // reshape_into_vector {}
}

impl<K> AddSubScl<Matrix<K>, K> for Matrix<K> {
    fn add(&mut self, v: &Matrix<K>){

    }
    fn sub(&mut self, v: &Matrix<K>){

    }
    fn scl(&mut self, a: K){

    }
}