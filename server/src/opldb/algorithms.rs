//! Shared algorithms that operate on StaticCache data.

use coefficients;
use opltypes::*;
use std::cmp;

use crate::opldb::static_cache::NonSortedNonUnique;
use crate::opldb::static_cache::PossiblyOwnedNonSortedNonUnique;
use crate::opldb::static_cache::PossiblyOwnedSortedUnique;
use crate::opldb::static_cache::SortedUnique;
use crate::opldb::{Entry, Meet, OplDb};
use crate::pages::selection::*;

/// Whether an `Entry` should be part of `BySquat` rankings and records.
#[inline]
pub fn filter_squat(entry: &Entry) -> bool {
    entry.highest_squatkg() > WeightKg::from_i32(0) && !entry.place.is_dq()
}

/// Whether an `Entry` should be part of `ByBench` rankings and records.
#[inline]
pub fn filter_bench(entry: &Entry) -> bool {
    entry.highest_benchkg() > WeightKg::from_i32(0) && !entry.place.is_dq()
}

/// Whether an `Entry` should be part of `ByDeadlift` rankings and records.
#[inline]
pub fn filter_deadlift(entry: &Entry) -> bool {
    entry.highest_deadliftkg() > WeightKg::from_i32(0) && !entry.place.is_dq()
}

/// Whether an `Entry` should be part of `ByTotal` rankings and records.
#[inline]
pub fn filter_total(entry: &Entry) -> bool {
    // TotalKg is defined to be zero if DQ.
    entry.totalkg > WeightKg::from_i32(0)
}

/// Whether an `Entry` should be part of `ByMcCulloch` rankings and records.
#[inline]
pub fn filter_mcculloch(entry: &Entry) -> bool {
    // McCulloch points are defined to be zero if DQ.
    entry.mcculloch > Points::from_i32(0)
}

/// Whether an `Entry` should be part of `ByWilks` rankings and records.
#[inline]
pub fn filter_wilks(entry: &Entry) -> bool {
    // Wilks is defined to be zero if DQ.
    entry.wilks > Points::from_i32(0)
}

/// Whether an `Entry` should be part of `ByGlossbrenner` rankings and records.
#[inline]
pub fn filter_glossbrenner(entry: &Entry) -> bool {
    // Glossbrenner is defined to be zero if DQ.
    entry.glossbrenner > Points::from_i32(0)
}

/// Whether an `Entry` should be part of `ByIPFPoints` rankings and records.
#[inline]
pub fn filter_ipfpoints(entry: &Entry) -> bool {
    // IPF Points are defined to be zero if DQ.
    entry.ipfpoints > Points::from_i32(0)
}

/// Defines an `Ordering` of Entries by Squat.
#[inline]
pub fn cmp_squat(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by SquatKg, higher first.
    a.highest_squatkg()
        .cmp(&b.highest_squatkg())
        .reverse()
        // If equal, sort by Date, earlier first.
        .then(
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date),
        )
        // If equal, sort by Bodyweight, lower first.
        .then(a.bodyweightkg.cmp(&b.bodyweightkg))
        // If for the same lifter on the same day, prefer Entry with largest Total.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by Bench.
#[inline]
pub fn cmp_bench(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by BenchKg, higher first.
    a.highest_benchkg()
        .cmp(&b.highest_benchkg())
        .reverse()
        // If equal, sort by Date, earlier first.
        .then(
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date),
        )
        // If equal, sort by Bodyweight, lower first.
        .then(a.bodyweightkg.cmp(&b.bodyweightkg))
        // If for the same lifter on the same day, prefer Entry with largest Total.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by Deadlift.
#[inline]
pub fn cmp_deadlift(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by DeadliftKg, higher first.
    a.highest_deadliftkg()
        .cmp(&b.highest_deadliftkg())
        .reverse()
        // If equal, sort by Date, earlier first.
        .then(
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date),
        )
        // If equal, sort by Bodyweight, lower first.
        .then(a.bodyweightkg.cmp(&b.bodyweightkg))
        // If for the same lifter on the same day, prefer Entry with largest Total.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by Total.
#[inline]
pub fn cmp_total(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by TotalKg, higher first.
    a.totalkg
        .cmp(&b.totalkg)
        .reverse()
        // If equal, sort by Date, earlier first.
        .then(
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date),
        )
        // If equal, sort by Bodyweight, lower first.
        .then(a.bodyweightkg.cmp(&b.bodyweightkg))
}

/// Defines an `Ordering` of Entries by McCulloch points.
#[inline]
pub fn cmp_mcculloch(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by McCulloch, higher first.
    a.mcculloch
        .cmp(&b.mcculloch)
        .reverse()
        // If equal, sort by Date, earlier first.
        .then(
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date),
        )
        // If that's equal too, sort by Total, highest first.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by Wilks.
#[inline]
pub fn cmp_wilks(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by Wilks, higher first.
    a.wilks
        .cmp(&b.wilks)
        .reverse()
        // If equal, sort by Date, earlier first.
        .then(
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date),
        )
        // If that's equal too, sort by Total, highest first.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by Glossbrenner.
#[inline]
pub fn cmp_glossbrenner(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by Glossbrenner, higher first.
    a.glossbrenner
        .cmp(&b.glossbrenner)
        .reverse()
        // If equal, sort by Date, earlier first.
        .then(
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date),
        )
        // If that's equal too, sort by Total, highest first.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by IPF Points.
#[inline]
pub fn cmp_ipfpoints(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by IPF Points, higher first.
    a.ipfpoints
        .cmp(&b.ipfpoints)
        .reverse()
        // If equal, sort by Date, earlier first.
        .then(
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date),
        )
        // If that's equal too, sort by Total, highest first.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by NASA Points.
///
/// Because NASA points aren't stored on the Entry, they are recalculated
/// each comparison. The computation is not particularly expensive,
/// but does involve floating-point division.
#[inline]
pub fn cmp_nasa(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    let a_points = coefficients::nasa(a.bodyweightkg, a.totalkg);
    let b_points = coefficients::nasa(b.bodyweightkg, b.totalkg);

    // First sort by NASA Points, higher first.
    a_points
        .cmp(&b_points)
        .reverse()
        // If equal, sort by Date, earlier first.
        .then(
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date),
        )
        // If that's equal too, sort by Total, highest first.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by Reshel points.
///
/// Because Reshel points aren't stored on the Entry, they are recalculated
/// each comparison. The computation is not particularly expensive,
/// but does involve powf().
#[inline]
pub fn cmp_reshel(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    let a_points = coefficients::reshel(a.sex, a.bodyweightkg, a.totalkg);
    let b_points = coefficients::reshel(b.sex, b.bodyweightkg, b.totalkg);

    // First sort by Reshel points, higher first.
    a_points
        .cmp(&b_points)
        .reverse()
        // If equal, sort by Date, earlier first.
        .then(
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date),
        )
        // If that's equal too, sort by Total, highest first.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by AH (Haleczko) points.
///
/// Because AH points aren't stored on the Entry, they are recalculated
/// each comparison. The computation is not particularly expensive,
/// but does involve powf().
#[inline]
pub fn cmp_ah(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    let a_points = coefficients::ah(a.sex, a.bodyweightkg, a.totalkg);
    let b_points = coefficients::ah(b.sex, b.bodyweightkg, b.totalkg);

    // First sort by AH points, higher first.
    a_points
        .cmp(&b_points)
        .reverse()
        // If equal, sort by Date, earlier first.
        .then(
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date),
        )
        // If that's equal too, sort by Total, highest first.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Gets a list of all entry indices matching the given selection.
pub fn get_entry_indices_for<'db>(
    selection: &Selection,
    opldb: &'db OplDb,
) -> PossiblyOwnedNonSortedNonUnique<'db> {
    let cache = opldb.get_static_cache();

    // Use the NonSortedNonUnique cached data.
    let equipment: &NonSortedNonUnique = match selection.equipment {
        EquipmentSelection::Raw => &cache.log_linear_time.raw,
        EquipmentSelection::Wraps => &cache.log_linear_time.wraps,
        EquipmentSelection::RawAndWraps => &cache.log_linear_time.raw_wraps,
        EquipmentSelection::Single => &cache.log_linear_time.single,
        EquipmentSelection::Multi => &cache.log_linear_time.multi,
    };
    let mut cur = PossiblyOwnedNonSortedNonUnique::Borrowed(equipment);

    // Apply the Sex filter.
    cur = match selection.sex {
        SexSelection::AllSexes => cur,
        SexSelection::Men => PossiblyOwnedNonSortedNonUnique::Owned(
            cur.intersect(&cache.log_linear_time.male),
        ),
        SexSelection::Women => PossiblyOwnedNonSortedNonUnique::Owned(
            cur.intersect(&cache.log_linear_time.female),
        ),
    };

    // Apply the Year filter.
    cur = match selection.year {
        YearSelection::AllYears => cur,
        YearSelection::Year2019 => PossiblyOwnedNonSortedNonUnique::Owned(
            cur.intersect(&cache.log_linear_time.year2019),
        ),
        YearSelection::Year2018 => PossiblyOwnedNonSortedNonUnique::Owned(
            cur.intersect(&cache.log_linear_time.year2018),
        ),
        YearSelection::Year2017 => PossiblyOwnedNonSortedNonUnique::Owned(
            cur.intersect(&cache.log_linear_time.year2017),
        ),
        YearSelection::Year2016 => PossiblyOwnedNonSortedNonUnique::Owned(
            cur.intersect(&cache.log_linear_time.year2016),
        ),
        YearSelection::Year2015 => PossiblyOwnedNonSortedNonUnique::Owned(
            cur.intersect(&cache.log_linear_time.year2015),
        ),
        YearSelection::Year2014 => PossiblyOwnedNonSortedNonUnique::Owned(
            cur.intersect(&cache.log_linear_time.year2014),
        ),
        _ => {
            let year = selection.year.as_u32().unwrap(); // Safe if not AllYears.
            let filter = NonSortedNonUnique(
                cur.0
                    .iter()
                    .filter_map(|&i| {
                        match opldb.get_meet(opldb.get_entry(i).meet_id).date.year()
                            == year
                        {
                            true => Some(i),
                            false => None,
                        }
                    })
                    .collect(),
            );
            PossiblyOwnedNonSortedNonUnique::Owned(filter)
        }
    };

    // Filter by federation manually.
    if selection.federation != FederationSelection::AllFederations {
        if let FederationSelection::One(fed) = selection.federation {
            let filter = NonSortedNonUnique(
                cur.0
                    .iter()
                    .filter_map(|&i| {
                        match opldb.get_meet(opldb.get_entry(i).meet_id).federation == fed
                        {
                            true => Some(i),
                            false => None,
                        }
                    })
                    .collect(),
            );
            cur = PossiblyOwnedNonSortedNonUnique::Owned(filter);
        } else if let FederationSelection::Meta(metafed) = selection.federation {
            let meets = opldb.get_meets();
            let filter = NonSortedNonUnique(
                cur.0
                    .iter()
                    .filter_map(|&i| match metafed.contains(opldb.get_entry(i), meets) {
                        true => Some(i),
                        false => None,
                    })
                    .collect(),
            );
            cur = PossiblyOwnedNonSortedNonUnique::Owned(filter);
        }
    }

    // Filter by AgeClass manually.
    if selection.ageclass != AgeClassSelection::AllAges {
        let filter = NonSortedNonUnique(
            cur.0
                .iter()
                .filter_map(|&i| {
                    use AgeClass::*;
                    use AgeClassSelection::*;

                    let class = opldb.get_entry(i).ageclass;
                    let byclass = opldb.get_entry(i).birthyearclass;

                    let matches: bool = match selection.ageclass {
                        AllAges => true,

                        // Age-based classes.
                        Youth512 => class == Class5_12,
                        Teenage1315 => class == Class13_15,
                        Teenage1617 => class == Class16_17,
                        Teenage1819 => class == Class18_19,
                        Juniors2023 => class == Class20_23,
                        Seniors2434 => class == Class24_34,
                        Submasters3539 => class == Class35_39,
                        Masters4049 => class == Class40_44 || class == Class45_49,
                        Masters5059 => class == Class50_54 || class == Class55_59,
                        Masters6069 => class == Class60_64 || class == Class65_69,
                        Masters7079 => class == Class70_74 || class == Class75_79,
                        Masters4044 => class == Class40_44,
                        Masters4549 => class == Class45_49,
                        Masters5054 => class == Class50_54,
                        Masters5559 => class == Class55_59,
                        Masters6064 => class == Class60_64,
                        Masters6569 => class == Class65_69,
                        Masters7074 => class == Class70_74,
                        Masters7579 => class == Class75_79,
                        MastersOver80 => class == Class80_999,

                        // BirthYear-based classes.
                        SubJuniorsY14Y18 => byclass == BirthYearClass::ClassY14Y18,
                        JuniorsY19Y23 => byclass == BirthYearClass::ClassY19Y23,
                        SeniorsY24Y39 => byclass == BirthYearClass::ClassY24Y39,
                        MastersY40Y49 => byclass == BirthYearClass::ClassY40Y49,
                        MastersY50Y59 => byclass == BirthYearClass::ClassY50Y59,
                        MastersY60Y69 => byclass == BirthYearClass::ClassY60Y69,
                        MastersOverY70 => byclass == BirthYearClass::ClassY70Y999,
                    };
                    if matches {
                        Option::Some(i)
                    } else {
                        Option::None
                    }
                })
                .collect(),
        );

        cur = PossiblyOwnedNonSortedNonUnique::Owned(filter);
    }

    // Filter by event manually.
    if selection.event != EventSelection::AllEvents {
        let filter = NonSortedNonUnique(
            cur.0
                .iter()
                .filter_map(|&i| {
                    let ev = opldb.get_entry(i).event;
                    let matches: bool = match selection.event {
                        EventSelection::AllEvents => true,
                        EventSelection::FullPower => ev.is_full_power(),
                        EventSelection::PushPull => ev.is_push_pull(),
                        EventSelection::SquatOnly => ev.is_squat_only(),
                        EventSelection::BenchOnly => ev.is_bench_only(),
                        EventSelection::DeadliftOnly => ev.is_deadlift_only(),
                    };
                    if matches {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect(),
        );

        cur = PossiblyOwnedNonSortedNonUnique::Owned(filter);
    }

    // Filter by weight class manually.
    if selection.weightclasses != WeightClassSelection::AllClasses {
        let (lower, upper) = selection.weightclasses.to_bounds();

        let filter = NonSortedNonUnique(
            cur.0
                .iter()
                .filter_map(|&i| {
                    let e = opldb.get_entry(i);

                    // Handle cases with explicit bodyweight.
                    if e.bodyweightkg > lower && e.bodyweightkg <= upper {
                        return Some(i);
                    }

                    // Handle SHW classes with unspecified bodyweight.
                    if upper == WeightKg::max_value() {
                        if let WeightClassKg::Over(over) = e.weightclasskg {
                            if over >= lower {
                                return Some(i);
                            }
                        }
                    }

                    None
                })
                .collect(),
        );

        cur = PossiblyOwnedNonSortedNonUnique::Owned(filter);
    }

    cur
}

/// Gets a full sorted list for the given selection.
///
/// In almost every case it's not necessary to generate the full list,
/// but doing so can be useful for debugging.
pub fn get_full_sorted_uniqued<'db>(
    selection: &Selection,
    opldb: &'db OplDb,
) -> PossiblyOwnedSortedUnique<'db> {
    let cache = opldb.get_static_cache();

    // First, try to use the constant-time cache.
    if selection.federation == FederationSelection::AllFederations
        && selection.weightclasses == WeightClassSelection::AllClasses
        && selection.year == YearSelection::AllYears
        && selection.ageclass == AgeClassSelection::AllAges
        && selection.event == EventSelection::AllEvents
    {
        let by_sort = match selection.sort {
            SortSelection::BySquat => &cache.constant_time.squat,
            SortSelection::ByBench => &cache.constant_time.bench,
            SortSelection::ByDeadlift => &cache.constant_time.deadlift,
            SortSelection::ByTotal => &cache.constant_time.total,
            SortSelection::ByGlossbrenner => &cache.constant_time.glossbrenner,
            SortSelection::ByIPFPoints => &cache.constant_time.ipfpoints,
            SortSelection::ByMcCulloch => &cache.constant_time.mcculloch,
            SortSelection::ByWilks => &cache.constant_time.wilks,
        };

        let sorted_uniqued = match selection.equipment {
            EquipmentSelection::Raw => &by_sort.raw,
            EquipmentSelection::Wraps => &by_sort.wraps,
            EquipmentSelection::RawAndWraps => &by_sort.raw_wraps,
            EquipmentSelection::Single => &by_sort.single,
            EquipmentSelection::Multi => &by_sort.multi,
        };

        // Since each lifter is only one sex, sex selections
        // can just be an O(n) filter.
        if selection.sex != SexSelection::AllSexes {
            return PossiblyOwnedSortedUnique::Owned(SortedUnique(
                sorted_uniqued
                    .0
                    .iter()
                    .filter_map(|&n| {
                        let sex = opldb.get_entry(n).sex;
                        match (selection.sex == SexSelection::Men && sex == Sex::M)
                            || (selection.sex == SexSelection::Women && sex == Sex::F)
                        {
                            true => Some(n),
                            false => None,
                        }
                    })
                    .collect(),
            ));
        }

        return PossiblyOwnedSortedUnique::Borrowed(sorted_uniqued);
    }

    // If the ConstantTime cache fails, use the NonSortedNonUnique cache data.
    let cur = get_entry_indices_for(selection, opldb);

    let entries = opldb.get_entries();
    let meets = opldb.get_meets();

    // TODO: Common out sort code with ConstantTimeCache::new()
    PossiblyOwnedSortedUnique::Owned(match selection.sort {
        SortSelection::BySquat => {
            cur.sort_and_unique_by(&entries, &meets, cmp_squat, filter_squat)
        }
        SortSelection::ByBench => {
            cur.sort_and_unique_by(&entries, &meets, cmp_bench, filter_bench)
        }
        SortSelection::ByDeadlift => {
            cur.sort_and_unique_by(&entries, &meets, cmp_deadlift, filter_deadlift)
        }
        SortSelection::ByTotal => {
            cur.sort_and_unique_by(&entries, &meets, cmp_total, filter_total)
        }
        SortSelection::ByGlossbrenner => cur.sort_and_unique_by(
            &entries,
            &meets,
            cmp_glossbrenner,
            filter_glossbrenner,
        ),
        SortSelection::ByIPFPoints => {
            cur.sort_and_unique_by(&entries, &meets, cmp_ipfpoints, filter_ipfpoints)
        }
        SortSelection::ByMcCulloch => {
            cur.sort_and_unique_by(&entries, &meets, cmp_mcculloch, filter_mcculloch)
        }
        SortSelection::ByWilks => {
            cur.sort_and_unique_by(&entries, &meets, cmp_wilks, filter_wilks)
        }
    })
}
