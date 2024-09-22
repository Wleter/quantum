use std::{any::{Any, TypeId}, collections::HashMap};

/// Struct to hold internal parameters.
/// Used to store information about a particle and composition of particles.
#[derive(Default)]
pub struct Params {
    params: HashMap<TypeId, Box<dyn Any>>,
}

impl Params {
    /// Insert or replace unique parameter of type `T`.
    pub fn insert<T: 'static>(&mut self, value: T) -> &mut Self {
        self.params.insert(TypeId::of::<T>(), Box::new(value));

        self
    }

    /// Returns the value of parameter of type `T` with given name if it exists.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.params.get(&TypeId::of::<T>())
            .map(|value| {
                value.downcast_ref::<T>()
            })
            .flatten()
    }
}