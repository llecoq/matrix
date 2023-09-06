use crate::Matrix;

/// Computes a projection matrix to be used to render 3D objects.
pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix::<f32> {

    let a = 1. / ((fov / 2.).tan() * ratio);
    let b = 1. / ((fov / 2.).tan());
    let c: f32 = -(far + near) / (far - near);
    let d: f32 = -2. * far * near / (far - near);

    Matrix::from(vec![
        vec![a, 0., 0., 0.],
        vec![0., b, 0., 0.],
        vec![0., 0., c, d],
        vec![0., 0., -1., 0.]
    ]).unwrap()
}