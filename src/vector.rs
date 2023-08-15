use core::fmt;

use crate::{traits::{AddSubScl, FloatOrComplex, MathDisplay}, MatrixError, Matrix};

//----------------------------------------------------- Structure
/// A basic vector structure.
#[derive(Clone, Debug)]
pub struct Vector<K: FloatOrComplex> {
    data: Vec<K>
}

//--------------------------------------------- Utility functions
impl<K> Vector<K> 
where
    K:  FloatOrComplex + fmt::Display + Clone
{
    /// Returns the size of `Vector<K>`.
    pub fn  get_size(&self) -> usize {
        self.data.len()
    }

    /// Displays `Vector<K>` on the standard output with a new line.
    pub fn  print_vector(&self) {
        println!("{}", self);
    }

    /// Reshapes `Vector<K>` into a matrix given the number of rows.
    /// Returns Err(MatrixError) if the dimensions are not valid.
    pub fn  reshape_into_matrix(&self, rows: usize) -> Result<Matrix<K>, MatrixError> {
        let mut matrix: Vec<Vec<K>> = Vec::new();
    
        if rows > 0 {
            for chunk in self.data.chunks(rows) {
                matrix.push(chunk.to_vec());
            }
        }

        Matrix::from(matrix)
    }

    /// Returns a clone of the data of `Vector<K>`.
    pub fn  get_data(&self) -> Vec<K> {
        self.data.clone()
    }
}


//----------------------------------------- Traits Implementation
impl<K> Vector<K>
where
    K:  FloatOrComplex
{
    /// Associated constructor `from`.
    pub fn from(vec_data: Vec<K>) -> Vector<K> { 
        Vector { data: vec_data }
    }
}

// Implement fmt::Display trait to be able to print `Vector<K>`
impl<K> fmt::Display for Vector<K> 
where
    K:  FloatOrComplex + fmt::Display
{
    /// Format and print the data of `Vector<K>`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let mut iter = self.data.iter().peekable();
        while let Some(element) = iter.next() {
            write!(f, "{:.1}", element)?;

            if let Some(_) = iter.peek() {
                write!(f, "][")?;
            }
        }
        write!(f, "]")
    }
}

impl<K> IntoIterator for Vector<K>
where
    K:  FloatOrComplex
{
    type Item = K;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Implements into_iter for `Vector<K>` to iter through data.
    fn  into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

// Add, Substract and Scale
impl<K> AddSubScl<Vector<K>, K> for Vector<K> 
where 
    K:  MathDisplay + FloatOrComplex + Clone
{
    /// Add a vector to another one
    /// 
    /// # Examples
    /// 
    /// ```
    /// use matrix::Vector;
    /// use crate::matrix::traits::AddSubScl;
    /// 
    /// let mut u = Vector::from(vec![2., 3.]);
    /// let v = Vector::from(vec![5., 7.]);
    /// 
    /// u.add(&v);
    /// println!("{}", u);
    /// 
    /// // [7.0]
    /// // [10.0]
    /// ```
    /// 
    /// When trying to add a vector of different size, add does nothing.
    fn add(&mut self, v: &Vector<K>) {
        if self.get_size() == v.get_size() {
            for (elem_1, elem_2) in self.data.iter_mut().zip(v.data.iter()) {
                *elem_1 = elem_1.clone() + elem_2.clone();
            }
        }
    }

    /// Substraction of a vector by another one.
    /// When trying to substract a vector of different size, sub does nothing.
    fn sub(&mut self, v: &Vector<K>) {
        if self.get_size() == v.get_size() {
            for (elem_1, elem_2) in self.data.iter_mut().zip(v.data.iter()) {
                *elem_1 = elem_1.clone() - elem_2.clone();
            }
        }     
    }

    /// Scaling of a vector by a scalar.
    fn scl(&mut self, a: K) {
        for element in self.data.iter_mut() {
            *element = element.clone() * a.clone();
        }
    }

}
