use core::fmt;

use crate::traits::AddSubScl;

//----------------------------------------------------- Structure
/// A basic vector structure.
#[derive(Clone, Debug)]
pub struct Vector<K> {
    data: Vec<K>
}

//--------------------------------------------- Utility functions
impl<K> Vector<K> 
where
    K:  std::fmt::Display
{

    /// Associated constructor
    pub fn from(vec: Vec<K>) -> Vector<K> { 
       Vector { data: vec }
    }

    /// Return the size of the vector.
    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    /// Print a vector.
    pub fn print_vector(&self) {
        println!("{}", self);
    }

    /// Reshape a vector into a matrix.
    pub fn reshape_into_matrix(&self) {

    }
}


//----------------------------------------- Traits Implementation
// Implement fmt::Display trait to be able to print Vector<K>
impl<K> fmt::Display for Vector<K> 
where
    K: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let mut iter = self.data.iter().peekable();
        while let Some(element) = iter.next() {
            write!(f, "{:.1}", element)?;

            if let Some(_) = iter.peek() {
                write!(f, "]\n[")?;
            }
        }

        write!(f, "]")
    }
} 

// Add, Substract and Scale
#[allow(unused_variables)]
impl<K> AddSubScl<Vector<K>, K> for Vector<K> 
where 
    K:  Clone 
        + std::fmt::Display
        + std::ops::Add<Output = K> 
{
    /// Add a vector to another one
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut u = Vector::from([2., 3.]);
    /// let v = Vector::from([5., 7.]);
    /// u.add(v);
    /// ```
    fn add(&mut self, v: &Vector<K>) {
        if self.get_size() == v.get_size() {
            for (elem1, elem2) in self.data.iter_mut().zip(v.data.iter()) {
                *elem1 = elem1.clone() + elem2.clone();
            }
        }
    }
    /// Substraction of a vector by another one.
    fn sub(&mut self, v: &Vector<K>) {

    }
    /// Scaling of a vector by a scalar.
    fn scl(&mut self, a: K) {

    }
}


//---------------------------------------------------- Unit test
#[test]
fn test_vector_utility_function() {
    let mut u = Vector::from(vec![2., 3., 6.]);
    let v = Vector::from(vec![5., 7.1, 2.]);
    let x: Vector<f32> = Vector::from(vec![1.]);
    u.add(&v);
    println!("{}", u);
    x.print_vector();
}