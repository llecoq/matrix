use crate::vector::Vector;
use crate::traits::AddSubScl;

//----------------------------------------------------- Structure
/// A basic matrix structure
#[allow(dead_code)]
pub struct Matrix<K> {
    rows: usize,
    columns: usize,
    data: Vec<Vector<K>>
}

//--------------------------------------------- Utility functions
#[allow(dead_code)]
impl<K> Matrix<K> {
    // fn get_shape {}
    // fn is_a_square {}
    // fn print_matrix {}
    // fn reshape_into_vector {}
}

//----------------------------------------- Traits Implementation
impl<K> Matrix<K> {
    pub fn from(input_vec: Vec<Vec<K>>) -> Matrix<K> {
        let mut data_vec: Vec<Vector<K>> = Vec::new();
        let first_column_size = input_vec.iter().len();

        let mut iter = input_vec.iter();
        for element in input_vec {
            match element.len() {
                first_column_size => {
                    // push into data_vec
                }
                _ => {
                    // handle error, not all columns are same sizes
                }
            }
        }

        Matrix {
            rows: 1,
            columns: 1,
            data: data_vec
        }
    }
}

#[allow(unused_variables)]
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