use crate::Matrix;

use super::test_utils::{assert_output, matrix_to_string};

#[test]
fn transpose_tests() {
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

}