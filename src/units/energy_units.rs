/// Enum for mass unit conversion
/// # Examples
/// ```
/// use split_operator::units::EnergyUnit;
/// let energy_kelvin = 1.0;
/// let energy_au = EnergyUnit::Kelvin.to_au(energy_kelvin);
pub enum EnergyUnit {
    Au,
    Kelvin,
    CmInv,
}

impl EnergyUnit {
    fn to_au_mul(&self) -> f64 {
        match self {
            EnergyUnit::Au => 1.0,
            EnergyUnit::Kelvin => 3.1668105e-6,
            EnergyUnit::CmInv => 219474.63,
        }
    }

    pub fn to_au(&self, energy: f64) -> f64 {
        match self {
            EnergyUnit::Au => energy,
            _ => energy * self.to_au_mul(),
        }
    }

    pub fn to_cm_inv(&self, energy: f64) -> f64 {
        match self {
            EnergyUnit::Au => energy / EnergyUnit::CmInv.to_au_mul(),
            _ => EnergyUnit::Au.to_cm_inv(self.to_au(energy)),
        }
    }

    pub fn to_kelvin(&self, energy: f64) -> f64 {
        match self {
            EnergyUnit::Au => energy / EnergyUnit::Kelvin.to_au_mul(),
            _ => EnergyUnit::Au.to_kelvin(self.to_au(energy)),
        }
    }
}
