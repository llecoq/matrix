use crate::traits::AddSubScl;

/// Computes a linear interpolation between two objects of the same type.
/// If the objects are not of the same size, result is undefined.
pub fn lerp<V, K>(u: &V, v: &V, t: f32) -> V
where
    V:  AddSubScl<V, K> + Clone,
    K: From<f32> + std::ops::Sub<Output = K>
{
    let mut result: V = u.clone();
    let mut v_clone: V = v.clone();

    result.scl(K::from(1.0) - K::from(t));
    v_clone.scl(K::from(t));
    result.add(&v_clone);
    result
}