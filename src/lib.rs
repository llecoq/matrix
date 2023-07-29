/*
** ---------------------------------------------------------- MODULES
*/
mod vector;
mod matrix;
mod add_sub_scl;

/*
** ------------------------------------------------------------ TRAITS  should be moved ?
*/
// T: type of structure (Vector or Matrix)
// K: number type
pub trait AddSubScl<T, K> {
    fn add(&mut self, t: &T);
    fn sub(&mut self, t: &T);
    fn scl(&mut self, a: K);
}
