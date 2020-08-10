//! The Entry object, expressed for GraphQL.

use crate::ManagedOplDb;
use opltypes::WeightUnits::Kg;
use opltypes::{Age, PointsSystem, WeightClassKg};

use crate::graphql::gqltypes;
use crate::graphql::{Lifter, Meet};

// TODO: See if we can share these macros.
/// Helper for getting the OplDb.
macro_rules! db {
    ($executor:ident) => {
        &$executor.context().0
    };
}

/// Helper for looking up an [opldb::Entry].
macro_rules! entry {
    ($self: ident, $executor:ident) => {
        $executor.context().0.get_entry($self.0)
    };
}

/// A unique entry in the database.
///
/// Each entry corresponds to a division placing.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Entry(pub u32);

graphql_object!(Entry: ManagedOplDb |&self| {
    /// The meet in which the entry occurred.
    field meet(&executor) -> Meet {
        Meet(entry!(self, executor).meet_id)
    }

    /// The lifter corresponding to this entry.
    field lifter(&executor) -> Lifter {
        Lifter(entry!(self, executor).lifter_id)
    }

    /// The lifter's sex for this entry.
    field sex(&executor) -> String {
        format!("{}", entry!(self, executor).sex)
    }

    /// The event for this entry, like "SBD".
    field event(&executor) -> String {
        format!("{}", entry!(self, executor).event)
    }

    /// The equipment for this entry.
    field equipment(&executor) -> gqltypes::Equipment {
        entry!(self, executor).equipment.into()
    }

    /// The lifter's age at this entry.
    field age(&executor) -> Option<f64> {
        match entry!(self, executor).age {
            Age::None => None,
            age => Some(age.into()),
        }
    }

    /// The division for this entry.
    field division(&executor) -> Option<&str> {
        entry!(self, executor).get_division()
    }

    /// The lifter's bodyweight in kilograms.
    field bodyweight_kg(&executor) -> Option<f64> {
        entry!(self, executor).bodyweightkg.into()
    }
    /// The lifter's bodyweight in pounds.
    field bodyweight_lbs(&executor) -> Option<f64> {
        entry!(self, executor).bodyweightkg.as_lbs().into()
    }

    /// The lifter's weightclass in kilograms.
    ///
    /// This is a String because SHW classes have a "+" suffix.
    field weight_class_kg(&executor) -> Option<String> {
        match entry!(self, executor).weightclasskg {
            WeightClassKg::None => None,
            wc => Some(format!("{}", wc)),
        }
    }
    /// The lifter's weightclass in pounds.
    ///
    /// This is a String because SHW classes have a "+" suffix.
    field weight_class_lbs(&executor) -> Option<String> {
        match entry!(self, executor).weightclasskg {
            WeightClassKg::None => None,
            wc => Some(format!("{}", wc.as_lbs())),
        }
    }

    /// The first squat attempt in kilograms.
    field squat1_kg(&executor) -> Option<f64> {
        entry!(self, executor).squat1kg.into()
    }
    /// The first squat attempt in pounds.
    field squat1_lbs(&executor) -> Option<f64> {
        entry!(self, executor).squat1kg.as_lbs().into()
    }

    /// The second squat attempt in kilograms.
    field squat2_kg(&executor) -> Option<f64> {
        entry!(self, executor).squat2kg.into()
    }
    /// The second squat attempt in pounds.
    field squat2_lbs(&executor) -> Option<f64> {
        entry!(self, executor).squat2kg.as_lbs().into()
    }

    /// The third squat attempt in kilograms.
    field squat3_kg(&executor) -> Option<f64> {
        entry!(self, executor).squat3kg.into()
    }
    /// The third squat attempt in pounds.
    field squat3_lbs(&executor) -> Option<f64> {
        entry!(self, executor).squat3kg.as_lbs().into()
    }

    /// The fourth squat attempt in kilograms.
    field squat4_kg(&executor) -> Option<f64> {
        entry!(self, executor).squat4kg.into()
    }
    /// The third squat attempt in pounds.
    field squat4_lbs(&executor) -> Option<f64> {
        entry!(self, executor).squat4kg.as_lbs().into()
    }

    /// The best squat of the first 3 attempts in kilograms.
    field best3_squat_kg(&executor) -> Option<f64> {
        entry!(self, executor).best3squatkg.into()
    }
    /// The best squat of the first 3 attempts in pounds.
    field best3_squat_lbs(&executor) -> Option<f64> {
        entry!(self, executor).best3squatkg.as_lbs().into()
    }

    /// The best squat of the first 4 attempts in kilograms.
    field best4_squat_kg(&executor) -> Option<f64> {
        entry!(self, executor).highest_squatkg().into()
    }
    /// The best squat of the first 4 attempts in pounds.
    field best4_squat_lbs(&executor) -> Option<f64> {
        entry!(self, executor).highest_squatkg().as_lbs().into()
    }

    /// The first bench attempt in kilograms.
    field bench1_kg(&executor) -> Option<f64> {
        entry!(self, executor).bench1kg.into()
    }
    /// The first bench attempt in pounds.
    field bench1_lbs(&executor) -> Option<f64> {
        entry!(self, executor).bench1kg.as_lbs().into()
    }

    /// The second bench attempt in kilograms.
    field bench2_kg(&executor) -> Option<f64> {
        entry!(self, executor).bench2kg.into()
    }
    /// The second bench attempt in pounds.
    field bench2_lbs(&executor) -> Option<f64> {
        entry!(self, executor).bench2kg.as_lbs().into()
    }

    /// The third bench attempt in kilograms.
    field bench3_kg(&executor) -> Option<f64> {
        entry!(self, executor).bench3kg.into()
    }
    /// The third bench attempt in pounds.
    field bench3_lbs(&executor) -> Option<f64> {
        entry!(self, executor).bench3kg.as_lbs().into()
    }

    /// The fourth bench attempt in kilograms.
    field bench4_kg(&executor) -> Option<f64> {
        entry!(self, executor).bench4kg.into()
    }
    /// The third bench attempt in pounds.
    field bench4_lbs(&executor) -> Option<f64> {
        entry!(self, executor).bench4kg.as_lbs().into()
    }

    /// The best bench of the first 3 attempts in kilograms.
    field best3_bench_kg(&executor) -> Option<f64> {
        entry!(self, executor).best3benchkg.into()
    }
    /// The best bench of the first 3 attempts in pounds.
    field best3_bench_lbs(&executor) -> Option<f64> {
        entry!(self, executor).best3benchkg.as_lbs().into()
    }

    /// The best bench of the first 4 attempts in kilograms.
    field best4_bench_kg(&executor) -> Option<f64> {
        entry!(self, executor).highest_benchkg().into()
    }
    /// The best bench of the first 4 attempts in pounds.
    field best4_bench_lbs(&executor) -> Option<f64> {
        entry!(self, executor).highest_benchkg().as_lbs().into()
    }

    /// The first deadlift attempt in kilograms.
    field deadlift1_kg(&executor) -> Option<f64> {
        entry!(self, executor).deadlift1kg.into()
    }
    /// The first deadlift attempt in pounds.
    field deadlift1_lbs(&executor) -> Option<f64> {
        entry!(self, executor).deadlift1kg.as_lbs().into()
    }

    /// The second deadlift attempt in kilograms.
    field deadlift2_kg(&executor) -> Option<f64> {
        entry!(self, executor).deadlift2kg.into()
    }
    /// The second deadlift attempt in pounds.
    field deadlift2_lbs(&executor) -> Option<f64> {
        entry!(self, executor).deadlift2kg.as_lbs().into()
    }

    /// The third deadlift attempt in kilograms.
    field deadlift3_kg(&executor) -> Option<f64> {
        entry!(self, executor).deadlift3kg.into()
    }
    /// The third deadlift attempt in pounds.
    field deadlift3_lbs(&executor) -> Option<f64> {
        entry!(self, executor).deadlift3kg.as_lbs().into()
    }

    /// The fourth deadlift attempt in kilograms.
    field deadlift4_kg(&executor) -> Option<f64> {
        entry!(self, executor).deadlift4kg.into()
    }
    /// The third deadlift attempt in pounds.
    field deadlift4_lbs(&executor) -> Option<f64> {
        entry!(self, executor).deadlift4kg.as_lbs().into()
    }

    /// The best deadlift of the first 3 attempts in kilograms.
    field best3_deadlift_kg(&executor) -> Option<f64> {
        entry!(self, executor).best3deadliftkg.into()
    }
    /// The best deadlift of the first 3 attempts in pounds.
    field best3_deadlift_lbs(&executor) -> Option<f64> {
        entry!(self, executor).best3deadliftkg.as_lbs().into()
    }

    /// The best deadlift of the first 4 attempts in kilograms.
    field best4_deadlift_kg(&executor) -> Option<f64> {
        entry!(self, executor).highest_deadliftkg().into()
    }
    /// The best deadlift of the first 4 attempts in pounds.
    field best4_deadlift_lbs(&executor) -> Option<f64> {
        entry!(self, executor).highest_deadliftkg().as_lbs().into()
    }

    /// The event total in kilograms.
    field total_kg(&executor) -> Option<f64> {
        entry!(self, executor).totalkg.into()
    }
    /// The event total in pounds.
    field total_lbs(&executor) -> Option<f64> {
        entry!(self, executor).totalkg.as_lbs().into()
    }

    /// The entry's place.
    field place(&executor) -> String {
        format!("{}", entry!(self, executor).place)
    }

    /// AH points.
    field ah(&executor) -> f64 {
        db!(executor).get_entry(self.0).points(PointsSystem::AH, Kg).into()
    }
    /// Dots points.
    field dots(&executor) -> f64 {
        db!(executor).get_entry(self.0).points(PointsSystem::Dots, Kg).into()
    }
    /// Glossbrenner points.
    field glossbrenner(&executor) -> f64 {
        db!(executor).get_entry(self.0).points(PointsSystem::Glossbrenner, Kg).into()
    }
    /// IPF Goodlift points.
    field goodlift(&executor) -> f64 {
        db!(executor).get_entry(self.0).points(PointsSystem::Goodlift, Kg).into()
    }
    /// IPF points.
    field ipf_points(&executor) -> f64 {
        db!(executor).get_entry(self.0).points(PointsSystem::IPFPoints, Kg).into()
    }
    /// McCulloch points.
    field mcculloch(&executor) -> f64 {
        db!(executor).get_entry(self.0).points(PointsSystem::McCulloch, Kg).into()
    }
    /// NASA points.
    field nasa(&executor) -> f64 {
        db!(executor).get_entry(self.0).points(PointsSystem::NASA, Kg).into()
    }
    /// Reshel points.
    field reshel(&executor) -> f64 {
        db!(executor).get_entry(self.0).points(PointsSystem::Reshel, Kg).into()
    }
    /// Schwartz/Malone points.
    field schwartz_malone(&executor) -> f64 {
        db!(executor).get_entry(self.0).points(PointsSystem::SchwartzMalone, Kg).into()
    }
    /// Wilks points.
    field wilks(&executor) -> f64 {
        db!(executor).get_entry(self.0).points(PointsSystem::Wilks, Kg).into()
    }
    /// Wilks2020 points.
    field wilks2020(&executor) -> f64 {
        db!(executor).get_entry(self.0).points(PointsSystem::Wilks2020, Kg).into()
    }

    /// Whether this entry counts as drug-tested.
    field tested(&executor) -> bool {
        entry!(self, executor).tested
    }

    // TODO: AgeClass
    // TODO: BirthYearClass
    // TODO: LifterCountry
    // TODO: LifterState
});
