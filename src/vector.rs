use crate::traits::AddSubScl;

/// Vector structure
pub struct Vector<K> {
    elements:   Vec<K>
}

//-------------------- Utility functions
impl<K> Vector<K> {
    // fn get_size {}
    // print_vector {}
    // reshape_into_matrix {}
}

impl<K> AddSubScl<Vector<K>, K> for Vector<K> {
    fn add(&mut self, v: &Vector<K>) {
        
    }
    fn sub(&mut self, v: &Vector<K>) {

    }
    fn scl(&mut self, a: K) {

    }
}