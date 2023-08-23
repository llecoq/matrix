use num_complex::Complex;

use crate::Matrix;

#[test]
fn f32_rank_tests() {
    let u = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]).unwrap();
    assert_eq!(u.rank(), 3);

    let u = Matrix::from(vec![
        vec![ 1., 2., 0., 0.],
        vec![ 2., 4., 0., 0.],
        vec![-1., 2., 1., 1.],
    ]).unwrap();
    assert_eq!(u.rank(), 2);

    let u = Matrix::from(vec![
        vec![ 8., 5., -2.],
        vec![ 4., 7., 20.],
        vec![ 7., 6., 1.],
        vec![21., 18., 7.],
    ]).unwrap();
    assert_eq!(u.rank(), 3);
    
    let u = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]).unwrap();
    assert_eq!(u.rank(), 3);

    let u = Matrix::from(vec![
        vec![ 1., 2., 0., 0.],
        vec![ 2., 4., 0., 0.],
        vec![-1., 2., 1., 1.],
    ]).unwrap();
    assert_eq!(u.rank(), 2);

    let u = Matrix::from(vec![
        vec![ 8., 5., -2.],
        vec![ 4., 7., 20.],
        vec![ 7., 6., 1.],
        vec![21., 18., 7.],
    ]).unwrap();
    assert_eq!(u.rank(), 3);
}

#[test]
fn complex_rank_tests() {
    let u = Matrix::from(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]).unwrap();
    assert_eq!(u.rank(), 3);

    let u = Matrix::from(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(2.0, 0.0), Complex::new(4.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(-1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)],
    ]).unwrap();
    assert_eq!(u.rank(), 2);

    let u = Matrix::from(vec![
        vec![Complex::new(8.0, 0.0), Complex::new(5.0, 0.0), Complex::new(-2.0, 0.0)],
        vec![Complex::new(4.0, 0.0), Complex::new(7.0, 0.0), Complex::new(20.0, 0.0)],
        vec![Complex::new(7.0, 0.0), Complex::new(6.0, 0.0), Complex::new(1.0, 0.0)],
        vec![Complex::new(21.0, 0.0), Complex::new(18.0, 0.0), Complex::new(7.0, 0.0)],
    ]).unwrap();
    assert_eq!(u.rank(), 3);
}