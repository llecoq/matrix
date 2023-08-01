use crate::vector::Vector;
use crate::traits::AddSubScl;

/// A basic matrix structure
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
    /// Add a matrix to another one
    fn add(&mut self, v: &Matrix<K>){

    }
    /// Substraction of a matrix by another one.
    fn sub(&mut self, v: &Matrix<K>){

    }
    /// Scaling of a matrix by a scalar.
    fn scl(&mut self, a: K){

    }
}