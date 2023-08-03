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
impl<K> Vector<K> {
    /// Associated constructor `from`
    pub fn from(vec_data: Vec<K>) -> Vector<K> { 
        Vector { data: vec_data }
    }
}

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
impl<K> AddSubScl<Vector<K>, K> for Vector<K> 
where 
    K:  Clone 
        + std::fmt::Display
        + std::ops::Sub<Output = K>
        + std::ops::Add<Output = K>
        + std::ops::Mul<Output = K>
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
    /// Attention ! When trying to add a vector of different size, add does nothing.
    fn add(&mut self, v: &Vector<K>) {
        if self.get_size() == v.get_size() {
            for (element_1, element_2) in self.data.iter_mut().zip(v.data.iter()) {
                *element_1 = element_1.clone() + element_2.clone();
            }
        }
    }

    /// Substraction of a vector by another one.
    /// Attention ! When trying to substract a vector of different size, sub does nothing.
    fn sub(&mut self, v: &Vector<K>) {
        if self.get_size() == v.get_size() {
            for (element_1, element_2) in self.data.iter_mut().zip(v.data.iter()) {
                *element_1 = element_1.clone() - element_2.clone();
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
