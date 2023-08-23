use crate::Matrix;

/// Computes a projection matrix to be used to render 3D objects.
pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix::<f32> {
   
    let a = (far + near) / (near - far);
    let b = (2.0 * far * near) / (near - far);

    Matrix::from(vec![
        vec![fov / ratio, 0., 0., 0.],
        vec![0., fov, 0., 0.],
        vec![0., 0., a, b],
        vec![0., 0., -1., 0.]
    ]).unwrap()
}