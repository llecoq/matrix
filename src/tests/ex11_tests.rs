use num_complex::Complex;

use crate::Matrix;

#[test]
fn f32_determinant_tests() {
    let u = Matrix::from(vec![
        vec![ 1., -1.],
        vec![-1., 1.],
    ]).unwrap();
    assert_eq!(u.determinant(), 0.0);
    // 0.0

    let u = Matrix::from(vec![
        vec![2., 0., 0.],
        vec![0., 2., 0.],
        vec![0., 0., 2.],
    ]).unwrap();
    assert_eq!(u.determinant(), 8.0);
    // 8.0

    let u = Matrix::from(vec![
        vec![8., 5., -2.],
        vec![4., 7., 20.],
        vec![7., 6., 1.],
    ]).unwrap();
    assert_eq!(u.determinant(), -174.0);
    // -174.0
 
    let u = Matrix::from(vec![
        vec![ 8., 5., -2., 4.],
        vec![ 4., 2.5, 20., 4.],
        vec![ 8., 5., 1., 4.],
        vec![28., -4., 17., 1.],
    ]).unwrap();
    assert_eq!(u.determinant(), 1032.0);
    // 1032
}

#[test]
fn complex_determinant_tests() {
    let u = Matrix::from(vec![
        vec![Complex::new(1.0, 1.0), Complex::new(-1.0, -1.0)],
        vec![Complex::new(-1.0, 1.0), Complex::new(1.0, -1.0)]
    ]).unwrap();
    assert_eq!(u.determinant(), Complex::new(0.0, 0.0));

    let u = Matrix::from(vec![
        vec![Complex::new(2.0, 1.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(2.0, -1.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(2.0, 0.5)]
    ]).unwrap();
    assert_eq!(u.determinant(), Complex::new(10.0, 2.5));

    let u = Matrix::from(vec![
        vec![Complex::new(8.0, -3.0), Complex::new(5.0, 2.0), Complex::new(-2.0, 1.5)],
        vec![Complex::new(4.0, 0.5), Complex::new(7.0, -0.5), Complex::new(20.0, 0.0)],
        vec![Complex::new(7.0, 0.0), Complex::new(6.0, 1.5), Complex::new(1.0, -2.0)]
    ]).unwrap();
    assert_eq!(u.determinant(), Complex::new(-352.75, 229.875));

    let u = Matrix::from(vec![
        vec![Complex::new(8.0, 0.5), Complex::new(5.0, -0.5), Complex::new(-2.0, 0.0), Complex::new(4.0, -1.0)],
        vec![Complex::new(4.0, 1.0), Complex::new(2.5, -0.5), Complex::new(20.0, 0.5), Complex::new(4.0, 0.0)],
        vec![Complex::new(8.0, -1.5), Complex::new(5.0, 0.0), Complex::new(1.0, -2.0), Complex::new(4.0, 1.5)],
        vec![Complex::new(28.0, 0.0), Complex::new(-4.0, -2.0), Complex::new(17.0, 0.0), Complex::new(1.0, -1.0)]
    ]).unwrap();
    assert_eq!(u.determinant(), Complex::new(1453.0, -8676.75));
}