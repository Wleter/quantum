use std::{mem::discriminant, ops::Deref};

use faer::{Entity, Mat};
use nalgebra::{DMatrix, Dim, SMatrix};
use ndarray::{Array, Array2};
use num::traits::Zero;

use crate::states::{braket::StateBraket, StatesBasis};

pub struct Operator<M> {
    backed: M
}

impl<E: Entity + Zero> Operator<Mat<E>> {
    pub fn from_matrix_elements<const N: usize, T: Copy + PartialEq, V: Copy + PartialEq, F>(elements: &StatesBasis<T, V>, action_states: [T; N], mat_element: F) -> Self 
        where F: Fn([StateBraket<T, V>; N]) -> E
    {
        let first = elements.first().unwrap_or_else(|| panic!("0 size states basis")); // same variants for other elements

        let indices = action_states.map(|s| {
            first.states_specific.iter()
                .enumerate()
                .find(|(_, &x)| discriminant(&x) == discriminant(&s))  // variants are distinct by creation in States
                .map_or_else(|| panic!("action state not found in elements"), |x| x.0)
        });

        let diagonal_indices: Vec<usize> = (0..first.states_specific.len())
            .into_iter()
            .filter(|x| !indices.contains(&x))
            .collect();
        
        let mat = Mat::from_fn(elements.len(), elements.len(), |i, j| {
            unsafe {
                let elements_i = elements.get_unchecked(i);
                let elements_j = elements.get_unchecked(j);

                for &index in &diagonal_indices {
                    if elements_i.states_specific.get_unchecked(index) != elements_j.states_specific.get_unchecked(index)
                        || elements_i.values.get_unchecked(index) != elements_j.values.get_unchecked(index) 
                    {
                        return E::zero()
                    }
                }

                let brakets = indices.map(|index| {
                    let ket = (
                        *elements_i.states_specific.get_unchecked(index),
                        *elements_i.values.get_unchecked(index)
                    );

                    let bra = (
                        *elements_j.states_specific.get_unchecked(index),
                        *elements_j.values.get_unchecked(index)
                    );

                    StateBraket {
                        ket,
                        bra
                    }
                });

                mat_element(brakets)  // introduce caching
            }
        });

        Self {
            backed: mat
        }
    }
}

impl<E: Entity> Deref for Operator<Mat<E>> {
    type Target = Mat<E>;

    fn deref(&self) -> &Self::Target {
        &self.backed
    }
}

impl<E: nalgebra::Scalar + Zero> Operator<DMatrix<E>> {
    pub fn from_matrix_elements<const N: usize, T: Copy + PartialEq, V: Copy + PartialEq, F>(elements: &StatesBasis<T, V>, action_states: [T; N], mat_element: F) -> Self 
        where F: Fn([StateBraket<T, V>; N]) -> E 
    {
        let first = elements.first().unwrap_or_else(|| panic!("0 size states basis")); // same variants for other elements

        let indices = action_states.map(|s| {
            first.states_specific.iter()
                .enumerate()
                .find(|(_, &x)| discriminant(&x) == discriminant(&s))  // variants are distinct by creation in States
                .map_or_else(|| panic!("action state not found in elements"), |x| x.0)
        });

        let diagonal_indices: Vec<usize> = (0..first.states_specific.len())
            .into_iter()
            .filter(|x| !indices.contains(&x))
            .collect();
        
        let mat = DMatrix::from_fn(elements.len(), elements.len(), |i, j| {
            unsafe {
                let elements_i = elements.get_unchecked(i);
                let elements_j = elements.get_unchecked(j);

                for &index in &diagonal_indices {
                    if elements_i.states_specific.get_unchecked(index) != elements_j.states_specific.get_unchecked(index)
                        || elements_i.values.get_unchecked(index) != elements_j.values.get_unchecked(index) 
                    {
                        return E::zero()
                    }
                }

                let brakets = indices.map(|index| {
                    let ket = (
                        *elements_i.states_specific.get_unchecked(index),
                        *elements_i.values.get_unchecked(index)
                    );

                    let bra = (
                        *elements_j.states_specific.get_unchecked(index),
                        *elements_j.values.get_unchecked(index)
                    );

                    StateBraket {
                        ket,
                        bra
                    }
                });

                mat_element(brakets)  // introduce caching
            }
        });

        Self {
            backed: mat
        }
    }
}

impl<E> Deref for Operator<DMatrix<E>> {
    type Target = DMatrix<E>;

    fn deref(&self) -> &Self::Target {
        &self.backed
    }
}

impl<const N: usize, E: nalgebra::Scalar + Zero> Operator<SMatrix<E, N, N>> {
    pub fn from_matrix_elements<const M: usize, T: Copy + PartialEq, V: Copy + PartialEq, F>(elements: &StatesBasis<T, V>, action_states: [T; M], mat_element: F) -> Self 
        where F: Fn([StateBraket<T, V>; M]) -> E 
    {
        assert!(N < 10, "For larger matrices use DMatrix backed matrices instead");
        assert!(N == elements.len(), "Elements does not have the same size as static matrix size");

        let first = elements.first().unwrap_or_else(|| panic!("0 size states basis")); // same variants for other elements

        let indices = action_states.map(|s| {
            first.states_specific.iter()
                .enumerate()
                .find(|(_, &x)| discriminant(&x) == discriminant(&s))  // variants are distinct by creation in States
                .map_or_else(|| panic!("action state not found in elements"), |x| x.0)
        });

        let diagonal_indices: Vec<usize> = (0..first.states_specific.len())
            .into_iter()
            .filter(|x| !indices.contains(&x))
            .collect();
        
        let mat = SMatrix::from_fn(|i, j| {
            unsafe {
                let elements_i = elements.get_unchecked(i);
                let elements_j = elements.get_unchecked(j);

                for &index in &diagonal_indices {
                    if elements_i.states_specific.get_unchecked(index) != elements_j.states_specific.get_unchecked(index)
                        || elements_i.values.get_unchecked(index) != elements_j.values.get_unchecked(index) 
                    {
                        return E::zero()
                    }
                }

                let brakets = indices.map(|index| {
                    let ket = (
                        *elements_i.states_specific.get_unchecked(index),
                        *elements_i.values.get_unchecked(index)
                    );

                    let bra = (
                        *elements_j.states_specific.get_unchecked(index),
                        *elements_j.values.get_unchecked(index)
                    );

                    StateBraket {
                        ket,
                        bra
                    }
                });

                mat_element(brakets)  // introduce caching
            }
        });

        Self {
            backed: mat
        }
    }
}

impl<const N: usize, E> Deref for Operator<SMatrix<E, N, N>> {
    type Target = SMatrix<E, N, N>;

    fn deref(&self) -> &Self::Target {
        &self.backed
    }
}

impl<E: Zero> Operator<Array2<E>> {
    pub fn from_matrix_elements<const N: usize, T: Copy + PartialEq, V: Copy + PartialEq, F>(elements: &StatesBasis<T, V>, action_states: [T; N], mat_element: F) -> Self 
        where F: Fn([StateBraket<T, V>; N]) -> E 
    {
        let first = elements.first().unwrap_or_else(|| panic!("0 size states basis")); // same variants for other elements

        let indices = action_states.map(|s| {
            first.states_specific.iter()
                .enumerate()
                .find(|(_, &x)| discriminant(&x) == discriminant(&s))  // variants are distinct by creation in States
                .map_or_else(|| panic!("action state not found in elements"), |x| x.0)
        });

        let diagonal_indices: Vec<usize> = (0..first.states_specific.len())
            .into_iter()
            .filter(|x| !indices.contains(&x))
            .collect();
        
        let mat = Array2::from_shape_fn((elements.len(), elements.len()), |(i, j)| {
            unsafe {
                let elements_i = elements.get_unchecked(i);
                let elements_j = elements.get_unchecked(j);

                for &index in &diagonal_indices {
                    if elements_i.states_specific.get_unchecked(index) != elements_j.states_specific.get_unchecked(index)
                        || elements_i.values.get_unchecked(index) != elements_j.values.get_unchecked(index) 
                    {
                        return E::zero()
                    }
                }

                let brakets = indices.map(|index| {
                    let ket = (
                        *elements_i.states_specific.get_unchecked(index),
                        *elements_i.values.get_unchecked(index)
                    );

                    let bra = (
                        *elements_j.states_specific.get_unchecked(index),
                        *elements_j.values.get_unchecked(index)
                    );

                    StateBraket {
                        ket,
                        bra
                    }
                });

                mat_element(brakets)  // introduce caching
            }
        });

        Self {
            backed: mat
        }
    }
}

impl<N: Dim, E> Deref for Operator<Array<E, N>> {
    type Target = Array<E, N>;

    fn deref(&self) -> &Self::Target {
        &self.backed
    }
}

#[cfg(test)]
mod test {
    use faer::{mat, Mat};
    use nalgebra::{DMatrix, SMatrix};
    use ndarray::Array2;

    use crate::states::{braket::StateBraket, irreducible_states::IrreducibleStates, state_type::StateType, States};

    use super::Operator;


    #[derive(Clone, Copy, Debug, PartialEq)]
    enum StateIds {
        ElectronSpin(u16),
        Vibrational
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum ElementValues {
        Spin(i16),
        Vibrational(i16)
    }

    fn prepare_states() -> States<StateIds, ElementValues> {
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

        let vib_elements = vec![
            ElementValues::Vibrational(-1),
            ElementValues::Vibrational(-2)
        ];
        let vib = StateType::Irreducible(IrreducibleStates::new(StateIds::Vibrational, vib_elements));
        states.push_state(vib);

        states
    }

    #[test]
    fn test_faer_operator() {
        let elements = prepare_states().get_basis();

        let operator = Operator::<Mat<f64>>::from_matrix_elements(&elements, [StateIds::ElectronSpin(0)], |[el_state]| {
            let ket = el_state.ket;

            match ket.1 {
                ElementValues::Spin(val) => val as f64,
                ElementValues::Vibrational(val) => val as f64,
            }
        });
        
        let expected = mat![
            [-2.0, -2.0, -2.0, -2.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [2.0, 2.0, 2.0, 2.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, -2.0, -2.0, -2.0, -2.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 2.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        ];

        assert_eq!(expected, operator.backed);

        let operator = Operator::<Mat<f64>>::from_matrix_elements(&elements, [StateIds::ElectronSpin(0)], |[el_state]| {
            let bra = el_state.bra;

            match bra.1 {
                ElementValues::Spin(val) => val as f64,
                ElementValues::Vibrational(val) => val as f64,
            }
        });

        let expected = mat![
            [-2.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [-2.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [-2.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [-2.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, -2.0, 0.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, -2.0, 0.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, -2.0, 0.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, -2.0, 0.0, 2.0, 0.0],
        ];

        assert_eq!(expected, operator.backed);

        let operator = Operator::<Mat<f64>>::from_matrix_elements(&elements, [StateIds::ElectronSpin(0), StateIds::Vibrational], |[el_state, vib]| {
            if vib.ket != vib.bra {
                let StateIds::ElectronSpin(ket_spin) = el_state.ket.0 else { unreachable!() };
                let StateIds::ElectronSpin(bra_spin) = el_state.bra.0 else { unreachable!() };
                let ElementValues::Spin(ket_spin_z) = el_state.ket.1 else { panic!("wrong state variant") };
                let ElementValues::Spin(bra_spin_z) = el_state.bra.1 else { panic!("wrong state variant") };

                ((ket_spin * 1000 + bra_spin * 100) as i16 + ket_spin_z * 10 + bra_spin_z) as f64
            } else {
                0.0
            }
        });

        let expected = mat![
            [0.0, 0.0, 0.0, 0.0, 2178.0, 2180.0, 2182.0, 1980.0],
            [0.0, 0.0, 0.0, 0.0, 2198.0, 2200.0, 2202.0, 2000.0],
            [0.0, 0.0, 0.0, 0.0, 2218.0, 2220.0, 2222.0, 2020.0],
            [0.0, 0.0, 0.0, 0.0, 198.0, 200.0, 202.0, 0.0],
            [2178.0, 2180.0, 2182.0, 1980.0, 0.0, 0.0, 0.0, 0.0],
            [2198.0, 2200.0, 2202.0, 2000.0, 0.0, 0.0, 0.0, 0.0],
            [2218.0, 2220.0, 2222.0, 2020.0, 0.0, 0.0, 0.0, 0.0],
            [198.0, 200.0, 202.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        ];

        println!("{:?}", operator.backed);
        assert_eq!(expected, operator.backed);
    }

    #[test]
    fn test_operators_different_backed() {
        let elements = prepare_states().get_basis();

        let matrix_elements = |[el_state, vib]: [StateBraket<StateIds, ElementValues>; 2]| {
            if vib.ket != vib.bra {
                let StateIds::ElectronSpin(ket_spin) = el_state.ket.0 else { unreachable!() };
                let StateIds::ElectronSpin(bra_spin) = el_state.bra.0 else { unreachable!() };
                let ElementValues::Spin(ket_spin_z) = el_state.ket.1 else { panic!("wrong state variant") };
                let ElementValues::Spin(bra_spin_z) = el_state.bra.1 else { panic!("wrong state variant") };

                ((ket_spin * 1000 + bra_spin * 100) as i16 + ket_spin_z * 10 + bra_spin_z) as f64
            } else {
                0.0
            }
        };

        let operator_faer = Operator::<Mat<f64>>::from_matrix_elements(&elements, [StateIds::ElectronSpin(0), StateIds::Vibrational], matrix_elements.clone());
        let operator_dmatrix = Operator::<DMatrix<f64>>::from_matrix_elements(&elements, [StateIds::ElectronSpin(0), StateIds::Vibrational], matrix_elements.clone());
        let operator_smatrix = Operator::<SMatrix<f64, 8, 8>>::from_matrix_elements(&elements, [StateIds::ElectronSpin(0), StateIds::Vibrational], matrix_elements.clone());
        let operator_ndarray = Operator::<Array2<f64>>::from_matrix_elements(&elements, [StateIds::ElectronSpin(0), StateIds::Vibrational], matrix_elements.clone());

        let faer_slice: Vec<f64> = operator_faer.col_iter()
            .map(|c| c.try_as_slice().unwrap().to_owned())
            .flatten()
            .collect();

        assert_eq!(&faer_slice, operator_dmatrix.backed.as_slice());
        assert_eq!(operator_dmatrix.backed.as_slice(), operator_smatrix.backed.as_slice());
        assert_eq!(operator_dmatrix.backed.transpose().as_slice(), operator_ndarray.backed.as_slice().unwrap()); // transpose since the memory layout is different for ndarray

    }
}