use std::ops::Deref;

use faer::{Entity, Mat};
use nalgebra::{DMatrix, Dim, SMatrix};
use ndarray::Array;

use crate::states::{braket::Braket, StatesBasis};

pub struct Operator<M> {
    backed: M
}

impl<E: Entity> Operator<Mat<E>> {
    fn from_matrix_elements<T, V, F: Fn(Braket<T, V>) -> E>(elements: StatesBasis<T, V>, f: F) -> Self {
        todo!()
    }
}

impl<E: Entity> Deref for Operator<Mat<E>> {
    type Target = Mat<E>;

    fn deref(&self) -> &Self::Target {
        &self.backed
    }
}

impl<E> Operator<DMatrix<E>> {
    fn from_matrix_elements<T, V, F: Fn(Braket<T, V>) -> E>(elements: StatesBasis<T, V>, f: F) -> Self {
        todo!()
    }
}

impl<E> Deref for Operator<DMatrix<E>> {
    type Target = DMatrix<E>;

    fn deref(&self) -> &Self::Target {
        &self.backed
    }
}

impl<const N: usize, E> Operator<SMatrix<E, N, N>> {
    fn from_matrix_elements<T, V, F: Fn(Braket<T, V>) -> E>(elements: StatesBasis<T, V>, f: F) -> Self {
        assert!(N < 10, "For larger matrices use DMatrix backed matrices instead");

        todo!()
    }
}

impl<const N: usize, E> Deref for Operator<SMatrix<E, N, N>> {
    type Target = SMatrix<E, N, N>;

    fn deref(&self) -> &Self::Target {
        &self.backed
    }
}

impl<N: Dim, E> Operator<Array<E, N>> {
    fn from_matrix_elements<T, V, F: Fn(Braket<T, V>) -> E>(elements: StatesBasis<T, V>, f: F) -> Self {
        todo!()
    }
}

impl<N: Dim, E> Deref for Operator<Array<E, N>> {
    type Target = Array<E, N>;

    fn deref(&self) -> &Self::Target {
        &self.backed
    }
}