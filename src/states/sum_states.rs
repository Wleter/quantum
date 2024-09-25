use super::irreducible_states::IrreducibleStates;

#[derive(Clone, Debug)]
pub struct SumStates<T, V>(pub Vec<IrreducibleStates<T, V>>);

impl<T, V> SumStates<T, V> {
    pub fn size(&self) -> usize {
        self.0.len()
    }
}