use crate::traits::AddSubScl;

/// A basic vector structure.
#[allow(dead_code)]
pub struct Vector<K> {
    size: usize,
    data: Vec<K>
}

//-------------------- Utility functions
#[allow(dead_code)]
impl<K> Vector<K> {

    /// Associated constructor
    fn from(vec: Vec<K>) -> Vector<K> { 
       Vector { size: vec.len(), data: vec }
    }

    /// Return the size of the vector.
    fn get_size(self) -> usize {
        self.data.len()
    }

    /// Print a vector.
    fn print_vector(self) {
        // for element in &self.data {
        //     println!("{:?}", element);
        // }
    }

    /// Reshape a vector into a matrix.
    fn reshape_into_matrix(self) {

    }
}
#[allow(unused_variables)]
impl<K> AddSubScl<Vector<K>, K> for Vector<K> 
where 
    K: Clone + std::ops::Add<Output = K> 
{
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
        if self.size == v.size {
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

#[test]
fn test_vector_utility_function() {
    let mut u = Vector::from(vec![2., 3.]);
    let v = Vector::from(vec![5., 7.]);
    u.add(&v);
    println!("{}", u);
}