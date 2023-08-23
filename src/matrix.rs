use core::fmt;
use std::f32::EPSILON;
use std::iter::Sum;
use std::ops::Index;

// use num_complex::ComplexFloat;

use crate::vector::{Vector, self};
use crate::traits::{AddSubScl, FloatOrComplex, MathDisplay};

//----------------------------------------------------- Structure
/// A basic matrix structure
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
    IsNotSquare,
    IsSingular
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MatrixError::InvalidFormat => write!(f, "Invalid matrix format"),
            MatrixError::RowVector => write!(f, "Library does not handle row vectors"),
            MatrixError::Empty => write!(f, "Empty matrix"),
            MatrixError::IsNotSquare => write!(f, "Matrix needs to be square to compute trace"),
            MatrixError::IsSingular => write!(f, "Matrix is singular and cannot be inverted")
        }
    }
}

//--------------------------------------------- Utility functions
impl<K> Matrix<K>
where
    K:  FloatOrComplex + MathDisplay + Clone + Copy
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
    K : FloatOrComplex + Clone + MathDisplay + Copy
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
    K:  FloatOrComplex + Clone + MathDisplay + Copy
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
    K:  FloatOrComplex + MathDisplay + Sum + Clone + Copy
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

    /// Computes the reduced row-echelon form of `Matrix<K>` on a clone of self, using the
    /// Gauss-Jordan elimination method with a maximum time complexity of O(n^3)
    /// and a maximum space complexity of O(n^2) for a square matrix.
    pub fn row_echelon(&self) -> Matrix<K> {
        let mut rref_matrix = self.clone();

        // 1. Set current_row to 0.
        let mut current_row :usize = 0;

        // 2. For each column:
        (0..rref_matrix.columns)
            .for_each(|current_column| {

            // 2.1. Find the row, pivotRow, below currentRow (including currentRow) with the largest absolute value
            if current_row < rref_matrix.rows {

                match Self::find_pivot_row(&rref_matrix, current_row, current_column) {
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
                    
                        // 2.4. Make all other entries in current_column (above and below the pivot) zero:
                        (0..rref_matrix.rows)
                            .filter(|&index| index != current_row)
                            .for_each(|index| {
                                let element: &K = &rref_matrix.data[index][current_column];
                            
                            if element.close_to_zero() == false {
                                let multiplication_factor: K = element.clone();
                                let mut scaled_current_row: Vector<K> = rref_matrix.data[current_row].clone();
                                // 2.4.1. Subtract an appropriate multiple of current_row from pivot_row such that the entry in current_column of current_row becomes zero.  
                                scaled_current_row.scl(multiplication_factor);
                                rref_matrix.data[index].sub(&scaled_current_row);
                            }
                        });
                        current_row += 1;
                    }
                    None => {} // increment to next column
                }
            }
        });
    
        rref_matrix
    } 


    /// Computes the determinant of `Matrix<K>` of 4 dimensions maximum and returns it.
    /// If the matrix isn't square, returns the first element of the first row.
    pub fn determinant(&self) -> K {
        let mut determinant: K = self.data[0][0].clone();

        match &*self.get_shape() {
            "(2,2)" => determinant = Self::det_2x2(&self),
            "(3,3)" => determinant = Self::det_3x3(&self),
            "(4,4)" => determinant = Self::det_4x4(&self),
            _ => {}
        }
        determinant
    }

    /// Computes the inverse matrix of `Matrix<K>` on a clone of self and returns it.
    /// Returns a `MatrixError` if the matrix is singular.
    pub fn inverse(&self) -> Result<Matrix<K>, MatrixError> {
        if self.is_a_square() && !self.determinant().close_to_zero() {
            // Augment the matrix with the identity matrix.
            let mut inverted_matrix: Matrix<K> = self.append_identity_matrix();
            // Apply Gauss-Jordan Elimination to the augmented matrix.
            inverted_matrix = inverted_matrix.row_echelon();
            // Extract the inverse (the right side of the matrix)
            inverted_matrix.extract_inverse();
            Ok(inverted_matrix)
        }
        else {
            Err(MatrixError::IsSingular)
        }
    }

    /// Computes the rank of self and returns it.
    pub fn rank(&self) -> usize {
        let mut rank: usize = 0;
        let rref_matrix: Matrix<K> = self.row_echelon();

        for row in rref_matrix {
            for element in row {
                if !element.close_to_zero() {
                    rank += 1;
                    break;
                }
            }
        }
        rank
    }

}

//--------------------------------------- Private utility functions
impl<K> Matrix<K> 
where
    K:  FloatOrComplex + Clone + Copy + MathDisplay
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
    fn find_pivot_row(matrix: &Matrix<K>, current_row: usize, current_column: usize) -> Option<usize> {
        // let current_row: usize = current_column;
        let mut largest_norm_value: f32 = matrix.data[current_row][current_column].norm_value();
        let mut pivot_row: usize = current_row;
    
        (current_row..matrix.rows).for_each(|index| {
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

    /// Returns the determinant of a 2x2 matrix.
    fn det_2x2(&self) -> K {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }
    
    /// Returns the determinant of a 3x3 matrix.
    fn det_3x3(&self) -> K {
        self.data[0][0] * (self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1])
            - self.data[0][1] * (self.data[1][0] * self.data[2][2] - self.data[1][2] * self.data[2][0])
            + self.data[0][2] * (self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0])
    }
    
    /// Returns the determinant of a 4x4 matrix.
    /// Laplace (or cofactor) expansion hard-coded for better efficiency given the 4x4 limitation.
    fn det_4x4(&self) -> K {
        let mat_1: Matrix<K> = Self::format_3x3(self, 0);
        let mat_2: Matrix<K> = Self::format_3x3(self, 1);
        let mat_3: Matrix<K> = Self::format_3x3(self, 2);
        let mat_4: Matrix<K> = Self::format_3x3(self, 3);

        self.data[0][0] * mat_1.determinant() - self.data[0][1] * mat_2.determinant() +
            self.data[0][2] * mat_3.determinant() - self.data[0][3] * mat_4.determinant()
    }

    /// Delete the first row and the given column from `Matrix<K>` and returns a new matrix.
    fn format_3x3(&self, column_index: usize) -> Matrix<K> {
        let mut matrix: Matrix<K> = self.clone();

        matrix.data.remove(0);
        for row in matrix.data.iter_mut() {
            row.remove(column_index);
        }

        matrix.columns = 3;
        matrix.rows = 3;
        matrix
    }

    /// Appends an identity matrix to a clone of `Matrix<K>` and returns it.
    fn append_identity_matrix(&self) -> Matrix<K> {
        let mut augmented_matrix: Matrix<K> = self.clone();

        for (index, row) in augmented_matrix.data.iter_mut().enumerate() {
            let mut vec: Vec<K> = vec![K::zero(); row.get_size()];

            vec[index] = K::one();
            row.append(&mut vec);
        }
        augmented_matrix
    }

    /// Extract the inverse of self by removing the left side of the matrix.
    fn extract_inverse(&mut self) {
        for row in self.data.iter_mut() {
            row.drain_and_drop(0..(row.get_size() / 2));
        }
    }

}