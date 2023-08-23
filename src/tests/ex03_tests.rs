use num_complex::Complex;

use crate::Vector;

#[test]
fn f32_dot_product_tests() {
    //---------------------------------------------------------------------------------f32
    let u = Vector::from(vec![0., 0.]);
    let v = Vector::from(vec![1., 1.]);
    assert_eq!(u.dot(&v), 0.);

    let u = Vector::from(vec![1., 1.]);
    let v = Vector::from(vec![1., 1.]);
    assert_eq!(u.dot(&v), 2.);

    let u = Vector::from(vec![-1., 6.]);
    let v = Vector::from(vec![3., 2.]);
    assert_eq!(u.dot(&v), 9.);

    let u = Vector::from(vec![-1., 6., 3., 9., 0., -9.]);
    let v = Vector::from(vec![3., 2., 6., 3., -6., -1.]);
    assert_eq!(u.dot(&v), 63.);

    let u = Vector::from(vec![3., 9., 3., -9.]);
    let v = Vector::from(vec![6., 3., -6., -1.]);
    assert_eq!(u.dot(&v), 36.);

    let u = Vector::from(vec![0., 0.]);
    let v = Vector::from(vec![0., 0.]);
    assert_eq!(u.dot(&v), 0.);

    let u: Vector<f32> = Vector::from(vec![]);
    let v = Vector::from(vec![]);
    assert_eq!(u.dot(&v), 0.);

    // undefined
    let u: Vector<f32> = Vector::from(vec![1.]);
    let v = Vector::from(vec![]);
    assert_eq!(u.dot(&v), 0.);

    let u: Vector<f32> = Vector::from(vec![]);
    let v = Vector::from(vec![1.]);
    assert_eq!(u.dot(&v), 0.);
    
}

#[test]
fn complex_dot_product_tests() {
    //---------------------------------------------------------------------------Complex<f32>
    let u = Vector::from(vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)]);
    let v = Vector::from(vec![Complex::new(1.0, 1.0), Complex::new(1.0, 1.0)]);
    assert_eq!(u.dot(&v), Complex::new(0.0, 0.0));

    let u = Vector::from(vec![Complex::new(1.0, 1.0), Complex::new(1.0, 1.0)]);
    let v = Vector::from(vec![Complex::new(1.0, 1.0), Complex::new(1.0, 1.0)]);
    assert_eq!(u.dot(&v), Complex::new(0.0, 4.0));

    let u = Vector::from(vec![Complex::new(-1.0, 6.0)]);
    let v = Vector::from(vec![Complex::new(3.0, 2.0)]);
    assert_eq!(u.dot(&v), Complex::new(-15.0, 16.0));

    let u = Vector::from(vec![Complex::new(-1.0, 6.0), Complex::new(3.0, 9.0), Complex::new(0.0, 0.0), Complex::new(-9.0, -1.0)]);
    let v = Vector::from(vec![Complex::new(3.0, 2.0), Complex::new(6.0, 3.0), Complex::new(-6.0, -6.0), Complex::new(-1.0, 0.0)]);
    assert_eq!(u.dot(&v), Complex::new(-15.0, 80.0));

    let u = Vector::from(vec![Complex::new(3.0, 9.0), Complex::new(3.0, -9.0)]);
    let v = Vector::from(vec![Complex::new(6.0, 3.0), Complex::new(-6.0, -1.0)]);
    assert_eq!(u.dot(&v), Complex::new(-36.0, 114.0));

    let u = Vector::from(vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)]);
    let v = Vector::from(vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)]);
    assert_eq!(u.dot(&v), Complex::new(0.0, 0.0));

    let u: Vector<Complex<f32>> = Vector::from(vec![]);
    let v = Vector::from(vec![]);
    assert_eq!(u.dot(&v), Complex::new(0.0, 0.0));
    // undefined
    let u: Vector<Complex<f32>> = Vector::from(vec![Complex::new(1.0, 0.0)]);
    let v = Vector::from(vec![]);
    assert_eq!(u.dot(&v), Complex::new(0.0, 0.0));

    let u: Vector<Complex<f32>> = Vector::from(vec![]);
    let v = Vector::from(vec![Complex::new(1.0, 0.0)]);
    assert_eq!(u.dot(&v), Complex::new(0.0, 0.0));

}