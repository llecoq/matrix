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
// impl<K> Matrix<K> {
//     pub fn from(input_vec: Vec<Vec<K>>) -> Matrix<K> {
//         // iter through vec
//         // check size of each vecs inside (need to be all the same)
//         // create a Vector<K> with each vec
//         // create new vec with the data

//         Matrix {
//             rows: 1,
//             columns: 1,
//             data: input_vec
//         }
//     }
// }

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