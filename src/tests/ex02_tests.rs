use crate::{Vector, Matrix, tests::test_utils::assert_output, lerp};

use super::test_utils::matrix_to_string;

#[test]
fn linear_interpolation_tests() {
    let v1: Vector<f32> = Vector::from(vec![0.]);
    let v2: Vector<f32> = Vector::from(vec![1.]);
    assert_output(&lerp(&v1, &v2, 0.), "[0.0]");

    let v1: Vector<f32> = Vector::from(vec![0.]);
    let v2: Vector<f32> = Vector::from(vec![1.]);
    assert_output(&lerp(&v1, &v2, 1.), "[1.0]");

    let v1: Vector<f32> = Vector::from(vec![0.]);
    let v2: Vector<f32> = Vector::from(vec![1.]);
    assert_output(&lerp(&v1, &v2, 0.5), "[0.5]");

    let v1: Vector<f32> = Vector::from(vec![21.]);
    let v2: Vector<f32> = Vector::from(vec![42.]);
    assert_output(&lerp(&v1, &v2, 0.3), "[27.3]");

    let v1: Vector<f32> = Vector::from(vec![2., 1.]);
    let v2: Vector<f32> = Vector::from(vec![4., 2.]);
    assert_output(&lerp(&v1, &v2, 0.3), "[2.6][1.3]");

    let input = vec![
        vec![2., 1.],
        vec![3., 4.]
    ];
    let matrix = Matrix::from(input.clone()).unwrap();
    let input_2 = vec![
        vec![20., 10.],
        vec![30., 40.]
    ];
    let matrix_2 = Matrix::from(input_2.clone()).unwrap();
    let result = vec![
        vec![11., 5.5],
        vec![16.5, 22.]
    ];
    assert_output(&lerp(&matrix, &matrix_2, 0.5), &matrix_to_string(&result));

    // udefined result because of different sizes
    let input = vec![
        vec![2., 1., 6.3],
        vec![3., 4., 9.6]
    ];
    let matrix = Matrix::from(input.clone()).unwrap();
    let input_2 = vec![
        vec![20., 10.],
        vec![30., 40.]
    ];
    let matrix_2 = Matrix::from(input_2.clone()).unwrap();
    let result = vec![
        vec![11., 5.5],
        vec![16.5, 22.]
    ];
    assert_output(&lerp(&matrix_2, &matrix, 0.5), &matrix_to_string(&result));    

    println!("{}", matrix);

}