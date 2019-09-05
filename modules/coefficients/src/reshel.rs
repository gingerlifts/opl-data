//! Definition of Reshel points.

use opltypes::*;

// TODO: Improve the accuracy of this implementation. Shared with OpenLifter.
// This implementation uses a curve of best fit from GNUPlot.
// At worst, it is off by about six Reshel points (0.01), affecting middleweights.
fn reshel_coefficient_men(bodyweightkg: f64) -> f64 {
    const A: f64 = 23740.8329088123;
    const B: f64 = -9.75618720662844;
    const C: f64 = 0.787990994925928;
    const D: f64 = -2.68445158813578;

    let normalized = bodyweightkg.max(50.0).min(174.75);
    A * (normalized + B).powf(D) + C
}

// TODO: Improve the accuracy of this implementation. Shared with OpenLifter.
// This implementation uses a curve of best fit from GNUPlot.
// At worst, it is off by about six Reshel points (0.01), affecting middleweights.
fn reshel_coefficient_women(bodyweightkg: f64) -> f64 {
    const A: f64 = 239.894659799145;
    const B: f64 = -20.5105859285582;
    const C: f64 = 1.16052601684125;
    const D: f64 = -1.61417872668708;

    let normalized = bodyweightkg.max(40.0).min(118.75);
    A * (normalized + B).powf(D) + C
}

/// Calculates Reshel points.
///
/// Reshel points are published only as heavily-rounded coefficient tables,
/// separately for men and women: http://www.irondawg.com/reshel_formula.htm.
pub fn reshel(sex: Sex, bodyweight: WeightKg, total: WeightKg) -> Points {
    if bodyweight.is_zero() || total.is_zero() {
        return Points::from_i32(0);
    }
    let coefficient: f64 = match sex {
        Sex::M => reshel_coefficient_men(f64::from(bodyweight)),
        Sex::F => reshel_coefficient_women(f64::from(bodyweight)),
    };
    Points::from(coefficient * f64::from(total))
}

// TODO: Tests. But at the moment, they don't match the tables.
