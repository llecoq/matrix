use crate::{Vector, traits::{FloatOrComplex, MathDisplay}};

/// Computes the cross product of two 3-dimensional vectors.
/// If one or both vectors are not 3-dimensional, returns an empty Vector<K>.
pub fn cross_product<K>(u: &Vector::<K>, v: &Vector::<K>) -> Vector::<K>
where
    K:  FloatOrComplex + MathDisplay + Copy
{
    let a1 = u.get(0);
    let a2 = u.get(1);
    let a3 = u.get(2);
    let b1 = v.get(0);
    let b2 = v.get(1);
    let b3 = v.get(2);

    match (a1, a2, a3, b1, b2, b3) {
        (Some(a_x), Some(a_y), Some(a_z), Some(b_x), Some(b_y), Some(b_z)) => {
            let x: K = *a_y * *b_z - *a_z * *b_y;
            let y: K = *a_z * *b_x - *a_x * *b_z;
            let z: K = *a_x * *b_y - *a_y * *b_x;

            Vector::from(vec![x, y, z])
        }
        _ => Vector::new()
    }
}