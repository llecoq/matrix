use num_complex::Complex;

use crate::{Matrix, Vector, tests::test_utils::assert_output};

#[test]
fn mul_vec_tests() {
    let u = Matrix::from(vec![
        vec![1., 0.],
        vec![0., 1.],
    ]).unwrap();
    let v = Vector::from(vec![4., 2.]);
    assert_output(&u.mul_vec(&v), "[4.0][2.0]");

    let u = Matrix::from(vec![
        vec![2., 0.],
        vec![0., 2.],
    ]).unwrap();
    let v = Vector::from(vec![4., 2.]);
    assert_output(&u.mul_vec(&v), "[8.0][4.0]");

    let u = Matrix::from(vec![
        vec![2., -2.],
        vec![-2., 2.],
    ]).unwrap();
    let v = Vector::from(vec![4., 2.]);
    assert_output(&u.mul_vec(&v), "[4.0][-4.0]");

    let u = Matrix::from(vec![
        vec![3., 0.],
        vec![0., 3.],
    ]).unwrap();
    let v = Vector::from(vec![3., 1.]);
    assert_output(&u.mul_vec(&v), "[9.0][3.0]");

    let u = Matrix::from(vec![
        vec![0., 4.],
        vec![4., 0.],
    ]).unwrap();
    let v = Vector::from(vec![2., 5.]);
    assert_output(&u.mul_vec(&v), "[20.0][8.0]");

    let u = Matrix::from(vec![
        vec![3., 2.],
        vec![2., 3.],
    ]).unwrap();
    let v = Vector::from(vec![3., 6.]);
    assert_output(&u.mul_vec(&v), "[21.0][24.0]");

    let u = Matrix::from(vec![
        vec![5., -1.],
        vec![-1., 5.],
    ]).unwrap();
    let v = Vector::from(vec![7., 3.]);
    assert_output(&u.mul_vec(&v), "[32.0][8.0]");

}

#[test]
fn mul_mat_tests() {
    let u = Matrix::from(vec![
        vec![1., 0.],
        vec![0., 1.],
    ]).unwrap();
    let v = Matrix::from(vec![
        vec![1., 0.],
        vec![0., 1.],
    ]).unwrap();
    assert_output(&u.mul_mat(&v), "[1.0][0.0]\n[0.0][1.0]");

    let u = Matrix::from(vec![
        vec![1., 0.],
        vec![0., 1.],
    ]).unwrap();
    let v = Matrix::from(vec![
        vec![2., 1.],
        vec![4., 2.],
    ]).unwrap();
    assert_output(&u.mul_mat(&v), "[2.0][1.0]\n[4.0][2.0]");

    let u = Matrix::from(vec![
        vec![3., -5.],
        vec![6., 8.],
    ]).unwrap();
    let v = Matrix::from(vec![
        vec![2., 1.],
        vec![4., 2.],
    ]).unwrap();
    assert_output(&u.mul_mat(&v), "[-14.0][-7.0]\n[44.0][22.0]");
        
    let u = Matrix::from(vec![
        vec![0.],
        vec![0.],
        vec![0.],
    ]).unwrap();
    let v = Matrix::from(vec![
        vec![2., 1., 5.],
        vec![4., 2., 3.],
    ]).unwrap();
    assert_output(&u.mul_mat(&v), "[0.0][0.0][0.0]\n[0.0][0.0][0.0]\n[0.0][0.0][0.0]");
    
    // should be empty because not matching row / column len
    let u = Matrix::from(vec![
        vec![3., -5., 0.],
        vec![6., 8., 6.],
    ]).unwrap();
    let v = Matrix::from(vec![
        vec![2., 1., 5.],
        vec![4., 2., 3.],
    ]).unwrap();
    assert_output(&u.mul_mat(&v), "[]");

    let u = Matrix::from(vec![
        vec![3., 0.],
        vec![0., 3.],
    ]).unwrap();
    let v = Vector::from(vec![3., 1.]);
    assert_output(&u.mul_vec(&v), "[9.0][3.0]");
            
    let u = Matrix::from(vec![
        vec![0., 4.],
        vec![4., 0.],
    ]).unwrap();
    let v = Vector::from(vec![2., 5.]);
    assert_output(&u.mul_vec(&v), "[20.0][8.0]");
    
    let u = Matrix::from(vec![
        vec![3., 2.],
        vec![2., 3.],
    ]).unwrap();
    let v = Vector::from(vec![3., 6.]);
    assert_output(&u.mul_vec(&v), "[15.0][24.0]");

    let u = Matrix::from(vec![
        vec![5., -1.],
        vec![-1., 5.],
    ]).unwrap();
    let v = Vector::from(vec![7., 3.]);
    assert_output(&u.mul_vec(&v), "[32.0][8.0]");

        
}
    
#[test]
fn complex_mul_vec_tests() {
    let u = Matrix::from(vec![
        vec![Complex::new(1., 1.), Complex::new(0., -1.)],
        vec![Complex::new(0., 1.), Complex::new(1., -1.)],
    ]).unwrap();
    let v = Vector::from(vec![Complex::new(4., 2.), Complex::new(3., -1.)]);
    assert_output(&u.mul_vec(&v), "[1.0+3.0i][0.0]");

    let u = Matrix::from(vec![
        vec![Complex::new(2., -2.), Complex::new(0., 2.)],
        vec![Complex::new(0., -2.), Complex::new(2., 2.)],
    ]).unwrap();
    let v = Vector::from(vec![Complex::new(4., 1.), Complex::new(3., 3.)]);
    assert_output(&u.mul_vec(&v), "[4.0+0.0i][2.0+4.0i]");

    let u = Matrix::from(vec![
        vec![Complex::new(2., 1.), Complex::new(-2., -1.)],
        vec![Complex::new(-2., 1.), Complex::new(2., -1.)],
    ]).unwrap();
    let v = Vector::from(vec![Complex::new(4., -2.), Complex::new(2., 4.)]);
    assert_output(&u.mul_vec(&v), "[10.0-10.0i][2.0+14.0i]");

}