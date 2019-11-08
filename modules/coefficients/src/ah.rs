//! Definition of the AH Formula, also called Haleczko. Used by ParaPL.

use opltypes::*;

/// Calculates the AH coefficient for men.
///
/// The full formula is defined in Excel:
///  =ROUND($AM$1/(POWER(LOG(I13),$AM$2))*M13,2)
///
/// Where:
///  I13: Bodyweight
///  M13: Lift Attempt
///  AM1: 3.2695
///  AM2: 1.95
pub fn ah_coefficient_men(bodyweightkg: f64) -> f64 {
    const AM1: f64 = 3.2695;
    const AM2: f64 = 1.95;

    let adjusted = bodyweightkg.max(32.0).min(157.0);

    AM1 / adjusted.log10().powf(AM2)
}

/// Calculates the AH coefficient for women.
///
/// The full formula is defined in Excel:
///  =ROUND($AG$1/(POWER(LOG(I13),$AG$10))*M13,2)
///
/// Where:
///  I13: Bodyweight
///  M13: Lift Attempt
///  AG1: 2.7566
///  AG10: 1.8
pub fn ah_coefficient_women(bodyweightkg: f64) -> f64 {
    const AG1: f64 = 2.7566;
    const AG10: f64 = 1.8;

    let adjusted = bodyweightkg.max(28.0).min(112.0);

    AG1 / adjusted.log10().powf(AG10)
}

/// Calculates AH points, used by ParaPL for bench-only competitions.
///
/// https://www.paralympic.org/sites/default/files/document/130801141325417_Appendix_2_AH_Haleczko_Formula.pdf
pub fn ah(sex: Sex, bodyweight: WeightKg, total: WeightKg) -> Points {
    if bodyweight.is_zero() || total.is_zero() {
        return Points::from_i32(0);
    }
    let coefficient: f64 = match sex {
        Sex::M => ah_coefficient_men(f64::from(bodyweight)),
        Sex::F => ah_coefficient_women(f64::from(bodyweight)),
    };
    Points::from(coefficient * f64::from(total))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests whether two floating-point numbers are equal to six decimal places,
    /// as published in the official AH coefficient tables.
    fn matches_table(a: f64, b: f64) -> bool {
        const FIGS: f64 = 1000000.0;
        (a * FIGS).round() == (b * FIGS).round()
    }

    #[test]
    fn male_coefficients() {
        assert!(matches_table(ah_coefficient_men(32.0), 1.472993));
        assert!(matches_table(ah_coefficient_men(60.0), 1.064247));
        assert!(matches_table(ah_coefficient_men(80.0), 0.932257));
        assert!(matches_table(ah_coefficient_men(100.0), 0.846200));
        assert!(matches_table(ah_coefficient_men(117.0), 0.792650));
        assert!(matches_table(ah_coefficient_men(144.0), 0.729355));
        assert!(matches_table(ah_coefficient_men(157.0), 0.705240));
    }

    #[test]
    fn female_coefficients() {
        assert!(matches_table(ah_coefficient_women(28.0), 1.417245));
        assert!(matches_table(ah_coefficient_women(35.0), 1.261172));
        assert!(matches_table(ah_coefficient_women(48.0), 1.082031));
        assert!(matches_table(ah_coefficient_women(70.0), 0.915248));
        assert!(matches_table(ah_coefficient_women(89.0), 0.829003));
        assert!(matches_table(ah_coefficient_women(100.0), 0.791625));
        assert!(matches_table(ah_coefficient_women(112.0), 0.757731));
    }
}
