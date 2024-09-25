pub mod sum_states;
pub mod irreducible_states;

use std::{iter::Peekable, slice::Iter};

use irreducible_states::IrreducibleStates;

#[derive(Clone, Debug)]
pub enum StateType<T, V> {
    Irreducible(IrreducibleStates<T, V>),
    Sum(Vec<IrreducibleStates<T, V>>)
}

impl<T, V> StateType<T, V> {
    pub fn size(&self) -> usize {
        match self {
            StateType::Irreducible(irreducible_states) => irreducible_states.size(),
            StateType::Sum(sum_states) => sum_states.iter().fold(0, |acc, s| acc + s.size()),
        }
    }

    pub fn iter(&self) -> StateIter<'_, T, V> {
        match self {
            StateType::Irreducible(s) => StateIter {
                state_type: self,
                sum_iter: Iter::default().peekable(),
                irreducible_iter: s.elements.iter(),
            },
            StateType::Sum(vec_s) => StateIter {
                state_type: self,
                sum_iter: vec_s.iter().peekable(),
                irreducible_iter: vec_s.first()
                    .unwrap_or_else(|| panic!("no states to iter"))
                    .elements
                    .iter(),
            },
        }
    }
}

pub struct StateIter<'a, T, V> {
    state_type: &'a StateType<T, V>,
    sum_iter: Peekable<Iter<'a, IrreducibleStates<T, V>>>,
    irreducible_iter: Iter<'a, V>,
}

impl<'a, T, K> Iterator for StateIter<'a, T, K> {
    type Item = (&'a T, &'a K);

    fn next(&mut self) -> Option<Self::Item> {
        match self.state_type {
            StateType::Irreducible(s) => {
                self.irreducible_iter.next()
                    .map(|v| (&s.state_specific, v))
            },
            StateType::Sum(_) => {
                match self.irreducible_iter.next() {
                    Some(val) => Some((&self.sum_iter.peek().unwrap().state_specific, val)),
                    None => {
                        self.sum_iter.next().unwrap();
                        self.sum_iter.peek()
                            .map(|s| {
                                self.irreducible_iter = s.elements.iter();

                                self.irreducible_iter.next()
                                    .map(|v| (&s.state_specific, v))
                            })
                            .flatten()
                    }
                }
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct States<T, V>(Vec<StateType<T, V>>);

impl<T, V> Default for States<T, V> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<T, V> States<T, V> {
    pub fn size(&self) -> usize {
        self.0.iter().fold(1, |acc, s| acc * s.size())
    }

    pub fn iter(&self) -> StatesIter<'_, T, V> {
        StatesIter {
            states: &self.0,
            states_iter: self.0.iter().map(|s| s.iter()).collect(),
            current: StatesElement {
                states_specific: Vec::with_capacity(self.0.len()),
                values: Vec::with_capacity(self.0.len()),
            },
            current_index: 0,
            size: self.size(),
        }
    }
}

#[derive(Debug)]
pub struct StatesElement<'a, T, V> {
    pub states_specific: Vec<&'a T>,
    pub values: Vec<&'a V>,
}

impl<'a, T, V> Clone for StatesElement<'a, T, V> {
    fn clone(&self) -> Self {
        Self { 
            states_specific: self.states_specific.clone(),
             values: self.values.clone() 
        }
    }
}

pub struct StatesIter<'a, T, V> {
    states: &'a [StateType<T, V>],
    states_iter: Vec<StateIter<'a, T, V>>,
    current: StatesElement<'a, T, V>,
    current_index: usize,
    size: usize,
}

impl<'a, T: std::fmt::Debug, V: std::fmt::Debug> Iterator for StatesIter<'a, T, V> {
    type Item = StatesElement<'a, T, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.size {
            return None;
        }
        if self.current_index == 0 {
            for s in self.states_iter.iter_mut() {
                let (s_curr, v_curr) = s.next().unwrap(); // at least 1 element exists

                self.current.states_specific.push(s_curr);
                self.current.values.push(v_curr);
            }
            self.current_index += 1;

            return Some(self.current.clone());
        }

        for (((s_spec, v), s), s_type) in self.current.states_specific.iter_mut()
            .zip(self.current.values.iter_mut())
            .zip(self.states_iter.iter_mut())
            .zip(self.states.iter())
        {
            match s.next() {
                Some((s_spec_new, v_new)) => {
                    *s_spec = s_spec_new;
                    *v = v_new;
                    break
                },
                None => {
                    *s = s_type.iter();
                    let (s_curr, v_curr) = s.next().unwrap(); // at least 1 element exists and asserted size
                    *s_spec = s_curr;
                    *v = v_curr;
                },
            }
        }
        self.current_index += 1;

        Some(self.current.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(unused)]
    #[derive(Clone, Debug, PartialEq)]
    enum StateIds {
        ElectronSpin(u16),
        NuclearSpin(u16),
        Vibrational
    }

    #[allow(unused)]
    #[derive(Clone, Debug, PartialEq)]
    enum ElementValues {
        Spin(i16),
        Vibrational(i16)
    }

    #[test]
    fn states_creation_v1() {
        let mut states = States::default();

        let triplet_elements = vec![
            ElementValues::Spin(-2), 
            ElementValues::Spin(0), 
            ElementValues::Spin(2)
        ];
        let triplet = IrreducibleStates::new(StateIds::ElectronSpin(2), triplet_elements);
        let singlet = IrreducibleStates::new(StateIds::ElectronSpin(0), vec![ElementValues::Spin(0)]);

        let e_state = StateType::Sum(vec![triplet, singlet]);
        states.0.push(e_state);

        let nuclear_elements = vec![
            ElementValues::Spin(-1), 
            ElementValues::Spin(1)
        ];
        let nuclear = StateType::Irreducible(IrreducibleStates::new(StateIds::NuclearSpin(1), nuclear_elements));
        states.0.push(nuclear);


        let vib_elements = vec![
            ElementValues::Vibrational(-1),
            ElementValues::Vibrational(-2)
        ];
        let vib = StateType::Irreducible(IrreducibleStates::new(StateIds::Vibrational, vib_elements));
        states.0.push(vib);

        let expected = "States([Sum([IrreducibleStates { state_specific: ElectronSpin(2), elements: [Spin(-2), Spin(0), Spin(2)] }, \
                                    IrreducibleStates { state_specific: ElectronSpin(0), elements: [Spin(0)] }]), \
                                Irreducible(IrreducibleStates { state_specific: NuclearSpin(1), elements: [Spin(-1), Spin(1)] }), \
                                Irreducible(IrreducibleStates { state_specific: Vibrational, elements: [Vibrational(-1), Vibrational(-2)] })])";

        assert_eq!(expected, format!("{:?}", states))
    }

    #[test]
    fn states_creation_v2() {
        let mut states = States::default();

        let triplet = IrreducibleStates::new(StateIds::ElectronSpin(2), vec![-2, 0, 2]);
        let singlet = IrreducibleStates::new(StateIds::ElectronSpin(0), vec![0]);

        let e_state = StateType::Sum(vec![triplet, singlet]);
        states.0.push(e_state);

        let nuclear = StateType::Irreducible(IrreducibleStates::new(StateIds::NuclearSpin(1), vec![-1, 1]));
        states.0.push(nuclear);

        let vib = StateType::Irreducible(IrreducibleStates::new(StateIds::Vibrational, vec![-1, -2]));
        states.0.push(vib);

        let expected = "States([Sum([IrreducibleStates { state_specific: ElectronSpin(2), elements: [-2, 0, 2] }, \
                                    IrreducibleStates { state_specific: ElectronSpin(0), elements: [0] }]), \
                                Irreducible(IrreducibleStates { state_specific: NuclearSpin(1), elements: [-1, 1] }), \
                                Irreducible(IrreducibleStates { state_specific: Vibrational, elements: [-1, -2] })])";

        assert_eq!(expected, format!("{:?}", states))
    }

    #[test]
    fn state_type_iteration() {
        let triplet = IrreducibleStates::new(StateIds::ElectronSpin(2), vec![-2, 0, 2]);
        let singlet = IrreducibleStates::new(StateIds::ElectronSpin(0), vec![0]);

        let e_state = StateType::Sum(vec![triplet, singlet]);

        let mut e_iter = e_state.iter();
        assert_eq!(Some((&StateIds::ElectronSpin(2), &-2)), e_iter.next());
        assert_eq!(Some((&StateIds::ElectronSpin(2), &0)), e_iter.next());
        assert_eq!(Some((&StateIds::ElectronSpin(2), &2)), e_iter.next());
        assert_eq!(Some((&StateIds::ElectronSpin(0), &0)), e_iter.next());
        assert_eq!(None, e_iter.next());

        let nuclear = StateType::Irreducible(IrreducibleStates::new(StateIds::NuclearSpin(1), vec![-1, 1]));
        let mut n_iter = nuclear.iter();
        assert_eq!(Some((&StateIds::NuclearSpin(1), &-1)), n_iter.next());
        assert_eq!(Some((&StateIds::NuclearSpin(1), &1)), n_iter.next());
        assert_eq!(None, n_iter.next());
    }

    #[test]
    fn state_iteration() {
        let mut states = States::default();

        let triplet = IrreducibleStates::new(StateIds::ElectronSpin(2), vec![-2, 0, 2]);
        let singlet = IrreducibleStates::new(StateIds::ElectronSpin(0), vec![0]);

        let e_state = StateType::Sum(vec![triplet, singlet]);
        states.0.push(e_state);

        let nuclear = StateType::Irreducible(IrreducibleStates::new(StateIds::NuclearSpin(1), vec![-1, 1]));
        states.0.push(nuclear);

        let vib = StateType::Irreducible(IrreducibleStates::new(StateIds::Vibrational, vec![-1, -2]));
        states.0.push(vib);

        let expected: Vec<&str> = "\
StatesElement { states_specific: [ElectronSpin(2), NuclearSpin(1), Vibrational], values: [-2, -1, -1] }
StatesElement { states_specific: [ElectronSpin(2), NuclearSpin(1), Vibrational], values: [0, -1, -1] }
StatesElement { states_specific: [ElectronSpin(2), NuclearSpin(1), Vibrational], values: [2, -1, -1] }
StatesElement { states_specific: [ElectronSpin(0), NuclearSpin(1), Vibrational], values: [0, -1, -1] }
StatesElement { states_specific: [ElectronSpin(2), NuclearSpin(1), Vibrational], values: [-2, 1, -1] }
StatesElement { states_specific: [ElectronSpin(2), NuclearSpin(1), Vibrational], values: [0, 1, -1] }
StatesElement { states_specific: [ElectronSpin(2), NuclearSpin(1), Vibrational], values: [2, 1, -1] }
StatesElement { states_specific: [ElectronSpin(0), NuclearSpin(1), Vibrational], values: [0, 1, -1] }
StatesElement { states_specific: [ElectronSpin(2), NuclearSpin(1), Vibrational], values: [-2, -1, -2] }
StatesElement { states_specific: [ElectronSpin(2), NuclearSpin(1), Vibrational], values: [0, -1, -2] }
StatesElement { states_specific: [ElectronSpin(2), NuclearSpin(1), Vibrational], values: [2, -1, -2] }
StatesElement { states_specific: [ElectronSpin(0), NuclearSpin(1), Vibrational], values: [0, -1, -2] }
StatesElement { states_specific: [ElectronSpin(2), NuclearSpin(1), Vibrational], values: [-2, 1, -2] }
StatesElement { states_specific: [ElectronSpin(2), NuclearSpin(1), Vibrational], values: [0, 1, -2] }
StatesElement { states_specific: [ElectronSpin(2), NuclearSpin(1), Vibrational], values: [2, 1, -2] }
StatesElement { states_specific: [ElectronSpin(0), NuclearSpin(1), Vibrational], values: [0, 1, -2] }"
            .split("\n")
            .collect();

        for (state, exp) in states.iter().zip(expected.into_iter()) {
            assert_eq!(exp, format!("{:?}", state));
        }
    }
}