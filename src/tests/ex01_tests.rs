use num_complex::Complex;

use crate::{Vector, tests::test_utils::assert_output, linear_combination};

#[test]
fn linear_combination_tests() {
    //------------------------------------------------------------------------------------f32
    let e1 = Vector::from(vec![1., 0., 0.]);
    let e2 = Vector::from(vec![0., 1., 0.]);
    let e3 = Vector::from(vec![0., 0., 1.]);
    let v1 = Vector::from(vec![1., 2., 3.]);
    let v2 = Vector::from(vec![0., 10., -100.]);

    assert_output(&linear_combination(&[e1, e2, e3], &[10., -2., 0.5]), "[10.0][-2.0][0.5]");
    assert_output(&linear_combination(&[v1, v2], &[10., -2.]), "[10.0][0.0][230.0]");

    let e1 = Vector::from(vec![-1., 0., 0.]);
    let e2 = Vector::from(vec![0., 105., 5.]);
    let e3 = Vector::from(vec![0.6, 0., -1.]);
    let v1 = Vector::from(vec![1., 2., 3.]);
    let v2 = Vector::from(vec![2., 15., -700.]);

    assert_output(&linear_combination(&[e1, e2, e3], &[16., -2.6, 0.5]), "[-15.7][-273.0][-13.5]");
    assert_output(&linear_combination(&[v1, v2], &[-10., -2.]), "[-14.0][-50.0][1370.0]");

    let e1 = Vector::from(vec![-1., 0., 0.]);
    let e2 = Vector::from(vec![0., 5.]);
    let e3 = Vector::from(vec![0.6, 0., -1.]);
    let v1 = Vector::from(vec![1., 2., 3.]);
    let v2 = Vector::from(vec![2., 15., -700.]);

    // undefined
    println!("{}", linear_combination(&[e1, e2, e3], &[16., 0.5]));
    println!("{}", linear_combination(&[v1, v2], &[1., 0., 7., 9.]));

    // empty
    let e1: Vector<f32> = Vector::from(vec![]);
    let e2 = Vector::from(vec![0., 5.]);
    let e3 = Vector::from(vec![0.6, 0., -1.]); 
    println!("{}", linear_combination(&[e1, e2, e3], &[16.]));

    //---------------------------------------------------------------------------Complex<f32>
    let e1 = Vector::from(vec![Complex::new(1., 0.5), Complex::new(0., -0.2), Complex::new(0., 1.3)]);
    let e2 = Vector::from(vec![Complex::new(0., 0.7), Complex::new(1., -0.8), Complex::new(0., 0.4)]);
    let e3 = Vector::from(vec![Complex::new(0., -1.1), Complex::new(0., 0.5), Complex::new(1., 0.9)]);
    let v1 = Vector::from(vec![Complex::new(1., -0.5), Complex::new(2., 0.2), Complex::new(3., -1.0)]);
    let v2 = Vector::from(vec![Complex::new(0., 0.3), Complex::new(10., -0.9), Complex::new(-100., 2.3)]);
    
    assert_output(&linear_combination(&[e1.clone(), e2.clone(), e3.clone()], &[Complex::new(10., -0.5), Complex::new(-2., 0.3), Complex::new(0.5, 1.2)]), "[11.4+2.5i][-2.5+0.2i][-0.1+13.9i]");
    assert_output(&linear_combination(&[v1.clone(), v2.clone()], &[Complex::new(10., 1.0), Complex::new(-2., -1.5)]), "[10.9-4.6i][-1.6-9.2i][234.4+138.4i]");
    
    println!("{}", linear_combination(&[e1, e2, e3], &[Complex::new(16., 0.8), Complex::new(0.5, -0.4)]));
    println!("{}", linear_combination(&[v1, v2], &[Complex::new(1., 0.3), Complex::new(0., -1.0), Complex::new(7., 2.0), Complex::new(9., -1.5)]));
    
    let e1: Vector<Complex<f32>> = Vector::from(vec![]);
    let e2 = Vector::from(vec![Complex::new(0., -0.6), Complex::new(5., 1.4)]);
    let e3 = Vector::from(vec![Complex::new(0.6, 0.3), Complex::new(0., -0.7), Complex::new(-1., 0.5)]);
    println!("{}", linear_combination(&[e1, e2, e3], &[Complex::new(16., -0.9)]));

}
