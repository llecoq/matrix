use num_complex::Complex;

use crate::{Matrix, tests::test_utils::compare_matrices};

#[test]
fn f32_inverse_tests() {
    let u = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]).unwrap();
    let result = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]).unwrap();
    compare_matrices(u.inverse().unwrap(), result);

    let u = Matrix::from(vec![
        vec![2., 0., 0.],
        vec![0., 2., 0.],
        vec![0., 0., 2.],
    ]).unwrap();
    let result = Matrix::from(vec![
        vec![0.5, 0., 0.],
        vec![0., 0.5, 0.],
        vec![0., 0., 0.5],
    ]).unwrap();
    compare_matrices(u.inverse().unwrap(), result);
    
    let u = Matrix::from(vec![
        vec![8., 5., -2.],
        vec![4., 7., 20.],
        vec![7., 6., 1.],
    ]).unwrap();
    let result = Matrix::from(vec![
        vec![0.649425287, 0.097701149, -0.655172414],
        vec![-0.781609195, -0.126436782, 0.965517241],
        vec![0.143678161, 0.074712644, -0.206896552],
    ]).unwrap();
    compare_matrices(u.inverse().unwrap(), result);

    let u = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]).unwrap();
    let result = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]).unwrap();
    compare_matrices(u.inverse().unwrap(), result);
    
    let u = Matrix::from(vec![
        vec![2., 0., 0.],
        vec![0., 2., 0.],
        vec![0., 0., 2.],
    ]).unwrap();
    let result = Matrix::from(vec![
        vec![0.5, 0., 0.],
        vec![0., 0.5, 0.],
        vec![0., 0., 0.5],
    ]).unwrap();
    compare_matrices(u.inverse().unwrap(), result);

    let u = Matrix::from(vec![
        vec![8., 5., -2.],
        vec![4., 7., 20.],
        vec![7., 6., 1.],
    ]).unwrap();
    let result = Matrix::from(vec![
        vec![0.649425287, 0.097701149, -0.655172414],
        vec![-0.781609195, -0.126436782, 0.965517241],
        vec![0.143678161, 0.074712644, -0.206896552],
    ]).unwrap();
    compare_matrices(u.inverse().unwrap(), result);
        
}

#[test]
fn complex_inverse_tests() {
    let u = Matrix::from(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 5.0), Complex::new(6.0, 0.0)],
        vec![Complex::new(0.0, 1.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(6.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]).unwrap();
    let result = Matrix::from(vec![
        vec![Complex::new(-0.0333333, 0.0), Complex::new(0.0, 0.166667), Complex::new(0.2, 0.0)],
        vec![Complex::new(0.0, 0.0333333), Complex::new(1.166667, 0.0), Complex::new(0.0, -0.2)],
        vec![Complex::new(0.2, 0.0), Complex::new(0.0, -1.0), Complex::new(-0.2, 0.0)],
    ]).unwrap();
    compare_matrices(u.inverse().unwrap(), result);

    let u = Matrix::from(vec![
        vec![Complex::new(2.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(2.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(2.0, 0.0)],
    ]).unwrap();
    let result = Matrix::from(vec![
        vec![Complex::new(0.5, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(0.5, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.5, 0.0)],
    ]).unwrap();
    compare_matrices(u.inverse().unwrap(), result);
}