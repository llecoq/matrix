use core::fmt;
use std::iter::Sum;
use std::ops::Index;

use crate::vector::{Vector, self};
use crate::traits::{AddSubScl, FloatOrComplex, MathDisplay};

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
impl<K> Matrix<K>
where
    K:  FloatOrComplex + fmt::Display + Clone
{

    /// A function that returns the shape of `Matrix<K>` in the format (rows, columns).
    pub fn get_shape(&self) -> String {
        format!("({},{})", self.rows, self.columns)
    }

    /// A function that returns true if `Matrix<K>` shape is a square.
    pub fn is_a_square(&self) -> bool {
        self.rows == self.columns
    }

    /// A function that displays `Matrix<K>` on the standard ouptut with a new line.
    pub fn print_matrix(&self) {
        println!("{}", self);
    }

    /// A function that is reshaping `Matrix<K>` into a signle `Vector<K>` and returns it.
    pub fn reshape_into_vector(&self) -> Vector<K> {
        let mut flattened_data: Vec<K> = Vec::new();

        for rows in &self.data {
            flattened_data.extend(rows.get_data());
        }   
        Vector::from(flattened_data)
    }
}

//----------------------------------------- Traits Implementation
impl<K> Matrix<K>
where
    K : FloatOrComplex + Clone
{
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

    /// Associated constructor `new`.
    /// Returns an empty `Matrix<K>`.
    pub fn new() -> Matrix<K> {
        Matrix {
            rows: (0),
            columns: (0),
            data: (Self::build_matrix_data(vec![vec![]]))
        }
    }

    /// Safe indexed read data
    pub fn get(&self, index: usize) -> Option<&vector::Vector<K>> {
        self.data.get(index)
    }
}

/// Implements `Index` for `Matrix<K>`
impl<K> Index<usize> for Matrix<K>
where 
    K:  FloatOrComplex
{
    type Output = Vector<K>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<K> fmt::Display for Matrix<K> 
where
    K: FloatOrComplex + fmt::Display
{
    /// Implements display for `Matrix<K>` data.
    /// This is not displaying the full content of `Matrix<K>`, use #[Derive(Debug)] for that.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Self::format_and_display_data(self, f, "")
    }
}

impl<K> fmt::Debug for Matrix<K>
where
    K:  FloatOrComplex + fmt::Display
{
    /// Implements pretty display for `Matrix<K>`.
    /// This is displaying the full content of `Matrix<K>` in a pretty way.
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

/// Implements `FromIterator` for `Matrix<K>`
impl<K> FromIterator<Vector<K>> for Matrix<K>
where
    K:  FloatOrComplex + Clone + std::fmt::Display
{
    fn from_iter<T: IntoIterator<Item = Vector<K>>>(iter: T) -> Self {
        let collected_data: Vec<Vector<K>> = iter.into_iter().collect();
        Matrix { rows: (collected_data.len()), columns: (collected_data[0].get_size()), data: (collected_data) }
    }
}

impl<K> IntoIterator for Matrix<K>
where
    K:  FloatOrComplex
{
    type Item = Vector<K>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Implements into_iter for `Matrix<K>` to iter through data.
    fn  into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<K> Matrix<K>
where
    K:  FloatOrComplex + fmt::Display
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
impl<K> AddSubScl<Matrix<K>, K> for Matrix<K>
where
    K:  MathDisplay + FloatOrComplex + Clone + Copy
{

    /// Adds `Matrix<K>` to another one
    /// When trying to add a matrix of different size, result is undefined.
    fn add(&mut self, m: &Matrix<K>) {
        // if self.get_shape() == m.get_shape() {
            for (elem_1, elem_2) in self.data.iter_mut().zip(m.clone().into_iter()) {
                elem_1.add(&elem_2);
            }
        // }
    }

    /// Substraction of `Matrix<K>` by another one.
    /// When trying to add a matrix of different size, result is undefined.
    fn sub(&mut self, m: &Matrix<K>){
        // if self.get_shape() == m.get_shape() {
            for (elem_1, elem_2) in self.data.iter_mut().zip(m.clone().into_iter()) {
                elem_1.sub(&elem_2);
            }
        // }
    }

    /// Scaling of `Matrix<K>` by a scalar.
    fn scl(&mut self, a: K){
        for elem in self.data.iter_mut().into_iter() {
            elem.scl(a);
        }
    }
}

impl<K> Matrix<K>
where
    K:  FloatOrComplex + MathDisplay + Sum + Clone
{
    /// Multiplies a matrix by a vector
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        let mut result: Vec<K> = Vec::new();

        for elem in &self.data {
            result.push(elem.dot(vec));
        }

        Vector::from(result)
    }

    /// Multiplies a matrix by a matrix.
    /// Returns an empty matrix if the number of rows of `self` is different form the number of columns of `mat`.
    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        if self.rows == mat.columns {

        }
        Self::new()
    }
}

//--------------------------------------- Private utility functions
impl<K> Matrix<K> 
where
    K:  FloatOrComplex + Clone + Iterator
{
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

    // Builds `Matrix<K>` data's up and returns it
    fn build_matrix_data(input: Vec<Vec<K>>) -> Vec<Vector<K>> {
        input
            .into_iter()
            .map(|element| Vector::from(element))
            .collect()
    }

    // // Transposes `Matrix<K>` rows into columns and vice-versa
    // fn transpose(input: &Matrix<K>) -> Matrix<K> {


    //     (0..input.columns).map(|j| {
    //         (0..input.rows).map(|i| input[i][j].clone()).collect::<Vector<K>>()
    //     }).collect();

    //     Self::new()
    // }
}