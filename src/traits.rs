use core::fmt;
use std::{ops::{Add, Sub, Mul}, f32::EPSILON, iter::Sum};

use num_complex::Complex;

//------------------------------------------------------------------------- AddSubScl
/// A trait for the Addition, Substraction and Scaling of both a vector and a matrix.
pub trait AddSubScl<T, K> {
    fn add(&mut self, t: &T);
    fn sub(&mut self, t: &T);
    fn scl(&mut self, a: K);
}

//-------------------------------------------------------------------- FloatOrComplex
/// A trait to be implemented for the f32 or Complex type
pub trait FloatOrComplex {
    fn norm_value(&self) -> f32;
    fn close_to_one(&self) -> bool;
    fn close_to_zero(&self) -> bool;
    fn scale_factor(&self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn divide(&self, denominator: f32) -> Self;
    fn real(&self) -> f32;
}

// Implement the trait for f32 and Complex types
impl FloatOrComplex for Complex<f32>
{
    /// Returns the normed value of `self`.
    fn norm_value(&self) -> f32 {
        self.norm()
    }

    /// Returns true if `self` is close to `one`.
    fn close_to_one(&self) -> bool {
        (self - Self::one()).norm() < EPSILON 
    }

    /// Returns true if `self` is close to `zero`.
    fn close_to_zero(&self) -> bool {
        self.norm_sqr() < EPSILON * EPSILON
    }

    /// Returns the scale factor of `self` to make it `one`.
    fn scale_factor(&self) -> Self {
        1.0 / self
    }

    /// Returns `zero`.
    fn zero() -> Self {
        Complex::new(0.0, 0.0)
    }

    /// Returns `one`.
    fn one() -> Self {
        Complex::new(1.0, 0.0)
    }

    /// Divides `self` by a f32 and returns it.
    fn divide(&self, denominator: f32) -> Self {
        self / denominator
    }
    
    /// Returns the real part of `self`.
    fn real(&self) -> f32 {
        self.re
    }

}  // Complex numbers

impl FloatOrComplex for f32 {
    /// Returns the absolute value of `self`.
    fn norm_value(&self) -> f32 {
        self.abs()
    }

    /// Returns true if `self` is close to `one`.
    fn close_to_one(&self) -> bool {
        (self - 1.0).abs() < EPSILON
    }
    
    /// Returns true if `self` is close to `zero`.
    fn close_to_zero(&self) -> bool {
        self.abs() < EPSILON
    }

    /// Returns the scale factor of `self` to make it `one`.
    fn scale_factor(&self) -> Self {
        1.0 / self
    }

    /// Returns `zero`.
    fn zero() -> Self {
        0.0
    }

    /// Returns `one`.
    fn one() -> Self {
        1.0
    }

    /// Divides `self` by a f32 and returns it.
    fn divide(&self, denominator: f32) -> Self {
        self / denominator
    }

    /// Returns the real part of `self`.
    fn real(&self) -> f32 {
        *self
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