use core::fmt;
use std::ops::{Add, Sub, Mul};

use num_complex::Complex;
use num_traits::Num;

//------------------------------------------------------------------------- AddSubScl
/// A trait for the Addition, Substraction and Scaling of both a vector and a matrix.
pub trait AddSubScl<T, K> { // will need to add a complex type later
    fn add(&mut self, t: &T);
    fn sub(&mut self, t: &T);
    fn scl(&mut self, a: K);
}


//-------------------------------------------------------------------- FloatOrComplex
/// A trait to be implemented for the f32 or Complex type
pub trait FloatOrComplex {
    fn norm_value(&self) -> f32;
    fn squares(&self) -> f32;
}

// Implement the trait for f32 and Complex types
impl<K: Num + Copy> FloatOrComplex for Complex<K> {
    /// Returns the normed value of the complex number.
    fn norm_value(&self) -> f32 {
        2. // will be handled later on
    }
    /// Squares the complex number and returns it.
    fn squares(&self) -> f32 {
        2.
    }
}  // Complex numbers

impl FloatOrComplex for f32 {
    /// Returns the absolute value of the number.
    fn norm_value(&self) -> f32 {
        self.abs()
    }
    /// Squares the number and returns it.
    fn squares(&self) -> f32 {
        self * self
    }
}  // Float numbers


//-------------------------------------------------------------------- MathDisplay
/// A trait including Display and mathematical operations
pub trait MathDisplay:
    fmt::Display
    + Add<Output = Self>
    + Sized + Sub<Output = Self>
    + Mul<Output = Self>
{}
// Blanket implementation
impl<K> MathDisplay for K where K:
    fmt::Display
    + Add<Output = K>
    + Sized + Sub<Output = K>
    + Mul<Output = K>
{}