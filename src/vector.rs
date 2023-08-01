use crate::traits::AddSubScl;

/// A basic vector structure.
pub struct Vector<K> {
    elements:   Vec<K>
}

//-------------------- Utility functions
impl<K> Vector<K> {
    // fn get_size {}
    // print_vector {}
    // reshape_into_matrix {}
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