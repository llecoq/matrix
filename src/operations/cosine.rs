use std::iter::Sum;

use crate::{Vector, traits::{FloatOrComplex, MathDisplay}};

/// Compute the cosine of the angle between two vectors
/// If one or both vectors are zero, the behaviour is undefined.
/// If the vectors are of different sizes, the behavior is undefined.
/// Returns NaN if one vector is empty
pub fn angle_cos<K>(u: &Vector::<K>, v: &Vector::<K>) -> f32
where
    K:  FloatOrComplex + MathDisplay + Clone + Sum + std::ops::Div<f32, Output = f32>, f32: Sum<K>
{
    u.dot(v) / (u.clone().norm() * v.clone().norm())
}