use crate::Matrix;
use crate::Vector;
use crate::AddSubScl;

/*
** ------------------------------------------------------------ VECTOR
*/
impl<T, K> AddSubScl<T, K> for Vector<K> {
    fn add(&mut self, v: &Vector<K>) {
        
    }
    fn sub(&mut self, v: &Vector<K>) {

    }
    fn scl(&mut self, a: K) {

    }
}

/*
** ------------------------------------------------------------ MATRIX
*/
impl<T, K> AddSubScl<T, K> for Matrix<K> {
    fn add(&mut self, v: &Matrix<K>){

    }
    fn sub(&mut self, v: &Matrix<K>){

    }
    fn scl(&mut self, a: K){

    }
}