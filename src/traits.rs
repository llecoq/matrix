use core::fmt;
use std::ops::{Add, Sub, Mul};

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
// Implement the trait for f32 and Complex types
impl<K: Num + Copy> FloatOrComplex for Complex<K> {}  // Complex numbers
impl FloatOrComplex for f32 {}  // Float numbers

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