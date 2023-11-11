pub mod energy_units;
pub mod mass_units;

pub fn convert_data_units(data: &Vec<f64>, conversion: fn(f64) -> f64) -> Vec<f64> {
    data.iter().map(|x| conversion(*x)).collect()
}