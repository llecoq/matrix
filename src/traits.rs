use num_complex::Complex;
use num_traits::Num;

/// A trait for the Addition, Substraction and Scaling of both a vector and a matrix.
pub trait AddSubScl<T, K> { // will need to add a complex type later
    fn add(&mut self, t: &T);
    fn sub(&mut self, t: &T);
    fn scl(&mut self, a: K);
}

/// A trait for the generic type K that need to be either numeric or complex
// Create a new trait that will be implemented by the types you are interested in.
pub trait ComplexOrNum {}

// Implement the trait for the types you care about.
impl<T: Num + Copy> ComplexOrNum for Complex<T> {}  // Complex numbers
impl ComplexOrNum for i32 {}  // Numeric types
impl ComplexOrNum for f64 {}  // Add more types as needed...
impl ComplexOrNum for f32 {}  // Add more types as needed...
