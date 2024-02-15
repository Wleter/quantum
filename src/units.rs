pub mod energy_units;
pub mod mass_units;
pub mod distance_units;

/// Trait for units that can be converted to atomic units.
pub trait Unit: Copy + Clone + Sized {
    const TO_AU_MUL: f64;

    fn to_au(&self, value: f64) -> f64 {
        value * Self::TO_AU_MUL
    }
}

#[derive(Copy, Clone)]
pub struct Au;

impl Unit for Au {
    const TO_AU_MUL: f64 = 1.0;
}

pub fn convert_data_units(data: &Vec<f64>, conversion: fn(f64) -> f64) -> Vec<f64> {
    data.iter().map(|x| conversion(*x)).collect()
}