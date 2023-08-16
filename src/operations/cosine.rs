use std::iter::Sum;

use crate::{Vector, traits::{FloatOrComplex, MathDisplay}};

/// Compute the cosine of the angle between two vectors
pub fn angle_cos<K>(u: &Vector::<K>, v: &Vector::<K>) -> f32
where
    K:  FloatOrComplex + MathDisplay + Clone + Sum + std::ops::Div<f32, Output = f32>
{
    u.dot(v) / (u.clone().norm() * v.clone().norm())
}