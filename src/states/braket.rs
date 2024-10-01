use super::StatesElement;

pub struct Braket<T, V> {
    pub ket: StatesElement<T, V>,
    pub bra: StatesElement<T, V>,
}

#[derive(PartialEq)]
pub struct StateBraket<T, V> {
    pub ket: (T, V),
    pub bra: (T, V),
}
