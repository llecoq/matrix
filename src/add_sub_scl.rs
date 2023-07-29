use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::AddSubScl;

/*
** ------------------------------------------------------------ VECTOR
*/
impl<K> AddSubScl<Vector<K>, K> for Vector<K> {
    fn add(&mut self, v: &Vector<K>) {
        
    }
    fn sub(&mut self, v: &Vector<K>) {

    }
    fn scl(&mut self, a: K) {

    }
}

/*
** ------------------------------------------------------------ VECTOR
*/
impl<K> AddSubScl<Matrix<K>, K> for Matrix<K> {
    fn add(&mut self, v: &Matrix<K>){

    }
    fn sub(&mut self, v: &Matrix<K>){

    }
    fn scl(&mut self, a: K){

    }
}