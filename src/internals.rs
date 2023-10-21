use std::collections::HashMap;

pub type Scalable = (f64, f64);

/// Struct to hold internal parameters of type `T`.
/// Used to store information about a particle and composition of particles.
#[derive(Default, Clone)]
pub struct Internals<T> {
    params: HashMap<&'static str, T>
}

impl<T> Internals<T> {
    pub fn new() -> Self {
        Internals { 
            params: HashMap::new() 
        }
    }

    /// Adds a parameter with given name and value and returns a mutable reference to self.
    pub fn insert_param(&mut self, name: &'static str, value: T) -> &mut Self  {
        self.params.insert(name, value);

        self
    }

    /// Returns the value of a parameter with given name. Panics if no name is found.
    pub fn get_param(&self, name: &'static str) -> &T {
        self.params.get(name).unwrap()
    }
}

impl Internals<Scalable> {
    /// Adds a parameter with given name and value and returns a mutable reference to self.
    pub fn insert_value(&mut self, name: &'static str, value: f64) -> &mut Self {
        self.params.insert(name, (value, 0.0));

        self
    }

    /// Returns the scaled value of a parameter with given name. Panics if no name is found.
    pub fn get_value(&self, name: &'static str) -> f64 {
        let param = self.params.get(name).unwrap();
        param.0 * param.1
    }

    /// Set a scale of a parameter with given name.
    pub fn set_value_scale(&mut self, name: &'static str, scale: f64) {
        if let Some(&mut mut param) = self.params.get_mut(name) {
            param.1 = scale;
        }
    }

    /// Scales a parameter with given name by given scaling factor. Can be used multiple times.
    pub fn scale_value(&mut self, name: &'static str, scaling: f64) {
        if let Some(&mut mut param) = self.params.get_mut(name) {
            param.1 *= scaling;
        }
    }
}