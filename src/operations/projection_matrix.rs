use crate::Matrix;

/// Computes a projection matrix to be used to render 3D objects.
pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix::<f32> {

    // Compute the scale factor based on the field of view.
    let scale = f32::tan(fov * 0.5) * near;

    // Compute the right and left, top and bottom values for the frustum.
    let right = ratio * scale;
    let left = -right;
    let top = scale;
    let bottom = -top;

    // Initialize the projection matrix to all zeros.
    let mut projection_matrix: Matrix<f32> = Matrix::from(vec![
        vec![0.0,0.0,0.0,0.0],
        vec![0.0,0.0,0.0,0.0],
        vec![0.0,0.0,0.0,0.0],
        vec![0.0,0.0,0.0,0.0],
    ]).unwrap();

    // Assign values to the matrix based on the frustum.
    projection_matrix[0][0] = (2.0 * near) / (right - left);
    projection_matrix[1][1] = (2.0 * near) / (top - bottom);
    projection_matrix[2][2] = -(far + near) / (far - near);
    projection_matrix[2][3] = -1.0;
    projection_matrix[3][2] = -(2.0 * far * near) / (far - near);

    projection_matrix

}