use core::fmt;

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

//------------------------------------------------- Error handling

#[derive(Debug)]
pub enum MatrixError {
    InvalidFormat,
    Empty
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MatrixError::InvalidFormat => write!(f, "Invalid matrix format"),
            MatrixError::Empty => write!(f, "Empty matrix")
        }
    }
    
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
    pub fn from(input: Vec<Vec<K>>) -> Result<Matrix<K>, MatrixError> {
        match Self::input_format_is_valid(&input) {
            Ok(0) => return Err(MatrixError::Empty),
            Ok(_) => return Ok(Matrix {
                        rows: input.len(),
                        columns: Self::first_column_size(&input),
                        data: Self::build_matrix_data(input)
                    }),
            Err(_) => return Err(MatrixError::InvalidFormat)
        }
    }
}

impl<K> fmt::Display for Matrix<K> 
where
    K: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.data.iter().peekable();

        while let Some(vector) = iter.next() {
            vector.fmt(f)?;

            if let Some(_) = iter.peek() {
                write!(f, "\n")?;
            }
        }
        Ok(())
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
    fn input_format_is_valid(input: &Vec<Vec<K>>) -> Result<usize, MatrixError> {
    let first_inner_len: usize = Self::first_column_size(input);
        if input
            .iter()
            .all(|inner_vec| inner_vec.len() == first_inner_len) {
                return Ok(first_inner_len);
        }
        Err(MatrixError::InvalidFormat)
    }

    // return the first colomn size or 0
    fn first_column_size(input: &Vec<Vec<K>>) -> usize {
        input
            .first()
            .map(Vec::len)
            .unwrap_or(0)
    }

    // build the matrix data up and returns it
    #[allow(unused_variables)]
    fn build_matrix_data(input: Vec<Vec<K>>) -> Vec<Vector<K>> {
        input
            .into_iter()
            .map(|element| Vector::from(element))
            .collect()
    }
}