use crate::traits::AddSubScl;

/// Computes a linear interpolation between two objects of the same type.
/// If the objects are not of the same size, result is undefined.
#[allow(unused_variables)]
pub fn lerp<V>(u: &V, v: &V, t: f32) -> V
where
    V:  AddSubScl<V, f32> + Clone
{
    let mut result: V = u.clone();
    let mut v_clone: V = v.clone();

    result.scl(1.0 - t);
    v_clone.scl(t);
    result.add(&v_clone);
    result
}