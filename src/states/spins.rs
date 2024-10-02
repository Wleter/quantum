#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DoubleSpin(u32, i32);

pub struct SpinOperators;

impl SpinOperators {
    pub fn proj_z(dspin_bra: DoubleSpin, dspin_ket: DoubleSpin) -> f64 {
        if dspin_bra == dspin_ket {
            dspin_bra.1 as f64
        } else {
            0.0
        }
    }

    pub fn ladder_plus(dspin_bra: DoubleSpin, dspin_ket: DoubleSpin) -> f64 {
        if dspin_bra.0 == dspin_ket.0 && dspin_bra.1 == dspin_ket.1 + 2 {
            (((dspin_ket.0 * (dspin_ket.0 + 2)) as i32 - dspin_bra.1 * dspin_ket.1) as f64).sqrt()
                / 2.0
        } else {
            0.0
        }
    }

    pub fn ladder_minus(dspin_bra: DoubleSpin, dspin_ket: DoubleSpin) -> f64 {
        if dspin_bra.0 == dspin_ket.0 && dspin_bra.1 + 2 == dspin_ket.1 {
            (((dspin_ket.0 * (dspin_ket.0 + 2)) as i32 - dspin_bra.1 * dspin_ket.1) as f64).sqrt()
                / 2.0
        } else {
            0.0
        }
    }

    pub fn dot(
        dspin1_braket: (DoubleSpin, DoubleSpin),
        dspin2_braket: (DoubleSpin, DoubleSpin),
    ) -> f64 {
        let val1 = Self::proj_z(dspin1_braket.0, dspin1_braket.1)
            * Self::proj_z(dspin2_braket.0, dspin2_braket.1);
        let val2 = 0.5
            * Self::ladder_plus(dspin1_braket.0, dspin1_braket.1)
            * Self::ladder_minus(dspin2_braket.0, dspin2_braket.1);
        let val3 = 0.5
            * Self::ladder_minus(dspin1_braket.0, dspin1_braket.1)
            * Self::ladder_plus(dspin2_braket.0, dspin2_braket.1);

        val1 + val2 + val3
    }
}
