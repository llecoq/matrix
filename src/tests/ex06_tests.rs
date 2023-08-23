use num_complex::Complex;

use crate::{Vector, cross_product, tests::test_utils::assert_output};

#[test]
fn cross_product_tests() {

    let u = Vector::from(vec![0., 0., 1.]);
    let v = Vector::from(vec![1., 0., 0.]);
    assert_output(&cross_product(&u, &v), "[0.0][1.0][0.0]");

    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    assert_output(&cross_product(&u, &v), "[-3.0][6.0][-3.0]");

    let u = Vector::from(vec![4., 2., -3.]);
    let v = Vector::from(vec![-2., -5., 16.]);
    assert_output(&cross_product(&u, &v), "[17.0][-58.0][-16.0]");

    let u = Vector::from(vec![2., 3., 1.]);
    let v = Vector::from(vec![4., 1., 3.]);
    assert_output(&cross_product(&u, &v), "[8.0][-2.0][-10.0]");

    let u = Vector::from(vec![6., 7., 5.]);
    let v = Vector::from(vec![3., 2., 1.]);
    assert_output(&cross_product(&u, &v), "[-3.0][9.0][-9.0]");
    
    let u = Vector::from(vec![9., 2., 6.]);
    let v = Vector::from(vec![1., 8., 3.]);
    assert_output(&cross_product(&u, &v), "[-42.0][-21.0][70.0]");
    
    let u = Vector::from(vec![3., 5., 7.]);
    let v = Vector::from(vec![2., 4., 6.]);
    assert_output(&cross_product(&u, &v), "[2.0][-4.0][2.0]");
    
    let u = Vector::from(vec![10., 20., 30.]);
    let v = Vector::from(vec![5., 3., 2.]);
    assert_output(&cross_product(&u, &v), "[-50.0][130.0][-70.0]");
    
    let u = Vector::from(vec![12., 24., 36.]);
    let v = Vector::from(vec![6., 5., 4.]);
    assert_output(&cross_product(&u, &v), "[-84.0][168.0][-84.0]");

}

#[test]
fn complex_cross_product_tests() {

    let u = Vector::from(vec![Complex::new(0., 1.), Complex::new(0., 2.), Complex::new(1., 3.)]);
    let v = Vector::from(vec![Complex::new(1., 2.), Complex::new(0., 3.), Complex::new(0., 4.)]);
    assert_output(&cross_product(&u, &v), "[1.0-3.0i][-1.0+5.0i][1.0-2.0i]");

    let u = Vector::from(vec![Complex::new(1., 2.), Complex::new(2., 3.), Complex::new(3., 4.)]);
    let v = Vector::from(vec![Complex::new(4., 5.), Complex::new(5., 6.), Complex::new(6., 7.)]);
    assert_output(&cross_product(&u, &v), "[0.0-6.0i][0.0+12.0i][0.0-6.0i]");

    let u = Vector::from(vec![Complex::new(4., 5.), Complex::new(2., 6.), Complex::new(-3., 7.)]);
    let v = Vector::from(vec![Complex::new(-2., 8.), Complex::new(-5., 9.), Complex::new(16., 10.)]);
    assert_output(&cross_product(&u, &v), "[20.0+178.0i][-64.0-158.0i][-13.0+7.0i]");

    let u = Vector::from(vec![Complex::new(2., 7.), Complex::new(3., 8.), Complex::new(1., 9.)]);
    let v = Vector::from(vec![Complex::new(4., 10.), Complex::new(1., 11.), Complex::new(3., 12.)]);
    assert_output(&cross_product(&u, &v), "[11.0+40.0i][-8.0+1.0i][-7.0-33.0i]");

    let u = Vector::from(vec![Complex::new(6., 11.), Complex::new(7., 12.), Complex::new(5., 13.)]);
    let v = Vector::from(vec![Complex::new(3., 14.), Complex::new(2., 15.), Complex::new(1., 16.)]);
    assert_output(&cross_product(&u, &v), "[0.0+23.0i][3.0+2.0i][-6.0-22.0i]");

    let u = Vector::from(vec![Complex::new(9., 13.), Complex::new(2., 14.), Complex::new(6., 15.)]);
    let v = Vector::from(vec![Complex::new(1., 16.), Complex::new(8., 17.), Complex::new(3., 18.)]);
    assert_output(&cross_product(&u, &v), "[-39.0-144.0i][-27.0-90.0i][73.0+211.0i]");

    let u = Vector::from(vec![Complex::new(3., 15.), Complex::new(5., 16.), Complex::new(7., 17.)]);
    let v = Vector::from(vec![Complex::new(2., 18.), Complex::new(4., 19.), Complex::new(6., 20.)]);
    assert_output(&cross_product(&u, &v), "[5.0-5.0i][-10.0+10.0i][5.0-5.0i]");

    let u = Vector::from(vec![Complex::new(10., 17.), Complex::new(20., 18.), Complex::new(30., 19.)]);
    let v = Vector::from(vec![Complex::new(5., 20.), Complex::new(3., 21.), Complex::new(2., 22.)]);
    assert_output(&cross_product(&u, &v), "[-47.0-211.0i][124.0+441.0i][-67.0-229.0i]");

    let u = Vector::from(vec![Complex::new(12., 19.), Complex::new(24., 20.), Complex::new(36., 21.)]);
    let v = Vector::from(vec![Complex::new(6., 22.), Complex::new(5., 23.), Complex::new(4., 24.)]);
    assert_output(&cross_product(&u, &v), "[-81.0-277.0i][162.0+554.0i][-81.0-277.0i]");
    
}