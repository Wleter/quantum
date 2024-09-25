
#[derive(Clone, Debug)]
pub struct IrreducibleStates<T, V> {
    pub(crate) state_specific: T,
    pub(crate) elements: Vec<V>
}

impl<T, V> IrreducibleStates<T, V> {
    pub fn new(state_specific: T, elements: Vec<V>) -> Self {
        assert!(elements.len() > 0, "0 dimensional state");

        Self {
            state_specific,
            elements
        }
    }
    
    pub fn size(&self) -> usize {
        self.elements.len()
    }
}