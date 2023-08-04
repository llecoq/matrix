use core::fmt;

use crate::vector::Vector;
use crate::traits::{AddSubScl, FloatOrComplex};

//----------------------------------------------------- Structure
/// A basic matrix structure
#[allow(dead_code)]
#[derive(Clone)]
pub struct Matrix<K: FloatOrComplex> {
    rows: usize,
    columns: usize,
    data: Vec<Vector<K>>
}

//------------------------------------------------- Error handling

#[derive(Debug)]
pub enum MatrixError {
    InvalidFormat,
    RowVector,
    Empty
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MatrixError::InvalidFormat => write!(f, "Invalid matrix format"),
            MatrixError::RowVector => write!(f, "Library does not handle row vectors"),
            MatrixError::Empty => write!(f, "Empty matrix")
        }
    }
}

//--------------------------------------------- Utility functions
#[allow(dead_code)]
impl<K: FloatOrComplex> Matrix<K>
where
    K:  fmt::Display
        + Clone
{

    /// A function that returns the shape of Matrix<K> in the format (rows, columns).
    pub fn get_shape(&self) -> String {
        format!("({},{})", self.rows, self.columns)
    }

    /// A function that returns true if Matrix<K> shape is a square.
    pub fn is_a_square(&self) -> bool {
        self.rows == self.columns
    }

    /// A function that displays Matrix<K> on the standard ouptut with a new line.
    pub fn print_matrix(&self) {
        println!("{}", self);
    }

    /// A function that is reshaping Matrix<K> into a signle Vector<K> and returns it.
    pub fn reshape_into_vector(&self) -> Vector<K> {
        let mut flattened_data: Vec<K> = Vec::new();

        for rows in &self.data {
            flattened_data.extend(rows.get_data());
        }   
        Vector::from(flattened_data)
    }
}

//----------------------------------------- Traits Implementation
impl<K: FloatOrComplex> Matrix<K> {
    /// Associated constructor `from`.
    /// Returns Err(MatrixError) if the format is not valid.
    pub fn from(input: Vec<Vec<K>>) -> Result<Matrix<K>, MatrixError> {
        match Self::input_format_is_valid(&input) {
            Ok(0) => return Err(MatrixError::Empty),
            Ok(_) => return Ok(Matrix {
                        rows: input.len(),
                        columns: Self::first_column_size(&input),
                        data: Self::build_matrix_data(input)
                    }),
            Err(err) => return Err(err)
        }
    }
}

impl<K: FloatOrComplex> fmt::Display for Matrix<K> 
where
    K: fmt::Display
{
    /// Implements display for Matrix<K> data.
    /// This is not displaying the full content of Matrix<K>, use #[Derive(Debug)] for that.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Self::format_and_display_data(self, f, "")
    }
}

impl<K: FloatOrComplex> fmt::Debug for Matrix<K>
where
    K:  fmt::Display
{
    /// Implements pretty display for Matrix<K>.
    /// This is displaying the full content of Matrix<K> in a pretty way.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix {{")?;
        writeln!(f, "   rows: {},", self.rows)?;
        writeln!(f, "   columns: {},", self.columns)?;
        writeln!(f, "   data: [")?;
        Self::format_and_display_data(self, f, "      ")?;
        write!(f, "\n")?;
        writeln!(f, "   ],")?;
        write!(f, "}}")
    }
}

impl<K: FloatOrComplex> IntoIterator for Matrix<K> {
    type Item = Vector<K>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Implements into_iter for Matrix<K> to iter through data.
    fn  into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<K: FloatOrComplex> Matrix<K>
where
    K:  fmt::Display
{
    // Format and displays the data, taking into account an input padding
    fn format_and_display_data(matrix: &Matrix<K>, f: &mut fmt::Formatter<'_>, padding: &str) -> fmt::Result {
        let mut iter = matrix.data.iter().peekable();
        
        while let Some(vector) = iter.next() {
            write!(f, "{}", padding)?;
            write!(f, "{}", vector)?;
            
            if let Some(_) = iter.peek() {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[allow(unused_variables)]
impl<K: FloatOrComplex> AddSubScl<Matrix<K>, K> for Matrix<K>
where
    K:  Clone
        + fmt::Display
    
{

    /// Adds Matrix<K> to another one
    /// When trying to add a matrix of different size, add does nothing.
    fn add(&mut self, m: &Matrix<K>) {
        if self.get_shape() == m.get_shape() {
            // for (mut elem_1, elem_2) in self.data.into_iter().zip(m.data.into_iter()) {
            //     elem_1 = elem_1 + elem_2;
            // }
        }
    }

    /// Substraction of Matrix<K> by another one.
    /// When trying to add a matrix of different size, sub does nothing.
    fn sub(&mut self, m: &Matrix<K>){

    }

    /// Scaling of Matrix<K> by a scalar.
    fn scl(&mut self, a: K){

    }
}

//--------------------------------------- Private utility functions
impl<K: FloatOrComplex> Matrix<K> {

    // Checks that all the columns are the same size, and thus, that the matrix is valid
    fn input_format_is_valid(input: &Vec<Vec<K>>) -> Result<usize, MatrixError> {
    let first_inner_len: usize = Self::first_column_size(input);
        if input.len() == 1 {
            return Err(MatrixError::RowVector);
        }
        else if input
            .iter()
            .all(|inner_vec| inner_vec.len() == first_inner_len) {
                return Ok(first_inner_len);
        }
        Err(MatrixError::InvalidFormat)
    }

    // Returns the first colomn size or 0
    fn first_column_size(input: &Vec<Vec<K>>) -> usize {
        input
            .first()
            .map(Vec::len)
            .unwrap_or(0)
    }

    // Builds Matrix<K> data's up and returns it
    #[allow(unused_variables)]
    fn build_matrix_data(input: Vec<Vec<K>>) -> Vec<Vector<K>> {
        input
            .into_iter()
            .map(|element| Vector::from(element))
            .collect()
    }
}