
#[derive(Clone, Debug)]
pub struct IrreducibleStates<T, V> {
    pub(crate) state_specific: T,
    pub(crate) basis: Vec<V>
}

impl<T: Copy, V: Copy> IrreducibleStates<T, V> {
    pub fn new(state_specific: T, basis: Vec<V>) -> Self {
        assert!(basis.len() > 0, "0 size basis is not allowed");

        Self {
            state_specific,
            basis
        }
    }
}

impl<T, V> IrreducibleStates<T, V> {
    pub fn size(&self) -> usize {
        self.basis.len()
    }
}