use num_complex::Complex;

use crate::Vector;

//-------------------------------------------------------------------- norm_1
#[test]
fn norm_1_tests() {

    let mut vec: Vector<f32> = Vector::from(vec![0., 0., 0.]);
    assert_eq!(vec.norm_1(), 0.0);

    let mut vec: Vector<f32> = Vector::from(vec![1., 2., 3.]);
    assert_eq!(vec.norm_1(), 6.0);
    
    let mut vec: Vector<f32> = Vector::from(vec![-1., -2.]);
    assert_eq!(vec.norm_1(), 3.0);

    let mut vec: Vector<f32> = Vector::from(vec![-1., -2., 3., 4., -6.3, 5.2, -3.]);
    assert_eq!(vec.norm_1(), 24.5);

    let mut vec: Vector<f32> = Vector::from(vec![]);
    assert_eq!(vec.norm_1(), 0.0);

    let mut vec: Vector<f32> = Vector::from(vec![1.]);
    assert_eq!(vec.norm_1(), 1.0);
    
    let mut vec: Vector<f32> = Vector::from(vec![-1.]);
    assert_eq!(vec.norm_1(), 1.0); 
}

//---------------------------------------------------------------------- norm
#[test]
fn norm_tests() {

    let mut vec: Vector<f32> = Vector::from(vec![0., 0., 0.]);
    assert_eq!(vec.norm(), 0.0);

    let mut vec: Vector<f32> = Vector::from(vec![1., 2., 3.]);
    assert_eq!(vec.norm(), 3.74165738);
    
    let mut vec: Vector<f32> = Vector::from(vec![-1., -2.]);
    assert_eq!(vec.norm(), 2.236067977);

    let mut vec: Vector<f32> = Vector::from(vec![-1., -2., 3., 4., -6.3, 5.2, -3.]);
    assert_eq!(vec.norm(), 10.282509);

    let mut vec: Vector<f32> = Vector::from(vec![]);
    assert_eq!(vec.norm(), 0.0);

    let mut vec: Vector<f32> = Vector::from(vec![1.]);
    assert_eq!(vec.norm(), 1.0);
    
    let mut vec: Vector<f32> = Vector::from(vec![-1.]);
    assert_eq!(vec.norm(), 1.0); 
}

//------------------------------------------------------------------- norm_inf
#[test]
fn norm_inf_tests() {
    let mut vec: Vector<f32> = Vector::from(vec![0., 0., 0.]);
    assert_eq!(vec.norm_inf(), 0.0);

    let mut vec: Vector<f32> = Vector::from(vec![1., 2., 3.]);
    assert_eq!(vec.norm_inf(), 3.);
    
    let mut vec: Vector<f32> = Vector::from(vec![-1., -2.]);
    assert_eq!(vec.norm_inf(), 2.);

    let mut vec: Vector<f32> = Vector::from(vec![-1., -2., 3., 4., -6.3, 5.2, -3.]);
    assert_eq!(vec.norm_inf(), 6.3);

    let mut vec: Vector<f32> = Vector::from(vec![]);
    assert_eq!(vec.norm_inf(), 0.0);

    let mut vec: Vector<f32> = Vector::from(vec![1.]);
    assert_eq!(vec.norm_inf(), 1.0);
    
    let mut vec: Vector<f32> = Vector::from(vec![-1.]);
    assert_eq!(vec.norm_inf(), 1.0); 
}

//---------------------------------------------------------------------- norm_1
#[test]
fn norm_1_complex_tests() {
    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(0., 0.)]);
    assert_eq!(vec.norm_1(), 0.0);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(1., 1.), Complex::new(2., 2.), Complex::new(3., 3.)]);
    assert_eq!(vec.norm_1(), 8.485281);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(-1., -1.), Complex::new(-2., -2.)]);
    assert_eq!(vec.norm_1(), 4.2426405);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(-1., -1.), Complex::new(-2., -2.), Complex::new(3., 3.), Complex::new(4., 4.), Complex::new(-6.3, -6.3), Complex::new(5.2, 5.2), Complex::new(-3., -3.)]);
    assert_eq!(vec.norm_1(), 34.64823);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![]);
    assert_eq!(vec.norm_1(), 0.0);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(1., 1.)]);
    assert_eq!(vec.norm_1(), 1.4142135);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(-1., -1.)]);
    assert_eq!(vec.norm_1(), 1.4142135);
}

//---------------------------------------------------------------------- norm
#[test]
fn norm_complex_tests() {
    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(0., 0.)]);
    assert_eq!(vec.norm(), 0.0);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(1., 1.), Complex::new(2., 2.), Complex::new(3., 3.)]);
    assert_eq!(vec.norm(), 5.2915025);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(-1., -1.), Complex::new(-2., -2.)]);
    assert_eq!(vec.norm(), 3.1622775);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(-1., -1.), Complex::new(-2., -2.), Complex::new(3., 3.), Complex::new(4., 4.), Complex::new(-6.3, -6.3), Complex::new(5.2, 5.2), Complex::new(-3., -3.)]);
    assert_eq!(vec.norm(), 14.541664);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![]);
    assert_eq!(vec.norm(), 0.0);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(1., 1.)]);
    assert_eq!(vec.norm(), (2.0 as f32).sqrt());

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(-1., -1.)]);
    assert_eq!(vec.norm(), (2.0 as f32).sqrt());
}

//------------------------------------------------------------------- norm_inf
#[test]
fn norm_inf_complex_tests() {
    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(0., 0.)]);
    assert_eq!(vec.norm_inf(), 0.0);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(1., 1.), Complex::new(2., 2.), Complex::new(3., 3.)]);
    assert_eq!(vec.norm_inf(), 4.2426407);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(-1., -1.), Complex::new(-2., -2.)]);
    assert_eq!(vec.norm_inf(), 2.828427);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(-1., -1.), Complex::new(-2., -2.), Complex::new(3., 3.), Complex::new(4., 4.), Complex::new(-6.3, -6.3), Complex::new(5.2, 5.2), Complex::new(-3., -3.)]);
    assert_eq!(vec.norm_inf(), 8.909546);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![]);
    assert_eq!(vec.norm_inf(), 0.0);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(1., 1.)]);
    assert_eq!(vec.norm_inf(), 1.4142135);

    let mut vec: Vector<Complex<f32>> = Vector::from(vec![Complex::new(-1., -1.)]);
    assert_eq!(vec.norm_inf(), 1.4142135);
}