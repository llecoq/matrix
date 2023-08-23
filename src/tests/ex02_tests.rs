use num_complex::Complex;

use crate::{Vector, Matrix, tests::test_utils::assert_output, lerp};

use super::test_utils::matrix_to_string;

#[test]
fn linear_interpolation_tests() {
    //----------------------------------------------------------------------------------f32
    let v1: Vector<f32> = Vector::from(vec![0.]);
    let v2: Vector<f32> = Vector::from(vec![1.]);
    assert_output(&lerp(&v1, &v2, 0.), "[0.0]");

    let v1: Vector<f32> = Vector::from(vec![0.]);
    let v2: Vector<f32> = Vector::from(vec![1.]);
    assert_output(&lerp(&v1, &v2, 1.), "[1.0]");

    let v1: Vector<f32> = Vector::from(vec![0.]);
    let v2: Vector<f32> = Vector::from(vec![1.]);
    assert_output(&lerp(&v1, &v2, 0.5), "[0.5]");

    let v1: Vector<f32> = Vector::from(vec![21.]);
    let v2: Vector<f32> = Vector::from(vec![42.]);
    assert_output(&lerp(&v1, &v2, 0.3), "[27.3]");

    let v1: Vector<f32> = Vector::from(vec![2., 1.]);
    let v2: Vector<f32> = Vector::from(vec![4., 2.]);
    assert_output(&lerp(&v1, &v2, 0.3), "[2.6][1.3]");

    let input = vec![
        vec![2., 1.],
        vec![3., 4.]
    ];
    let matrix = Matrix::from(input.clone()).unwrap();
    let input_2 = vec![
        vec![20., 10.],
        vec![30., 40.]
    ];
    let matrix_2 = Matrix::from(input_2.clone()).unwrap();
    let result = vec![
        vec![11., 5.5],
        vec![16.5, 22.]
    ];
    assert_output(&lerp(&matrix, &matrix_2, 0.5), &matrix_to_string(&result));

    // udefined result because of different sizes
    let input = vec![
        vec![2., 1., 6.3],
        vec![3., 4., 9.6]
    ];
    let matrix = Matrix::from(input.clone()).unwrap();
    let input_2 = vec![
        vec![20., 10.],
        vec![30., 40.]
    ];
    let matrix_2 = Matrix::from(input_2.clone()).unwrap();
    let result = vec![
        vec![11., 5.5],
        vec![16.5, 22.]
    ];
    assert_output(&lerp(&matrix_2, &matrix, 0.5), &matrix_to_string(&result));    

    println!("{}", matrix);

}

#[test]
fn complex_linear_interpolation_tests() {
    //---------------------------------------------------------------------------Complex<f32>
    let v1: Vector<Complex<f32>> = Vector::from(vec![Complex::new(0., 0.5)]);
    let v2: Vector<Complex<f32>> = Vector::from(vec![Complex::new(1., -0.5)]);
    assert_output(&lerp(&v1, &v2, 0.), "[0.0+0.5i]");

    let v1: Vector<Complex<f32>> = Vector::from(vec![Complex::new(0., 0.2)]);
    let v2: Vector<Complex<f32>> = Vector::from(vec![Complex::new(1., -0.2)]);
    assert_output(&lerp(&v1, &v2, 1.), "[1.0-0.2i]");

    let v1: Vector<Complex<f32>> = Vector::from(vec![Complex::new(0., 0.7)]);
    let v2: Vector<Complex<f32>> = Vector::from(vec![Complex::new(1., -0.7)]);
    assert_output(&lerp(&v1, &v2, 0.5), "[0.5+0.0i]");

    let v1: Vector<Complex<f32>> = Vector::from(vec![Complex::new(21., 1.5)]);
    let v2: Vector<Complex<f32>> = Vector::from(vec![Complex::new(42., -1.5)]);
    assert_output(&lerp(&v1, &v2, 0.3), "[27.3+0.6i]");

    let v1: Vector<Complex<f32>> = Vector::from(vec![Complex::new(2., 0.9), Complex::new(1., -0.8)]);
    let v2: Vector<Complex<f32>> = Vector::from(vec![Complex::new(4., -0.9), Complex::new(2., 0.8)]);
    assert_output(&lerp(&v1, &v2, 0.3), "[2.6+0.4i][1.3-0.3i]");

    let input = vec![
        vec![Complex::new(2., 0.4), Complex::new(1., -0.4)],
        vec![Complex::new(3., -0.5), Complex::new(4., 0.5)]
    ];
    let matrix = Matrix::from(input.clone()).unwrap();
    let input_2 = vec![
        vec![Complex::new(20., -1.0), Complex::new(10., 1.0)],
        vec![Complex::new(30., 0.6), Complex::new(40., -0.6)]
    ];
    let matrix_2 = Matrix::from(input_2.clone()).unwrap();
    let result = vec![
        vec![Complex::new(11., -0.3), Complex::new(5.5, 0.3)],
        vec![Complex::new(16.5, 0.05), Complex::new(22., -0.05)]
    ];
    assert_output(&lerp(&matrix, &matrix_2, 0.5), &matrix_to_string(&result));

    // undefined result because of different sizes
    let input = vec![
        vec![Complex::new(2., 0.3), Complex::new(1., -0.3), Complex::new(6.3, 0.6)],
        vec![Complex::new(3., -0.4), Complex::new(4., 0.4), Complex::new(9.6, -0.6)]
    ];
    let matrix = Matrix::from(input.clone()).unwrap();
    let input_2 = vec![
        vec![Complex::new(20., -0.5), Complex::new(10., 0.5)],
        vec![Complex::new(30., 0.7), Complex::new(40., -0.7)]
    ];
    let matrix_2 = Matrix::from(input_2.clone()).unwrap();
    let result = vec![
        vec![Complex::new(11., -0.1), Complex::new(5.5, 0.1)],
        vec![Complex::new(16.5, 0.15), Complex::new(22., -0.15)]
    ];
        assert_output(&lerp(&matrix_2, &matrix, 0.5), &matrix_to_string(&result));
}
