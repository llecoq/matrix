use crate::{Matrix, Vector, tests::test_utils::assert_output};

#[test]
fn mul_vec_tests() {
    let u = Matrix::from(vec![
        vec![1., 0.],
        vec![0., 1.],
    ]).unwrap();
    let v = Vector::from(vec![4., 2.]);
    assert_output(&u.mul_vec(&v), "[4.0][2.0]");
    // [4.]
    // [2.]
    let u = Matrix::from(vec![
        vec![2., 0.],
        vec![0., 2.],
    ]).unwrap();
    let v = Vector::from(vec![4., 2.]);
    assert_output(&u.mul_vec(&v), "[8.0][4.0]");
    // [8.]
    // [4.]
    let u = Matrix::from(vec![
        vec![2., -2.],
        vec![-2., 2.],
    ]).unwrap();
    let v = Vector::from(vec![4., 2.]);
    assert_output(&u.mul_vec(&v), "[4.0][-4.0]");
    // [4.]
    // [-4.]
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
    // [1., 0.]
    // [0., 1.]
    let u = Matrix::from(vec![
        vec![1., 0.],
        vec![0., 1.],
    ]).unwrap();
    let v = Matrix::from(vec![
        vec![2., 1.],
        vec![4., 2.],
    ]).unwrap();
    assert_output(&u.mul_mat(&v), "[2.0][1.0]\n[4.0][2.0]");
    // [2., 1.]
    // [4., 2.]
    let u = Matrix::from(vec![
        vec![3., -5.],
        vec![6., 8.],
    ]).unwrap();
    let v = Matrix::from(vec![
        vec![2., 1.],
        vec![4., 2.],
    ]).unwrap();
    assert_output(&u.mul_mat(&v), "[-14.0][-7.0]\n[44.0][22.0]");
    // [-14., -7.]
    // [44., 22.]
}