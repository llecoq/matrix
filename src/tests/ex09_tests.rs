use num_complex::Complex;

use crate::Matrix;

use super::test_utils::{assert_output, matrix_to_string};

#[test]
fn f32_transpose_tests() {
    let u = Matrix::from(vec![
        vec![1., 0.],
        vec![0., 1.],
        vec![0., 1.],
        vec![1., 0.],
    ]).unwrap();
    let result: Vec<Vec<f32>> = vec![
        vec![1., 0., 0., 1.],
        vec![0., 1., 1., 0.],
    ];
    assert_output(&u.transpose(), matrix_to_string(&result).as_str());

    let u = Matrix::from(vec![
        vec![2., -5., 0.],
        vec![4., 3., 7.],
        vec![-2., 3., 4.],
        vec![-2., 3., 4.],
    ]).unwrap();
    let result: Vec<Vec<f32>> = vec![
        vec![2., 4., -2., -2.],
        vec![-5., 3., 3., 3.],
        vec![0., 7., 4., 4.],
    ];
    assert_output(&u.transpose(), matrix_to_string(&result).as_str());

    let u = Matrix::from(vec![
        vec![-2., -8., 4.],
        vec![1., -23., 4.],
        vec![0., 6., 4.],
    ]).unwrap();
    let result: Vec<Vec<f32>> = vec![
        vec![-2., 1., 0.],
        vec![-8., -23., 6.],
        vec![4., 4., 4.],
    ];
    assert_output(&u.transpose(), matrix_to_string(&result).as_str());

    let u = Matrix::from(vec![
        vec![0., -8., 4.],
        vec![1., -0., 4.],
        vec![0., 6., 0.],
    ]).unwrap();
    let result: Vec<Vec<f32>> = vec![
        vec![0., 1., 0.],
        vec![-8., 0., 6.],
        vec![4., 4., 0.],
    ];
    assert_output(&u.transpose(), matrix_to_string(&result).as_str());

    let u = Matrix::from(vec![
        vec![0., -8., 4.],
        vec![1., 50., 4.],
        vec![0., 6., 0.],
    ]).unwrap();
    let result: Vec<Vec<f32>> = vec![
        vec![0., 1., 0.],
        vec![-8., 50., 6.],
        vec![4., 4., 0.],
    ]; 
    assert_output(&u.transpose(), matrix_to_string(&result).as_str());

    let u = Matrix::from(vec![
        vec![0., -8., 4.],
        vec![1., -0., 4.],
        vec![0., 6., 0.],
        vec![0., 6., 0.],
    ]).unwrap();
    let result: Vec<Vec<f32>> = vec![
        vec![0., 1., 0., 0.],
        vec![-8., 0., 6., 6.],
        vec![4., 4., 0., 0.],
    ]; 
    assert_output(&u.transpose(), matrix_to_string(&result).as_str());

    // Transposing a 2x3 matrix
    let u = Matrix::from(vec![
        vec![1., 2., 3.],
        vec![4., 5., 6.],
    ]).unwrap();
    let result: Vec<Vec<f32>> = vec![
        vec![1., 4.],
        vec![2., 5.],
        vec![3., 6.],
    ];
    assert_output(&u.transpose(), matrix_to_string(&result).as_str());

    // Transposing a 4x1 matrix
    let u = Matrix::from(vec![
        vec![1.],
        vec![2.],
        vec![3.],
        vec![4.],
    ]).unwrap();
    let result: Vec<Vec<f32>> = vec![
        vec![1., 2., 3., 4.]
    ];
    assert_output(&u.transpose(), matrix_to_string(&result).as_str());

    // Transposing a 3x4 matrix
    let u = Matrix::from(vec![
        vec![1., 2., 3., 4.],
        vec![5., 6., 7., 8.],
        vec![9., 10., 11., 12.],
    ]).unwrap();
    let result: Vec<Vec<f32>> = vec![
        vec![1., 5., 9.],
        vec![2., 6., 10.],
        vec![3., 7., 11.],
        vec![4., 8., 12.],
    ];
    assert_output(&u.transpose(), matrix_to_string(&result).as_str());

    // Transposing a 3x2 matrix
    let u = Matrix::from(vec![
        vec![1., 4.],
        vec![2., 5.],
        vec![3., 6.],
    ]).unwrap();
    let result: Vec<Vec<f32>> = vec![
        vec![1., 2., 3.],
        vec![4., 5., 6.],
    ];
    assert_output(&u.transpose(), matrix_to_string(&result).as_str());

}

#[test]
fn complex_transpose_tests() {

    // Transposing a 3x3 matrix (square matrix) with complex numbers
    let u = Matrix::from(vec![
        vec![Complex::new(1.0, 2.0), Complex::new(3.0, 4.0), Complex::new(5.0, 6.0)],
        vec![Complex::new(7.0, 8.0), Complex::new(9.0, 0.0), Complex::new(1.0, 2.0)],
        vec![Complex::new(3.0, 4.0), Complex::new(5.0, 6.0), Complex::new(7.0, 8.0)]
    ]).unwrap();
    let result: Vec<Vec<Complex<f32>>> = vec![
        vec![Complex::new(1.0, 2.0), Complex::new(7.0, 8.0), Complex::new(3.0, 4.0)],
        vec![Complex::new(3.0, 4.0), Complex::new(9.0, 0.0), Complex::new(5.0, 6.0)],
        vec![Complex::new(5.0, 6.0), Complex::new(1.0, 2.0), Complex::new(7.0, 8.0)]
    ];
    assert_output(&u.transpose(), matrix_to_string(&result).as_str());

    let u = Matrix::from(vec![
        vec![Complex::new(1.0, 2.0), Complex::new(2.0, 0.0), Complex::new(3.0, 4.0), Complex::new(5.0, 6.0)],
        vec![Complex::new(1.0, 2.0), Complex::new(1.0, 2.0), Complex::new(1.0, 2.0), Complex::new(1.0, 2.0)]
    ]).unwrap();
    let result: Vec<Vec<Complex<f32>>> = vec![
        vec![Complex::new(1.0, 2.0), Complex::new(1.0, 2.0)],
        vec![Complex::new(2.0, 0.0), Complex::new(1.0, 2.0)],
        vec![Complex::new(3.0, 4.0), Complex::new(1.0, 2.0)],
        vec![Complex::new(5.0, 6.0), Complex::new(1.0, 2.0)]
    ];
    assert_output(&u.transpose(), matrix_to_string(&result).as_str());

}