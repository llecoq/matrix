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
    // println!("{}, {}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    // let u = Vector::from([1., 2., 3.]);
    // println!("{}, {}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 6.0, 3.74165738, 3.0
    // let u = Vector::from([-1., -2.]);
    // println!("{}, {}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
}

//-------------------------------------------------------------------- norm
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