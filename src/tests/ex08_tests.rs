use crate::{Matrix, MatrixError};

use super::test_utils::{assert_output, floats_are_close};

#[test]
fn trace_tests() {
    let u = Matrix::from(vec![
        vec![1., 0.],
        vec![0., 1.],
    ]).unwrap();
    let result: Result<f32, MatrixError> = u.trace();
    assert_output(&result.unwrap(), "2");

    let u = Matrix::from(vec![
        vec![2., -5., 0.],
        vec![4., 3., 7.],
        vec![-2., 3., 4.],
    ]).unwrap();
    let result: Result<f32, MatrixError> = u.trace();
    assert_output(&result.unwrap(), "9");

    let u = Matrix::from(vec![
        vec![-2., -8., 4.],
        vec![1., -23., 4.],
        vec![0., 6., 4.],
    ]).unwrap();
    let result: Result<f32, MatrixError> = u.trace();
    assert_output(&result.unwrap(), "-21");

    let u = Matrix::from(vec![
        vec![0., -8., 4.],
        vec![1., -0., 4.],
        vec![0., 6., 0.],
    ]).unwrap();
    let result: Result<f32, MatrixError> = u.trace();
    assert_output(&result.unwrap(), "0");

    let u = Matrix::from(vec![
        vec![0., -8., 4.],
        vec![1., 50., 4.],
        vec![0., 6., 0.],
    ]).unwrap();
    let result: Result<f32, MatrixError> = u.trace();
    assert_output(&result.unwrap(), "50");

    let u = Matrix::from(vec![
        vec![1.1, 2., 3.6, 4.0, 5.3, 6.2, 7.6, 8., 9., 1., 2., 13.],
        vec![2.1, 3., 2.6, 5.0, 9.3, 4.2, 2.6, 4., 6., 1., 2., 13.],
        vec![3.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6., 1., 2., 13.],
        vec![4.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6., 1., 2., 13.],
        vec![5.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6., 1., 2., 13.],
        vec![6.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6., 1., 2., 13.],
        vec![7.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6., 1., 2., 13.],
        vec![8.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6., 1., 2., 13.],
        vec![9.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6., 1., 2., 13.],
        vec![10.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6., 1., 2., 13.],
        vec![11.1, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6., 1., 2., 13.],
        vec![12.3, 2., 4.5, 5.0, 9.3, 4.2, 2.6, 4., 6., 1., 2., 13.]
    ]).unwrap();
    let result: Result<f32, MatrixError> = u.trace();
    assert_eq!(floats_are_close(result.unwrap(), 55.7), true);

    // Not square
    let u = Matrix::from(vec![
        vec![0., -8., 4.],
        vec![1., -0., 4.],
        vec![0., 6., 0.],
        vec![0., 6., 0.],
    ]).unwrap();
    match u.trace() {
        Ok(result) => println!("{}", result),
        Err(err_msg) => println!("{}", err_msg)
    }


}