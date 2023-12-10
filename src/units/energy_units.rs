use super::Unit;

/// Enum for representing energy unit values 
/// # Examples
/// ```
/// use quantum::units::energy_units::{Energy, Kelvin, CmInv}};
/// let energy_kelvin: Energy = Energy::new(1.0, Kelvin);
/// let energy_cm_inv: Energy = energy_kelvin.to(CmInv);
/// let energy = energy_kelvin.to_au();
pub struct Energy<U: Unit> {
    pub value: f64,
    pub unit: U,
}

impl<U: Unit> Energy<U> {
    pub fn new(value: f64, unit: U) -> Self {
        Self { value, unit }
    }

    pub fn to_au(&self) -> f64 {
        self.unit.to_au(self.value)
    }

    pub fn to<V: Unit>(&self, unit: V) -> Energy<V> {
        Energy {
            value: self.unit.to_au(self.value) / unit.to_au(1.0),
            unit,
        }
    }
}

pub struct Kelvin;
#[allow(dead_code)]
impl Unit for Kelvin {
    const TO_AU_MUL: f64 = 3.1668105e-6;
}

pub struct CmInv;
#[allow(dead_code)]
impl Unit for CmInv {
    const TO_AU_MUL: f64 = 4.5563352812e-6;
}

pub struct MHz;
#[allow(dead_code)]
impl Unit for MHz {
    const TO_AU_MUL: f64 = 1.51982850071586e-10;
}

pub struct GHz;
#[allow(dead_code)]
impl Unit for GHz {
    const TO_AU_MUL: f64 = 1.51982850071586e-07;
}


/// Enum for energy unit conversion
/// # Examples
/// ```
/// use quantum::units::energy_units::EnergyUnit;
/// let energy_kelvin = 1.0;
/// let energy_au = EnergyUnit::Kelvin.to_au(energy_kelvin);
#[deprecated(note = "Use Energy struct instead")]
pub enum EnergyUnit {
    Au,
    Kelvin,
    CmInv,
    MHz,
    GHz,
}


#[deprecated(note = "Use Energy enum instead")]
#[allow(warnings)]
impl EnergyUnit {
    #[deprecated(note = "Use Energy enum instead")]
    fn to_au_mul(&self) -> f64 {
        match self {
            EnergyUnit::Au => 1.0,
            EnergyUnit::Kelvin => 3.1668105e-6,
            EnergyUnit::CmInv => 1.0 / 219474.63,
            EnergyUnit::MHz => 1.51982850071586e-10,
            EnergyUnit::GHz => 1.51982850071586e-07,
        }
    }
    #[deprecated(note = "Use Energy enum instead")]
    pub fn to_au(&self, energy: f64) -> f64 {
        match self {
            EnergyUnit::Au => energy,
            _ => energy * self.to_au_mul(),
        }
    }
    #[deprecated(note = "Use Energy enum instead")]
    pub fn to_cm_inv(&self, energy: f64) -> f64 {
        match self {
            EnergyUnit::Au => energy / EnergyUnit::CmInv.to_au_mul(),
            _ => EnergyUnit::Au.to_cm_inv(self.to_au(energy)),
        }
    }
    #[deprecated(note = "Use Energy enum instead")]
    pub fn to_kelvin(&self, energy: f64) -> f64 {
        match self {
            EnergyUnit::Au => energy / EnergyUnit::Kelvin.to_au_mul(),
            _ => EnergyUnit::Au.to_kelvin(self.to_au(energy)),
        }
    }

    #[deprecated(note = "Use Energy enum instead")]
    pub fn to_mega_hz(&self, energy: f64) -> f64 {
        match self {
            EnergyUnit::Au => energy / EnergyUnit::MHz.to_au_mul(),
            _ => EnergyUnit::Au.to_mega_hz(self.to_au(energy)),
        }
    }
    #[deprecated(note = "Use Energy enum instead")]
    pub fn to_giga_hz(&self, energy: f64) -> f64 {
        match self {
            EnergyUnit::Au => energy / EnergyUnit::GHz.to_au_mul(),
            _ => EnergyUnit::Au.to_giga_hz(self.to_au(energy)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn energy_units() {
        let energy_kelvin = Energy::new(1.0, Kelvin);
        let energy_cm_inv = energy_kelvin.to(CmInv);
        let energy_from_kelvin = energy_kelvin.to_au();
        let energy_from_cm_inv = energy_cm_inv.to_au();
        assert_eq!(energy_from_kelvin, energy_from_cm_inv);
        assert!(energy_cm_inv.value > 0.6950);
        assert!(energy_cm_inv.value < 0.6951);
    }
}