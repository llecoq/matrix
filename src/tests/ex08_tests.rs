use num_complex::Complex;

use crate::{Matrix, MatrixError};

use super::test_utils::{assert_output, numbers_are_close};

#[test]
fn f32_trace_tests() {

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
    assert_eq!(numbers_are_close(result.unwrap(), 55.7), true);

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
    let u = Matrix::from(vec![
        vec![3., 4.],
        vec![5., 6.],
    ]).unwrap();
    let result: Result<f32, MatrixError> = u.trace();
    assert_output(&result.unwrap(), "9");

    let u = Matrix::from(vec![
        vec![7., 8., 9.],
        vec![10., 11., 12.],
        vec![13., 14., 15.],
    ]).unwrap();
    let result: Result<f32, MatrixError> = u.trace();
    assert_output(&result.unwrap(), "33");

    let u = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., -2., 0.],
        vec![0., 0., 3.],
    ]).unwrap();
    let result: Result<f32, MatrixError> = u.trace();
    assert_output(&result.unwrap(), "2");

    let u = Matrix::from(vec![
        vec![4., 0., 0., 0.],
        vec![0., 5., 0., 0.],
        vec![0., 0., 6., 0.],
        vec![0., 0., 0., 7.],
    ]).unwrap();
    let result: Result<f32, MatrixError> = u.trace();
    assert_output(&result.unwrap(), "22");
}

#[test]
fn complex_trace_tests() {
    let u = Matrix::from(vec![
        vec![Complex::new(2., 1.), Complex::new(0., -1.)],
        vec![Complex::new(1., -1.), Complex::new(2., 1.)],
    ]).unwrap();
    let result: Result<Complex<f32>, MatrixError> = u.trace();
    assert_output(&result.unwrap(), "4+2i");  

    let u = Matrix::from(vec![
        vec![Complex::new(1., 2.), Complex::new(3., 4.), Complex::new(5., 6.)],
        vec![Complex::new(7., 8.), Complex::new(9., 10.), Complex::new(11., 12.)],
        vec![Complex::new(13., 14.), Complex::new(15., 16.), Complex::new(17., 18.)],
    ]).unwrap();
    let result: Result<Complex<f32>, MatrixError> = u.trace();
    assert_output(&result.unwrap(), "27+30i");  

    let u = Matrix::from(vec![
        vec![Complex::new(0., 1.), Complex::new(1., 0.)],
        vec![Complex::new(2., 0.), Complex::new(0., 2.)],
    ]).unwrap();
    let result: Result<Complex<f32>, MatrixError> = u.trace();
    assert_output(&result.unwrap(), "0+3i");  

    let u = Matrix::from(vec![
        vec![Complex::new(3., 0.), Complex::new(0., 3.)],
        vec![Complex::new(4., 5.), Complex::new(6., 7.)],
    ]).unwrap();
    let result: Result<Complex<f32>, MatrixError> = u.trace();
    assert_output(&result.unwrap(), "9+7i");  
}
