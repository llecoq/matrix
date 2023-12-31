use num_complex::Complex;

use crate::{Matrix, tests::test_utils::{assert_output, matrix_to_string, compare_matrices}};

#[test]
fn f32_row_echelon_tests() {
    let u = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]).unwrap();
    let result: Vec<Vec<f32>> = vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ];
    assert_output(&u.row_echelon(), matrix_to_string(&result).as_str());
  
    let u = Matrix::from(vec![
        vec![1., 2.],
        vec![3., 4.],
    ]).unwrap();
    let result: Vec<Vec<f32>> = vec![
        vec![1., 0.],
        vec![0., 1.],
    ];
    assert_output(&u.row_echelon(), matrix_to_string(&result).as_str());
    
    let u = Matrix::from(vec![
        vec![1., 2.],
        vec![2., 4.],
    ]).unwrap();
    let result: Vec<Vec<f32>> = vec![
        vec![1., 2.],
        vec![0., 0.],
    ];
    assert_output(&u.row_echelon(), matrix_to_string(&result).as_str());
    
    let u = Matrix::from(vec![
        vec![8., 5., -2., 4., 28.],
        vec![4., 2.5, 20., 4., -4.],
        vec![8., 5., 1., 4., 17.],
    ]).unwrap();
    let result: Matrix<f32> = Matrix::from(vec![
        vec![1.0, 0.625, 0.0, 0.0, -12.1666667],
        vec![0.0, 0.0, 1.0, 0.0, -3.6666667],
        vec![0.0, 0.0, 0.0, 1.0, 29.5],
    ]).unwrap();
    compare_matrices(u.row_echelon(), result);

    let u = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]).unwrap();
    let result: Matrix<f32> = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]).unwrap();
    compare_matrices(u.row_echelon(), result);

    let u = Matrix::from(vec![
        vec![1., 2.],
        vec![3., 4.],
    ]).unwrap();
    let result: Matrix<f32> = Matrix::from(vec![
        vec![1., 0.],
        vec![0., 1.],
    ]).unwrap();
    compare_matrices(u.row_echelon(), result);

    let u = Matrix::from(vec![
        vec![1., 2.],
        vec![2., 4.],
    ]).unwrap();
    let result: Matrix<f32> = Matrix::from(vec![
        vec![1., 2.],   
        vec![0., 0.],
    ]).unwrap();
    compare_matrices(u.row_echelon(), result);

    let u = Matrix::from(vec![
        vec![8., 5., -2., 4., 28.],
        vec![4., 2.5, 20., 4., -4.],
        vec![8., 5., 1., 4., 17.],
    ]).unwrap();
    let result: Matrix<f32> = Matrix::from(vec![
        vec![1.0, 0.625, 0.0, 0.0, -12.1666667],
        vec![0.0, 0.0, 1.0, 0.0, -3.6666667],
        vec![0.0, 0.0, 0.0, 1.0, 29.5],
    ]).unwrap();
    compare_matrices(u.row_echelon(), result);

}

#[test]
fn complex_row_echelon_tests() {
    let u = Matrix::from(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]).unwrap();
    let result = Matrix::from(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]).unwrap();
    compare_matrices(u.row_echelon(), result);

    let u = Matrix::from(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)],
        vec![Complex::new(3.0, 0.0), Complex::new(4.0, 0.0)],
    ]).unwrap();
    let result = Matrix::from(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]).unwrap();
    compare_matrices(u.row_echelon(), result);

    let u = Matrix::from(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)],
        vec![Complex::new(2.0, 0.0), Complex::new(4.0, 0.0)],
    ]).unwrap();
    let result = Matrix::from(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
    ]).unwrap();
    compare_matrices(u.row_echelon(), result);

    let u = Matrix::from(vec![
        vec![Complex::new(8.0, 0.0), Complex::new(5.0, 0.0), Complex::new(-2.0, 0.0), Complex::new(4.0, 0.0), Complex::new(28.0, 0.0)],
        vec![Complex::new(4.0, 0.0), Complex::new(2.5, 0.0), Complex::new(20.0, 0.0), Complex::new(4.0, 0.0), Complex::new(-4.0, 0.0)],
        vec![Complex::new(8.0, 0.0), Complex::new(5.0, 0.0), Complex::new(1.0, 0.0), Complex::new(4.0, 0.0), Complex::new(17.0, 0.0)],
    ]).unwrap();
    let result = Matrix::from(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(0.625, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(-12.1666667, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(-3.6666667, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(29.5, 0.0)],
    ]).unwrap();
    compare_matrices(u.row_echelon(), result);
}