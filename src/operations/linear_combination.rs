use core::fmt;

use crate::{Vector, traits::FloatOrComplex, traits::AddSubScl};

/// Calculate the linear combination of the vectors of u scaled by their respective
/// coefficients.
/// If the two arrays provided as input are not of the same size, or if the array’s contents
/// are incoherent, the result is undefined.
#[allow(unused_variables)]
pub fn linear_combination<K: FloatOrComplex>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K:  fmt::Display
        + std::fmt::Display
        + std::ops::Sub<Output = K>
        + std::ops::Add<Output = K>
        + std::ops::Mul<Output = K>
        + Clone
{
    let mut scl_vectors: Vec<Vector<K>> = vec![];

    for (elem_1, elem_2) in u.iter().zip(coefs.iter()) {
        let mut vec_copy: Vector<K> = elem_1.clone();

        vec_copy.scl(elem_2.clone());
        scl_vectors.push(vec_copy);
    }

    if let Some(first_element) = scl_vectors.first() {
        let mut result = first_element.clone();

        for elem in scl_vectors.iter().skip(1).take(1) {
            result.add(&elem);
        }
        return result;
    }
    else {
        println!("Empty Vector input");
        return Vector::from(vec![]);
    }
}
