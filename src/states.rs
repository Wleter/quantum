pub mod irreducible_states;
pub mod state_type;
pub mod braket;

use std::ops::Deref;

use state_type::{StateTypeIter, StateType};

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

    pub fn push_state(&mut self, state: StateType<T, V>) -> &mut Self {
        let variant = state.discriminant();
        if let Some(variant) = variant {
            if self.0.iter().any(|x| x.discriminant() == Some(variant)) {
                panic!("Each state has to have unique variant type");
            }

            self.0.push(state);
        } else {
            panic!("Each state has to have unique variant type");
        }

        self
    }
}
    
impl<T: Copy, V: Copy> States<T, V> {
    pub fn iter_elements(&self) -> StatesIter<'_, T, V> {
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

    pub fn get_basis(&self) -> StatesBasis<T, V> {
        self.iter_elements().collect()
    }
}

#[derive(Debug, Clone)]
pub struct StatesElement<T, V> {
    pub states_specific: Vec<T>,
    pub values: Vec<V>,
}

pub struct StatesIter<'a, T, V> {
    states: &'a [StateType<T, V>],
    states_iter: Vec<StateTypeIter<'a, T, V>>,
    current: StatesElement<T, V>,
    current_index: usize,
    size: usize,
}

impl<'a, T: Copy, V: Copy> Iterator for StatesIter<'a, T, V> {
    type Item = StatesElement<T, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.size {
            return None;
        }
        if self.current_index == 0 {
            for s in self.states_iter.iter_mut() {
                let (s_curr, v_curr) = s.next().unwrap(); // at least 1 element exists

                self.current.states_specific.push(*s_curr);
                self.current.values.push(*v_curr);
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
                    *s_spec = *s_spec_new;
                    *v = *v_new;
                    break
                },
                None => {
                    *s = s_type.iter();
                    let (s_curr, v_curr) = s.next().unwrap(); // at least 1 element exists
                    *s_spec = *s_curr;
                    *v = *v_curr;
                },
            }
        }
        self.current_index += 1;

        Some(self.current.clone())
    }
}

pub struct StatesBasis<T, V>(Vec<StatesElement<T, V>>);

impl<T, V> FromIterator<StatesElement<T, V>> for StatesBasis<T, V> {
    fn from_iter<I: IntoIterator<Item = StatesElement<T, V>>>(iter: I) -> Self {
        let mut elements = StatesBasis(vec![]);

        for val in iter {
            elements.0.push(val);
        }

        elements
    }
}

impl<T, V> Deref for StatesBasis<T, V> {
    type Target = Vec<StatesElement<T, V>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use irreducible_states::IrreducibleStates;

    use super::*;

    #[allow(unused)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    enum StateIds {
        ElectronSpin(u16),
        NuclearSpin(u16),
        Vibrational
    }

    #[allow(unused)]
    #[derive(Clone, Copy, Debug, PartialEq)]
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
        states.push_state(e_state);

        let nuclear_elements = vec![
            ElementValues::Spin(-1), 
            ElementValues::Spin(1)
        ];
        let nuclear = StateType::Irreducible(IrreducibleStates::new(StateIds::NuclearSpin(1), nuclear_elements));
        states.push_state(nuclear);


        let vib_elements = vec![
            ElementValues::Vibrational(-1),
            ElementValues::Vibrational(-2)
        ];
        let vib = StateType::Irreducible(IrreducibleStates::new(StateIds::Vibrational, vib_elements));
        states.push_state(vib);

        let expected = "States([Sum([IrreducibleStates { state_specific: ElectronSpin(2), basis: [Spin(-2), Spin(0), Spin(2)] }, \
                                    IrreducibleStates { state_specific: ElectronSpin(0), basis: [Spin(0)] }]), \
                                Irreducible(IrreducibleStates { state_specific: NuclearSpin(1), basis: [Spin(-1), Spin(1)] }), \
                                Irreducible(IrreducibleStates { state_specific: Vibrational, basis: [Vibrational(-1), Vibrational(-2)] })])";

        assert_eq!(expected, format!("{:?}", states))
    }

    #[test]
    fn states_creation_v2() {
        let mut states = States::default();

        let triplet = IrreducibleStates::new(StateIds::ElectronSpin(2), vec![-2, 0, 2]);
        let singlet = IrreducibleStates::new(StateIds::ElectronSpin(0), vec![0]);

        let e_state = StateType::Sum(vec![triplet, singlet]);
        states.push_state(e_state);

        let nuclear = StateType::Irreducible(IrreducibleStates::new(StateIds::NuclearSpin(1), vec![-1, 1]));
        states.push_state(nuclear);

        let vib = StateType::Irreducible(IrreducibleStates::new(StateIds::Vibrational, vec![-1, -2]));
        states.push_state(vib);

        let expected = "States([Sum([IrreducibleStates { state_specific: ElectronSpin(2), basis: [-2, 0, 2] }, \
                                    IrreducibleStates { state_specific: ElectronSpin(0), basis: [0] }]), \
                                Irreducible(IrreducibleStates { state_specific: NuclearSpin(1), basis: [-1, 1] }), \
                                Irreducible(IrreducibleStates { state_specific: Vibrational, basis: [-1, -2] })])";

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
        states.push_state(e_state);

        let nuclear = StateType::Irreducible(IrreducibleStates::new(StateIds::NuclearSpin(1), vec![-1, 1]));
        states.push_state(nuclear);

        let vib = StateType::Irreducible(IrreducibleStates::new(StateIds::Vibrational, vec![-1, -2]));
        states.push_state(vib);

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

        for (state, exp) in states.iter_elements().zip(expected.into_iter()) {
            assert_eq!(exp, format!("{:?}", state));
        }
    }

    #[test]
    fn state_filtering() {
        let mut states = States::default();

        let triplet = IrreducibleStates::new(StateIds::ElectronSpin(2), vec![-2, 0, 2]);
        let singlet = IrreducibleStates::new(StateIds::ElectronSpin(0), vec![0]);

        let e_state = StateType::Sum(vec![triplet, singlet]);
        states.push_state(e_state);

        let nuclear = StateType::Irreducible(IrreducibleStates::new(StateIds::NuclearSpin(1), vec![-1, 1]));
        states.push_state(nuclear);

        let vib = StateType::Irreducible(IrreducibleStates::new(StateIds::Vibrational, vec![-1, -2]));
        states.push_state(vib);

        let filtered: StatesBasis<StateIds, i32> = states
            .iter_elements()
            .filter(|s| s.states_specific[0] == StateIds::ElectronSpin(0))
            .collect();

        let expected: Vec<&str> = "\
StatesElement { states_specific: [ElectronSpin(0), NuclearSpin(1), Vibrational], values: [0, -1, -1] }
StatesElement { states_specific: [ElectronSpin(0), NuclearSpin(1), Vibrational], values: [0, 1, -1] }
StatesElement { states_specific: [ElectronSpin(0), NuclearSpin(1), Vibrational], values: [0, -1, -2] }
StatesElement { states_specific: [ElectronSpin(0), NuclearSpin(1), Vibrational], values: [0, 1, -2] }"
            .split("\n")
            .collect();

        for (state, &exp) in filtered.iter().zip(expected.iter()) {
            assert_eq!(exp, format!("{:?}", state));
        }
    }

    #[test]
    #[should_panic]
    fn test_wrong_state_initialization() {
        let mut states = States::default();

        let triplet_elements = vec![
            ElementValues::Spin(-2), 
            ElementValues::Spin(0), 
            ElementValues::Spin(2)
        ];
        let triplet = IrreducibleStates::new(StateIds::ElectronSpin(2), triplet_elements);
        let singlet = IrreducibleStates::new(StateIds::ElectronSpin(0), vec![ElementValues::Spin(0)]);

        let e_state = StateType::Sum(vec![triplet, singlet]);
        states.push_state(e_state);

        let nuclear_elements = vec![
            ElementValues::Spin(-1), 
            ElementValues::Spin(1)
        ];
        let nuclear = StateType::Irreducible(IrreducibleStates::new(StateIds::NuclearSpin(1), nuclear_elements));
        states.push_state(nuclear);

        let second_electron_spin = IrreducibleStates::new(StateIds::ElectronSpin(1), vec![ElementValues::Spin(-1), ElementValues::Spin(1)]);
        states.push_state(StateType::Irreducible(second_electron_spin));
    }
}