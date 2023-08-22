use core::fmt;
use std::{ops::{Add, Sub, Mul}, f32::EPSILON, iter::Sum};

// use num_complex::{Complex, ComplexFloat};
// use num_traits::Num;

//------------------------------------------------------------------------- AddSubScl
/// A trait for the Addition, Substraction and Scaling of both a vector and a matrix.
pub trait AddSubScl<T, K> { // will need to add a complex type later
    fn add(&mut self, t: &T);
    fn sub(&mut self, t: &T);
    fn scl(&mut self, a: K);
}


//-------------------------------------------------------------------- FloatOrComplex
/// A trait to be implemented for the f32 or Complex type
#[allow(unused_must_use)]
pub trait FloatOrComplex {
    fn norm_value(&self) -> f32;
    fn squares(&self) -> f32;
    fn close_to_one(&self) -> bool;
    fn close_to_zero(&self) -> bool;
    fn scale_factor(&self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}

// // Implement the trait for f32 and Complex types
// impl<K: Num + Copy> FloatOrComplex for Complex<K> {
//     /// Returns the normed value of the complex number.
//     fn norm_value(&self) -> f32 {
//         2. // will be handled later on
//     }
//     /// Squares the complex number and returns it.
//     fn squares(&self) -> f32 {
//         2. // will be handled later on
//     }
//     /// Divides the complex number by a given denominator.
//     fn divide(&self, numerator: &f32) -> Self {
//         numerator / self // will be handled later on
//     }

//     fn close_to_one(&self) -> bool {
//         false // will be handled later on
//     }

// }  // Complex numbers

impl FloatOrComplex for f32 {
    /// Returns the absolute value of the number.
    fn norm_value(&self) -> f32 {
        self.abs()
    }
    /// Squares the number and returns it.
    fn squares(&self) -> f32 {
        self * self
    }

    /// Returns true if the float is close to one.
    fn close_to_one(&self) -> bool {
        (self - 1.0).abs() < EPSILON
    }
    
    /// Returns true if the float is close to zero.
    fn close_to_zero(&self) -> bool {
        self.abs() < EPSILON
    }

    /// Returns the scale factor of a float to make it 1.0.
    fn scale_factor(&self) -> Self {
        1.0 / self
    }

    /// Returns zero.
    fn zero() -> Self {
        0.0
    }

    /// Returns one.
    fn one() -> Self {
        1.0
    }

}  // Float numbers


//-------------------------------------------------------------------- MathDisplay
/// A trait including Display and mathematical operations
pub trait MathDisplay:
    fmt::Display
    + Add<Output = Self>
    + Sized + Sub<Output = Self>
    + Mul<Output = Self>
    + Sum
{}
// Blanket implementation
impl<K> MathDisplay for K where K:
    fmt::Display
    + Add<Output = K>
    + Sized + Sub<Output = K>
    + Mul<Output = K>
    + Sum
{}