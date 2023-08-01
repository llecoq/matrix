use crate::traits::AddSubScl;

/// A basic vector structure.
#[allow(dead_code)]
pub struct Vector<K> {
    elements:   Vec<K>
}

//-------------------- Utility functions
#[allow(dead_code)]
impl<K> Vector<K> {

    /// Return the size of the vector.
    fn get_size(self) -> usize {
        self.elements.len()
    }

    /// Print a vector.
    fn print_vector(self) {
        // for element in &self.elements {
        //     println!("{:?}", element);
        // }
    }

    /// Reshape a vector into a matrix.
    fn reshape_into_matrix(self) {

    }
}

impl<K> AddSubScl<Vector<K>, K> for Vector<K> {
    /// Add a vector to another one
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut u = Vector::from([2., 3.]);
    /// let v = Vector::from([5., 7.]);
    /// u.add(v);
    /// assert_eq!();
    /// ```
    fn add(&mut self, v: &Vector<K>) {
        
    }
    /// Substraction of a vector by another one.
    fn sub(&mut self, v: &Vector<K>) {

    }
    /// Scaling of a vector by a scalar.
    fn scl(&mut self, a: K) {

    }
}