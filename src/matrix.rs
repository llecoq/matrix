use core::fmt;
use std::f32::EPSILON;
use std::iter::Sum;
use std::ops::Index;

// use num_complex::ComplexFloat;

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
    Empty,
    IsNotSquare
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MatrixError::InvalidFormat => write!(f, "Invalid matrix format"),
            MatrixError::RowVector => write!(f, "Library does not handle row vectors"),
            MatrixError::Empty => write!(f, "Empty matrix"),
            MatrixError::IsNotSquare => write!(f, "Matrix needs to be square to compute trace")
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
    K : FloatOrComplex + Clone + std::fmt::Display
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

#[allow(unused_variables)]
impl<K> Matrix<K>
where
    K:  FloatOrComplex + MathDisplay + Sum + Clone
{
    /// Multiplies a matrix by a vector and returns a new `Vector<K>`.
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        let mut result: Vec<K> = Vec::new();

        for elem in &self.data {
            result.push(elem.dot(vec));
        }

        Vector::from(result)
    }

    /// Multiplies a matrix by a matrix and returns a new `Matrix<K>`.
    /// Returns an empty matrix if the number of rows of `self` is different form the number of columns of `mat`.
    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        if self.rows == mat.columns {
            let transposed_mat: Matrix<K> = mat.transpose();
            let mut new_data: Vec<Vec<K>> = Vec::new();

            for self_row in self.clone() {
                let mut new_row: Vec<K> = Vec::new();

                for mat_column in transposed_mat.clone() {
                    new_row.push(self_row.dot(&mat_column));
                }
                new_data.push(new_row);
            }
            return Matrix::from(new_data).unwrap();
        }
        Self::new()
    }

    /// Computes the trace of `Matrix<K>`.
    /// If the matrix isn't square, returns MatrixError,
    /// else returns a new `Matrix<K>` on success.
    pub fn trace(&self) -> Result<K, MatrixError> {
        if self.is_a_square() {
            return Ok((0..self.rows).map(|i| self.data[i][i].clone()).sum::<K>());
        }
        Err(MatrixError::IsNotSquare)
    }

    /// Transposes `Matrix<K>` rows into columns and vice-versa
    /// and returns a new `Matrix<K>`.
    pub fn transpose(&self) -> Matrix<K> {
        (0..self.columns).map(|j| {
            (0..self.rows).map(|i| self[i][j].clone()).collect::<Vector<K>>()
        }).collect::<Matrix<K>>()
    }

    /// Computes the reduced row-echelon form of `Matrix<K>` using the
    /// Gauss-Jordan elimination method with a maximum time complexity of O(n^3)
    /// and a maximum space complexity of O(n^2) for a square matrix.
    pub fn row_echelon(&self) -> Matrix<K> {
        let mut rref_matrix = self.clone();

        // 1. Set current_row to 0.
        let mut current_row :usize = 0;

        // 2. For each column:
        let _ = (0..rref_matrix.columns).map( |current_column| {

            // 2.1. Find the row, pivotRow, below currentRow (including currentRow) with the largest absolute value
            match Self::find_pivot_row(&rref_matrix, current_column) {
                Some(index) => {
                    
                    // 2.2. Swap current_row with pivot_row.
                    if index != current_row {
                        rref_matrix.data.swap(current_row, index);
                    }
                    let leading_coeff: &K = &rref_matrix.data[current_row][current_column].clone();

                    // 2.3. If the leading entry (pivot) in current_row is not 1:
                    if leading_coeff.close_to_one() == false {
                        let scale_factor: K = leading_coeff.scale_factor();

                        // 2.3.1. Scale the entire current_row by the pivot to make the leading entry 1.
                        rref_matrix.data[current_row].scl(scale_factor);
                    }

                    // 2.4. Make all other entries in current_column (below the pivot) zero:
                    let _ = ((current_row + 1)..rref_matrix.rows).map(|index| {
                        let element: &K = &rref_matrix.data[index][current_column];

                        if element.close_to_zero() == false {
                            let multiplication_factor: K = leading_coeff.divide(&element);

                            // 2.4.1. Subtract an appropriate multiple of current_row from pivot_row such that the entry in current_column of current_row becomes zero.  
                            rref_matrix.data[index].scl(multiplication_factor);
                        }
                    });
                }
                None => {} // increment to next column
            }
            current_row += 1;
        });
    
        Matrix::new()
    } 

}

//--------------------------------------- Private utility functions
impl<K> Matrix<K> 
where
    K:  FloatOrComplex + Clone + std::fmt::Display
{
    /// Checks that all the columns are the same size, and thus, that the matrix is valid.
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

    /// Returns the first colomn size or 0.
    fn first_column_size(input: &Vec<Vec<K>>) -> usize {
        input
            .first()
            .map(Vec::len)
            .unwrap_or(0)
    }

    /// Builds `Matrix<K>` data's up and returns it.
    fn build_matrix_data(input: Vec<Vec<K>>) -> Vec<Vector<K>> {
        input
            .into_iter()
            .map(|element| Vector::from(element))
            .collect()
    }

    /// Finds the pivot row with the largest absolute value of the given column and returns the index of it
    /// or None if its value is close to 0.
    fn find_pivot_row(matrix: &Matrix<K>, current_column: usize) -> Option<usize> {
        let current_row: usize = current_column;
        let mut largest_norm_value: f32 = matrix.data[current_row][current_column].norm_value();
        let mut pivot_row: usize = current_row;
    
        let _ = (current_row..matrix.rows).map(|index| {
            let current_norm_value: f32 = matrix.data[index][current_column].norm_value();
            
            if current_norm_value > largest_norm_value {
                largest_norm_value = current_norm_value;
                pivot_row = index;
            };
        });
        match Self::close_to_float(&largest_norm_value, &0.0) {
            false => return Some(pivot_row),
            true => return None
        }
    }

    /// Takes a number `n` in argument and if it's close to 0, returns true.
    fn close_to_float(a: &f32, b: &f32) -> bool {
        (a - b).abs() < EPSILON
    }
    
    // /// Finds the pivot row with the largest absolute value of the given column and returns the index of it.
    // fn find_pivot_row(matrix: &Matrix<K>, current_column: usize) -> usize {
    //     let current_row: usize = current_column;
    //     let (pivot_row, _) = (current_row..matrix.rows)
    //         .map(|index| (index, matrix.data[index][current_column].norm_value()))
    //         .fold((current_row, matrix.data[current_row][current_column].norm_value()),
    //             |(max_row, max_val), (idx, val)| {
    //                 if val > max_val { (idx, val) } else { (max_row, max_val) }
    //             });
    
    //     pivot_row
    // }

}