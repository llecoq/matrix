use crate::vector::Vector;
use crate::traits::AddSubScl;

//----------------------------------------------------- Structure
/// A basic matrix structure
#[allow(dead_code)]
#[derive(Debug)]
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
            rows: input.len(),
            columns: Self::first_column_size(&input),
            data: Self::build_matrix_data(input)
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

    // check that all the columns are the same size, and thus, that the matrix is valid
    fn input_matrix_is_valid(input: &Vec<Vec<K>>) -> bool {
    let first_inner_len: usize = Self::first_column_size(input);
        input
            .iter()
            .all(|inner_vec| inner_vec.len() == first_inner_len)
    }

    // return the first colomn size or 0
    fn first_column_size(input: &Vec<Vec<K>>) -> usize {
        input
            .first()
            .map(Vec::len)
            .unwrap_or(0)
    }

    // build the matrix data up and return it
    #[allow(unused_variables)]
    fn build_matrix_data(input: Vec<Vec<K>>) -> Vec<Vector<K>> {
        let data: Vec<Vector<K>> = input
            .into_iter()
            .map(|element| Vector::from(element))
            .collect();

        data
    }
}