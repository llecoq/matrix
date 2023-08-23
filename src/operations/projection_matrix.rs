use crate::Matrix;

/// Computes a projection matrix to be used to render 3D objects.
pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix::<f32> {

    let scale_y: f32 = 1.0 / f32::tan(fov / 2.0);
    let scale_x: f32 = scale_y / ratio;
   
    let a = (far + near) / (near - far);
    let b = (2.0 * far * near) / (near - far);

    let projection_matrix: Matrix<f32> = Matrix::from(vec![
        vec![scale_x, 0., 0., 0.],
        vec![0., scale_y, 0., 0.],
        vec![0., 0., a, b],
        vec![0., 0., -1., 0.]
    ]).unwrap();
   
    projection_matrix
}