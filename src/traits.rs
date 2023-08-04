use num_complex::Complex;
use num_traits::Num;

/// A trait for the Addition, Substraction and Scaling of both a vector and a matrix.
pub trait AddSubScl<T, K> { // will need to add a complex type later
    fn add(&mut self, t: &T);
    fn sub(&mut self, t: &T);
    fn scl(&mut self, a: K);
}

/// A trait to be implemented for the f32 or Complex type
pub trait FloatOrComplex {}

// Implement the trait for the types needed
impl<K: Num + Copy> FloatOrComplex for Complex<K> {}  // Complex numbers
impl FloatOrComplex for f32 {}  // Float numbers
