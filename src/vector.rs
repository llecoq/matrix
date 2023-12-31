use core::fmt;
use std::ops::{Index, RangeBounds, IndexMut};

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
    K:  FloatOrComplex + MathDisplay + Clone + Copy
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
    pub fn get_data(&self) -> Vec<K> {
        self.data.clone()
    }

    /// Moves all the elements of other into self, leaving other empty.
    pub fn append(&mut self, other: &mut Vec<K>) {
        self.data.append(other);
    }

    /// Removes the specified range from the vector in bulk, returning all removed elements as an iterator. 
    /// The iterator is dropped before being fully consumed, droping the remaining removed elements.
    pub fn drain_and_drop<R>(&mut self, range: R)
    where 
        R: RangeBounds<usize> 
    {
        self.data.drain(range).for_each(drop);
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

    /// Associated constructor `new`
    pub fn new() -> Vector<K> {
        Vector { data: vec![] }
    }

    /// Safe indexed read data
    pub fn get(&self, index: usize) -> Option<&K> {
        self.data.get(index)
    }

    pub fn remove(&mut self, index: usize) {
        self.data.remove(index);
    }
}

/// Implements `Index` for `Vector<K>`.
impl<K> Index<usize> for Vector<K>
where
    K:  FloatOrComplex
{
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

/// Implements `IndexMut` for `Vector<K>`.
impl<K> IndexMut<usize> for Vector<K>
where
    K: FloatOrComplex
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// Implements `FromIterator` for `Vector<K>` to be able to use `collect` method.
impl<K> FromIterator<K> for Vector<K>
where
    K:  FloatOrComplex
{
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let collected_data: Vec<K> = iter.into_iter().collect();        
        Vector {data: collected_data}
    }
}

// Implement fmt::Display trait to be able to print `Vector<K>`.
impl<K> fmt::Display for Vector<K> 
where
    K:  FloatOrComplex + fmt::Display
{
    /// Format and print the data of `Vector<K>`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let mut iter = self.data.iter().peekable();
        while let Some(element) = iter.next() {
            if element.close_to_zero() {
                write!(f, "0.0")?;
            }
            else {
                write!(f, "{:.1}", element)?;
            }

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
    /// Adds a vector to `self`
    /// When trying to add a vector of different size, result is undefined.
    fn add(&mut self, v: &Vector<K>) {
        // if self.get_size() == v.get_size() {
            for (elem_1, elem_2) in self.data.iter_mut().zip(v.data.iter()) {
                *elem_1 = elem_1.clone() + elem_2.clone();
            }
        // }
    }

    /// Substraction of a vector to `self`.
    /// When trying to substract a vector of different size, result is undefined.
    fn sub(&mut self, v: &Vector<K>) {
        // if self.get_size() == v.get_size() {
            for (elem_1, elem_2) in self.data.iter_mut().zip(v.data.iter()) {
                *elem_1 = elem_1.clone() - elem_2.clone();
            }
        // }     
    }

    /// Scaling of `self` by a scalar.
    fn scl(&mut self, a: K) {
        for element in self.data.iter_mut() {
            *element = element.clone() * a.clone();
        }
    }

}

impl<K> Vector<K>
where
    K:  FloatOrComplex + std::ops::Mul + std::iter::Sum<<K as std::ops::Mul>::Output> + Clone
{
    /// Compute the dot product of two vectors of the same dimension.
    /// If both vectors have different dimensions, the behavior is undefined.
    pub fn dot(&self, v: &Vector::<K>) -> K {
        self.clone()
            .into_iter()
            .clone()
            .zip(v.clone().into_iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}


impl<K> Vector<K>
where
    K: FloatOrComplex + Clone
{
    /// Returns the L1 norm (Taxicab or Manhattan norm) of `Vector<K>`.
    pub fn norm_1(&mut self) -> f32 {
        self.clone()
            .into_iter()
            .map(|elem| elem.norm_value())
            .sum()
    }

    /// Returns the L2 norm (Euclidian norm) of `Vector<K>`.
    pub fn norm(&mut self) -> f32 {
        let squared_sum: f32 = self.clone()
            .into_iter()
            .map(|elem| elem.norm_value() * elem.norm_value())
            .sum();

        squared_sum.sqrt()
    }

    /// Returns the L∞ norm (Supremum or Maximum or Infinity norm) of `Vector<K>`.
    /// Return 0.0 when `Vector<K>` is empty.
    pub fn norm_inf(&mut self) -> f32 {
        let result: Option<K> = self.clone()
            .into_iter()
            .max_by(|x, y| x.norm_value().partial_cmp(&y.norm_value()).unwrap());

        match result {
            Some(x) => return x.norm_value(),
            None => return 0.0
        }
    }
}