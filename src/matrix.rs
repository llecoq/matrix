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
#[allow(unused_variables)]
impl<K> Matrix<K> {
    pub fn from(input: Vec<Vec<K>>) -> Matrix<K> 
    where
        K: std::fmt::Debug
{
        match Self::input_matrix_is_valid(&input) {
            true => {println!("true");}
            false => {println!("false");}
        }    

        Matrix {
            rows: 1,
            columns: 1,
            data: vec![Vector::from(vec![])]
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

//--------------------------------------- Private utility functions
impl<K> Matrix<K> {
// check that all the columns are all the same size, and thus, that the matrix is valid
    fn input_matrix_is_valid(input: &Vec<Vec<K>>) -> bool {
        let first_inner_len: usize = input
        .first()
        .map(Vec::len)
        .unwrap_or(0);
    input
        .iter()
        .all(|inner_vec| inner_vec.len() == first_inner_len)
    }
}