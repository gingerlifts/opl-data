//! Precalculated cache of data, such as sorts.

use itertools::Itertools;
use std::cmp::Ordering;
use std::ops::Deref;

use opldb::fields::*;
use opldb::{Entry, Meet, OplDb};
use pages::selection::*;

/// List of indices into the opldb.entries vector,
/// in no particular order, but such that entries from the same
/// lifter are next to each other (sorted by LifterID).
///
/// This is useful to get `O(n log n)` lookup, which allows for
/// performing a uniqueness operation without constructing
/// a HashMap.
///
/// Because it's non-sorted, that also means that there doesn't
/// need to be a version of the data stored for each way in
/// which the data can be sorted, so there's memory savings.
#[derive(Clone, Debug, PartialEq)]
pub struct NonSortedNonUnique(pub Vec<u32>);

/// List of indices into the opldb.entries vector,
/// in some sorted order, but with each lifter potentially
/// occurring multiple times.
///
/// This is useful to get `O(n)` lookup, since it stores
/// the filter/sort algorithm in an intermediate output,
/// where further filtering and uniqueness can be applied.
pub struct SortedNonUnique(pub Vec<u32>);

/// List of indices into the opldb.entries vector,
/// in some sorted order, with each lifter occurring at
/// most once.
///
/// This is useful to get `O(1)` lookup, since it stores
/// the filter/sort/unique algorithm in its final output.
pub struct SortedUnique(pub Vec<u32>);

// TODO: Can we templatize these PossiblyOwned types?
/// Allows remembering whether or not a returned SortedUnique is to be
/// deallocated.
pub enum PossiblyOwnedNonSortedNonUnique<'db> {
    Borrowed(&'db NonSortedNonUnique),
    Owned(NonSortedNonUnique),
}

impl<'db> Deref for PossiblyOwnedNonSortedNonUnique<'db> {
    type Target = NonSortedNonUnique;

    fn deref(&self) -> &NonSortedNonUnique {
        match &self {
            PossiblyOwnedNonSortedNonUnique::Borrowed(x) => x,
            PossiblyOwnedNonSortedNonUnique::Owned(x) => &x,
        }
    }
}

/// Allows remembering whether or not a returned SortedUnique is to be
/// deallocated.
pub enum PossiblyOwnedSortedUnique<'db> {
    Borrowed(&'db SortedUnique),
    Owned(SortedUnique),
}

impl<'db> Deref for PossiblyOwnedSortedUnique<'db> {
    type Target = SortedUnique;

    fn deref(&self) -> &SortedUnique {
        match &self {
            PossiblyOwnedSortedUnique::Borrowed(x) => x,
            PossiblyOwnedSortedUnique::Owned(x) => &x,
        }
    }
}

impl NonSortedNonUnique {
    /// Unions the indices from both source inputs.
    pub fn union(&self, other: &NonSortedNonUnique) -> NonSortedNonUnique {
        debug_assert!(self.maintains_invariants());
        debug_assert!(other.maintains_invariants());

        // March and add the least element to the list.
        let mut acc = Vec::<u32>::with_capacity(self.0.len().max(other.0.len()));

        let mut self_index = 0;
        let mut other_index = 0;

        while self_index < self.0.len() && other_index < other.0.len() {
            let a = self.0[self_index];
            let b = other.0[other_index];

            if a == b {
                acc.push(a);
                self_index += 1;
                other_index += 1;
            } else if a < b {
                acc.push(a);
                self_index += 1;
            } else {
                acc.push(b);
                other_index += 1;
            }
        }

        // One of the lists is depleted.
        // Accumulate what remains of the other list.
        // FIXME: Does this re-iterate over the iterator again?
        for &n in self.0.iter().skip(self_index) {
            acc.push(n);
        }
        for &n in other.0.iter().skip(other_index) {
            acc.push(n);
        }

        NonSortedNonUnique(acc)
    }

    /// Intersects the indices from both source inputs.
    pub fn intersect(&self, other: &NonSortedNonUnique) -> NonSortedNonUnique {
        debug_assert!(self.maintains_invariants());
        debug_assert!(other.maintains_invariants());

        // March and matching elements to the list.
        let mut acc = Vec::<u32>::new();

        if self.0.len() == 0 || other.0.len() == 0 {
            return NonSortedNonUnique(acc);
        }

        let mut self_index = 0;
        let mut other_index = 0;

        let mut a = self.0[self_index];
        let mut b = other.0[other_index];

        loop {
            if a == b {
                acc.push(a);
                self_index += 1;
                other_index += 1;
                if self_index == self.0.len() || other_index == other.0.len() {
                    break;
                }
                a = self.0[self_index];
                b = other.0[other_index];
            } else if a < b {
                self_index += 1;
                if self_index == self.0.len() {
                    break;
                }
                a = self.0[self_index];
            } else {
                other_index += 1;
                if other_index == other.0.len() {
                    break;
                }
                b = other.0[other_index];
            }
        }

        NonSortedNonUnique(acc)
    }

    /// Sorts and uniques the data with reference to a comparator.
    pub fn sort_and_unique_by<F>(&self, opldb: &OplDb, compare: F) -> SortedUnique
    where
        F: Fn(u32, u32) -> Ordering,
    {
        debug_assert!(self.maintains_invariants());

        // First, group contiguous entries by lifter_id, so only the best
        // entry for each lifter is counted.
        // The group_by() operation is lazy and does not perform any action yet.
        let groups = self
            .0
            .iter()
            .group_by(|idx| opldb.get_entry(**idx).lifter_id);

        // Perform the grouping operation, generating a new vector.
        let mut list: Vec<u32> = groups
            .into_iter()
            .map(|(_key, group)| *group.min_by(|&x, &y| compare(*x, *y)).unwrap())
            .collect();

        // Sort max-first.
        // Stable sorting is used since it benchmarks faster than unstable.
        list.sort_by(|&x, &y| compare(x, y));

        SortedUnique(list)
    }

    // TODO -- using this method takes 32ms instead of 46ms, quite a savings!
    // Apparently the indirection overhead is pretty high, and it's much faster
    // to just make a sort method directly for each of the sort options.
    // Alas, JS perf is actually better here due to JITs.
    pub fn sort_and_unique_by_wilks(&self, opldb: &OplDb) -> SortedUnique {
        debug_assert!(self.maintains_invariants());

        // First, group contiguous entries by lifter_id, so only the best
        // entry for each lifter is counted.
        // The group_by() operation is lazy and does not perform any action yet.
        let groups = self
            .0
            .iter()
            .group_by(|idx| opldb.get_entry(**idx).lifter_id);

        // Perform the grouping operation, generating a new vector.
        let mut list: Vec<u32> = groups
            .into_iter()
            .map(|(_key, group)| {
                *group
                    .max_by(|&x, &y| {
                        opldb.get_entry(*x).wilks.cmp(&opldb.get_entry(*y).wilks)
                    })
                    .unwrap()
            })
            .collect();

        // Sort max-first.
        // Stable sorting is used since it benchmarks faster than unstable.
        list.sort_by(|&x, &y| {
            opldb
                .get_entry(x)
                .wilks
                .cmp(&opldb.get_entry(y).wilks)
                .reverse()
        });

        SortedUnique(list)
    }

    /// Tests that the list is monotonically increasing.
    pub fn maintains_invariants(&self) -> bool {
        if self.0.len() == 0 {
            return true;
        }

        let mut prev = self.0[0];
        for &i in self.0.iter().skip(1) {
            if prev >= i {
                return false;
            }
            prev = i;
        }
        return true;
    }
}

/// Owning structure of all precomputed data.
pub struct StaticCache {
    pub constant_time: ConstantTimeCache,
    pub linear_time: LinearTimeCache,
    pub log_linear_time: LogLinearTimeCache,
}

impl StaticCache {
    pub fn new(meets: &Vec<Meet>, entries: &Vec<Entry>) -> StaticCache {
        let loglin = LogLinearTimeCache::new(meets, entries);

        StaticCache {
            constant_time: ConstantTimeCache::new(&loglin, meets, entries),
            linear_time: LinearTimeCache::new(),
            log_linear_time: loglin,
        }
    }

    /// Gets a full sorted list for the given selection.
    ///
    /// In almost every case it's not necessary to generate the full list,
    /// but doing so can be useful for debugging.
    pub fn get_full_sorted_uniqued<'db>(
        &'db self,
        selection: &Selection,
        opldb: &'db OplDb,
    ) -> PossiblyOwnedSortedUnique<'db> {
        // First, try to use the constant-time cache.
        if selection.federation == FederationSelection::AllFederations
            && selection.weightclasses == WeightClassSelection::AllClasses
            && selection.year == YearSelection::AllYears
        {
            let by_sort = match selection.sort {
                SortSelection::BySquat => &self.constant_time.squat,
                SortSelection::ByBench => &self.constant_time.bench,
                SortSelection::ByDeadlift => &self.constant_time.deadlift,
                SortSelection::ByTotal => &self.constant_time.total,
                SortSelection::ByWilks => &self.constant_time.wilks,
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

        // If the constant-time cache fails, generate a new list
        // using the NonSortedNonUnique data.
        let equipment: &NonSortedNonUnique = match selection.equipment {
            EquipmentSelection::Raw => &self.log_linear_time.raw,
            EquipmentSelection::Wraps => &self.log_linear_time.wraps,
            EquipmentSelection::RawAndWraps => &self.log_linear_time.raw_wraps,
            EquipmentSelection::Single => &self.log_linear_time.single,
            EquipmentSelection::Multi => &self.log_linear_time.multi,
        };
        let mut cur = PossiblyOwnedNonSortedNonUnique::Borrowed(equipment);

        // Apply the Sex filter.
        cur = match selection.sex {
            SexSelection::AllSexes => cur,
            SexSelection::Men => PossiblyOwnedNonSortedNonUnique::Owned(
                cur.intersect(&self.log_linear_time.male),
            ),
            SexSelection::Women => PossiblyOwnedNonSortedNonUnique::Owned(
                cur.intersect(&self.log_linear_time.female),
            ),
        };

        // Apply the Year filter.
        cur = match selection.year {
            YearSelection::AllYears => cur,
            YearSelection::Year2018 => PossiblyOwnedNonSortedNonUnique::Owned(
                cur.intersect(&self.log_linear_time.year2018),
            ),
            YearSelection::Year2017 => PossiblyOwnedNonSortedNonUnique::Owned(
                cur.intersect(&self.log_linear_time.year2017),
            ),
            YearSelection::Year2016 => PossiblyOwnedNonSortedNonUnique::Owned(
                cur.intersect(&self.log_linear_time.year2016),
            ),
            YearSelection::Year2015 => PossiblyOwnedNonSortedNonUnique::Owned(
                cur.intersect(&self.log_linear_time.year2015),
            ),
            YearSelection::Year2014 => PossiblyOwnedNonSortedNonUnique::Owned(
                cur.intersect(&self.log_linear_time.year2014),
            ),
        };

        // Filter by federation manually.
        if selection.federation != FederationSelection::AllFederations {
            if let FederationSelection::One(fed) = selection.federation {
                let filter = NonSortedNonUnique(
                    cur.0
                        .iter()
                        .filter_map(|&i| {
                            match opldb.get_meet(opldb.get_entry(i).meet_id).federation
                                == fed
                            {
                                true => Some(i),
                                false => None,
                            }
                        })
                        .collect(),
                );
                cur = PossiblyOwnedNonSortedNonUnique::Owned(filter);
            } else if let FederationSelection::Meta(metafed) = selection.federation {
                let filter = NonSortedNonUnique(
                    cur.0
                        .iter()
                        .filter_map(|&i| {
                            match metafed.contains(opldb.get_entry(i), &opldb) {
                                true => Some(i),
                                false => None,
                            }
                        })
                        .collect(),
                );
                cur = PossiblyOwnedNonSortedNonUnique::Owned(filter);
            }
        }

        // Filter by weight class manually.
        if selection.weightclasses != WeightClassSelection::AllClasses {
            let (lower, upper) = selection.weightclasses.to_bounds();

            let filter = NonSortedNonUnique(
                cur.0
                    .iter()
                    .filter_map(|&i| {
                        match opldb.get_entry(i).bodyweightkg > lower
                            && opldb.get_entry(i).bodyweightkg <= upper
                        {
                            true => Some(i),
                            false => None,
                        }
                    })
                    .collect(),
            );

            cur = PossiblyOwnedNonSortedNonUnique::Owned(filter);
        }

        // Only show entries with non-empty values in the sort category.
        let cur = match selection.sort {
            SortSelection::BySquat => {
                PossiblyOwnedNonSortedNonUnique::Owned(NonSortedNonUnique(
                    cur.0
                        .iter()
                        .filter_map(|&i| {
                            if opldb.get_entry(i).highest_squatkg() > WeightKg(0) {
                                Some(i)
                            } else {
                                None
                            }
                        })
                        .collect(),
                ))
            }
            SortSelection::ByBench => {
                PossiblyOwnedNonSortedNonUnique::Owned(NonSortedNonUnique(
                    cur.0
                        .iter()
                        .filter_map(|&i| {
                            if opldb.get_entry(i).highest_benchkg() > WeightKg(0) {
                                Some(i)
                            } else {
                                None
                            }
                        })
                        .collect(),
                ))
            }
            SortSelection::ByDeadlift => {
                PossiblyOwnedNonSortedNonUnique::Owned(NonSortedNonUnique(
                    cur.0
                        .iter()
                        .filter_map(|&i| {
                            if opldb.get_entry(i).highest_deadliftkg() > WeightKg(0) {
                                Some(i)
                            } else {
                                None
                            }
                        })
                        .collect(),
                ))
            }
            // Nothing needed for total: all entries without totals are DQ'd
            // and already filtered-out.
            SortSelection::ByTotal => cur,
            SortSelection::ByWilks => {
                PossiblyOwnedNonSortedNonUnique::Owned(NonSortedNonUnique(
                    cur.0
                        .iter()
                        .filter_map(|&i| {
                            if opldb.get_entry(i).wilks > Points(0) {
                                Some(i)
                            } else {
                                None
                            }
                        })
                        .collect(),
                ))
            }
        };

        let entries = opldb.get_entries();
        let meets = opldb.get_meets();

        // TODO: Common out sort code with ConstantTimeCache::new()
        PossiblyOwnedSortedUnique::Owned(match selection.sort {
            SortSelection::BySquat => cur.sort_and_unique_by(&opldb, |x: u32, y: u32| {
                let x = x as usize;
                let y = y as usize;

                entries[x]
                    .highest_squatkg()
                    .cmp(&entries[y].highest_squatkg())
                    .reverse()
                    .then(entries[x].bodyweightkg.cmp(&entries[y].bodyweightkg))
                    .then(
                        meets[entries[x].meet_id as usize]
                            .date
                            .cmp(&meets[entries[y].meet_id as usize].date),
                    )
            }),
            SortSelection::ByBench => cur.sort_and_unique_by(&opldb, |x: u32, y: u32| {
                let x = x as usize;
                let y = y as usize;

                entries[x]
                    .highest_benchkg()
                    .cmp(&entries[y].highest_benchkg())
                    .reverse()
                    .then(entries[x].bodyweightkg.cmp(&entries[y].bodyweightkg))
                    .then(
                        meets[entries[x].meet_id as usize]
                            .date
                            .cmp(&meets[entries[y].meet_id as usize].date),
                    )
            }),
            SortSelection::ByDeadlift => {
                cur.sort_and_unique_by(&opldb, |x: u32, y: u32| {
                    let x = x as usize;
                    let y = y as usize;

                    entries[x]
                        .highest_deadliftkg()
                        .cmp(&entries[y].highest_deadliftkg())
                        .reverse()
                        .then(entries[x].bodyweightkg.cmp(&entries[y].bodyweightkg))
                        .then(
                            meets[entries[x].meet_id as usize]
                                .date
                                .cmp(&meets[entries[y].meet_id as usize].date),
                        )
                })
            }
            SortSelection::ByTotal => cur.sort_and_unique_by(&opldb, |x: u32, y: u32| {
                let x = x as usize;
                let y = y as usize;

                entries[x]
                    .totalkg
                    .cmp(&entries[y].totalkg)
                    .reverse()
                    .then(entries[x].bodyweightkg.cmp(&entries[y].bodyweightkg))
                    .then(
                        meets[entries[x].meet_id as usize]
                            .date
                            .cmp(&meets[entries[y].meet_id as usize].date),
                    )
            }),
            SortSelection::ByWilks => cur.sort_and_unique_by_wilks(&opldb),
        })
    }
}

/// Stores all sorts for a given equipment type.
pub struct ConstantTimeBy {
    pub raw: SortedUnique,
    pub wraps: SortedUnique,
    pub raw_wraps: SortedUnique,
    pub single: SortedUnique,
    pub multi: SortedUnique,
}

/// Owning structure of all `O(1)` lookup data.
pub struct ConstantTimeCache {
    // Weight comparisons.
    pub squat: ConstantTimeBy,
    pub bench: ConstantTimeBy,
    pub deadlift: ConstantTimeBy,
    pub total: ConstantTimeBy,

    // Points comparisons.
    pub wilks: ConstantTimeBy,
}

impl ConstantTimeCache {
    /// Sorts and uniques the data with reference to a comparator.
    ///
    /// The comparator should return greatest-first, in sorted order
    /// by however it should show up in the final database.
    ///
    /// TODO: Filter out zero entries (like lifters with no squat for by-squat,
    /// etc.)
    fn sort_and_unique_by<F>(
        idxl: &NonSortedNonUnique,
        entries: &Vec<Entry>,
        compare: F,
    ) -> SortedUnique
    where
        F: Fn(u32, u32) -> Ordering,
    {
        // First, group contiguous entries by lifter_id, so only the best
        // entry for each lifter is counted.
        // The group_by() operation is lazy and does not perform any action yet.
        let groups = idxl
            .0
            .iter()
            .group_by(|idx| entries[**idx as usize].lifter_id);

        // Perform the grouping operation, generating a new vector.
        let mut vec: Vec<u32> = groups
            .into_iter()
            // `min_by()` takes the best entry due to comparator ordering.
            .map(|(_key, group)| *group.min_by(|&x, &y| compare(*x, *y)).unwrap())
            .collect();

        vec.sort_by(|&x, &y| compare(x, y));
        vec.shrink_to_fit();
        SortedUnique(vec)
    }

    pub fn new(
        loglin: &LogLinearTimeCache,
        meets: &Vec<Meet>,
        entries: &Vec<Entry>,
    ) -> ConstantTimeCache {
        let by_squat = |x: u32, y: u32| {
            let x = x as usize;
            let y = y as usize;

            // First sort by SquatKg, highest first.
            entries[x].highest_squatkg().cmp(&entries[y].highest_squatkg()).reverse()
                // If equal, sort by Bodyweight, since this is for rankings.
                // (Records would sort by Date before Bodyweight.)
                .then(entries[x].bodyweightkg.cmp(&entries[y].bodyweightkg))
                // If that's equal too, sort by Date, earliest first.
                .then(meets[entries[x].meet_id as usize].date.cmp(
                        &meets[entries[y].meet_id as usize].date))
        };

        let squat = ConstantTimeBy {
            raw: Self::sort_and_unique_by(&loglin.raw, entries, by_squat),
            wraps: Self::sort_and_unique_by(&loglin.wraps, entries, by_squat),
            raw_wraps: Self::sort_and_unique_by(&loglin.raw_wraps, entries, by_squat),
            single: Self::sort_and_unique_by(&loglin.single, entries, by_squat),
            multi: Self::sort_and_unique_by(&loglin.multi, entries, by_squat),
        };

        let by_bench = |x: u32, y: u32| {
            let x = x as usize;
            let y = y as usize;

            // First sort by SquatKg, highest first.
            entries[x].highest_benchkg().cmp(&entries[y].highest_benchkg()).reverse()
                // If equal, sort by Bodyweight, since this is for rankings.
                // (Records would sort by Date before Bodyweight.)
                .then(entries[x].bodyweightkg.cmp(&entries[y].bodyweightkg))
                // If that's equal too, sort by Date, earliest first.
                .then(meets[entries[x].meet_id as usize].date.cmp(
                        &meets[entries[y].meet_id as usize].date))
        };

        let bench = ConstantTimeBy {
            raw: Self::sort_and_unique_by(&loglin.raw, entries, by_bench),
            wraps: Self::sort_and_unique_by(&loglin.wraps, entries, by_bench),
            raw_wraps: Self::sort_and_unique_by(&loglin.raw_wraps, entries, by_bench),
            single: Self::sort_and_unique_by(&loglin.single, entries, by_bench),
            multi: Self::sort_and_unique_by(&loglin.multi, entries, by_bench),
        };

        let by_deadlift = |x: u32, y: u32| {
            let x = x as usize;
            let y = y as usize;

            // First sort by SquatKg, highest first.
            entries[x].highest_deadliftkg().cmp(
                    &entries[y].highest_deadliftkg()).reverse()
                // If equal, sort by Bodyweight, since this is for rankings.
                // (Records would sort by Date before Bodyweight.)
                .then(entries[x].bodyweightkg.cmp(&entries[y].bodyweightkg))
                // If that's equal too, sort by Date, earliest first.
                .then(meets[entries[x].meet_id as usize].date.cmp(
                        &meets[entries[y].meet_id as usize].date))
        };

        let deadlift = ConstantTimeBy {
            raw: Self::sort_and_unique_by(&loglin.raw, entries, by_deadlift),
            wraps: Self::sort_and_unique_by(&loglin.wraps, entries, by_deadlift),
            raw_wraps: Self::sort_and_unique_by(&loglin.raw_wraps, entries, by_deadlift),
            single: Self::sort_and_unique_by(&loglin.single, entries, by_deadlift),
            multi: Self::sort_and_unique_by(&loglin.multi, entries, by_deadlift),
        };

        let by_total = |x: u32, y: u32| {
            let x = x as usize;
            let y = y as usize;

            // First sort by SquatKg, highest first.
            entries[x].totalkg.cmp(&entries[y].totalkg).reverse()
                // If equal, sort by Bodyweight, since this is for rankings.
                // (Records would sort by Date before Bodyweight.)
                .then(entries[x].bodyweightkg.cmp(&entries[y].bodyweightkg))
                // If that's equal too, sort by Date, earliest first.
                .then(meets[entries[x].meet_id as usize].date.cmp(
                        &meets[entries[y].meet_id as usize].date))
        };

        let total = ConstantTimeBy {
            raw: Self::sort_and_unique_by(&loglin.raw, entries, by_total),
            wraps: Self::sort_and_unique_by(&loglin.wraps, entries, by_total),
            raw_wraps: Self::sort_and_unique_by(&loglin.raw_wraps, entries, by_total),
            single: Self::sort_and_unique_by(&loglin.single, entries, by_total),
            multi: Self::sort_and_unique_by(&loglin.multi, entries, by_total),
        };

        let by_wilks = |x: u32, y: u32| {
            let x = x as usize;
            let y = y as usize;

            // First sort by Wilks, highest first.
            entries[x].wilks.cmp(&entries[y].wilks).reverse()
                // If equal, sort by Date, earliest first.
                .then(meets[entries[x].meet_id as usize].date.cmp(
                        &meets[entries[y].meet_id as usize].date))
                // If that's equal too, sort by Total, highest first.
                .then(entries[x].totalkg.cmp(&entries[y].totalkg))
        };

        let wilks = ConstantTimeBy {
            raw: Self::sort_and_unique_by(&loglin.raw, entries, by_wilks),
            wraps: Self::sort_and_unique_by(&loglin.wraps, entries, by_wilks),
            raw_wraps: Self::sort_and_unique_by(&loglin.raw_wraps, entries, by_wilks),
            single: Self::sort_and_unique_by(&loglin.single, entries, by_wilks),
            multi: Self::sort_and_unique_by(&loglin.multi, entries, by_wilks),
        };

        ConstantTimeCache {
            squat,
            bench,
            deadlift,
            total,
            wilks,
        }
    }
}

/// Owning structure of all `O(n)` lookup data.
pub struct LinearTimeCache {}

impl LinearTimeCache {
    pub fn new() -> LinearTimeCache {
        LinearTimeCache {}
    }
}

/// Owning structure of all `O(n log n)` lookup data.
pub struct LogLinearTimeCache {
    /// List of all non-DQ Raw entry indices by LifterID.
    pub raw: NonSortedNonUnique,
    /// List of all non-DQ Wraps entry incides by LifterID.
    pub wraps: NonSortedNonUnique,
    /// List of all non-DQ Raw+Wraps entry indices by LifterID.
    pub raw_wraps: NonSortedNonUnique,
    /// List of all non-DQ Single-ply entry indices by LifterID.
    pub single: NonSortedNonUnique,
    /// List of all non-DQ Multi-ply entry indices by LifterID.
    pub multi: NonSortedNonUnique,

    /// List of all non-DQ Male entry indices by LifterID.
    pub male: NonSortedNonUnique,
    /// List of all non-DQ Female entry indices by LifterID.
    pub female: NonSortedNonUnique,

    pub year2018: NonSortedNonUnique,
    pub year2017: NonSortedNonUnique,
    pub year2016: NonSortedNonUnique,
    pub year2015: NonSortedNonUnique,
    pub year2014: NonSortedNonUnique,
}

impl LogLinearTimeCache {
    fn filter_entries<F>(entries: &Vec<Entry>, select: F) -> NonSortedNonUnique
    where
        F: Fn(&Entry) -> bool,
    {
        let mut vec = Vec::new();
        for i in 0..entries.len() {
            if select(&entries[i]) {
                vec.push(i as u32);
            }
        }
        vec.shrink_to_fit();
        NonSortedNonUnique(vec)
    }

    pub fn new(meets: &Vec<Meet>, entries: &Vec<Entry>) -> LogLinearTimeCache {
        LogLinearTimeCache {
            raw: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && e.equipment == Equipment::Raw
            }),
            wraps: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && e.equipment == Equipment::Wraps
            }),
            raw_wraps: Self::filter_entries(entries, |e| {
                !e.place.is_dq()
                    && (e.equipment == Equipment::Raw || e.equipment == Equipment::Wraps)
            }),
            single: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && e.equipment == Equipment::Single
            }),
            multi: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && e.equipment == Equipment::Multi
            }),

            male: Self::filter_entries(entries, |e| !e.place.is_dq() && e.sex == Sex::M),
            female: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && e.sex == Sex::F
            }),

            year2018: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && meets[e.meet_id as usize].date.year() == 2018
            }),
            year2017: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && meets[e.meet_id as usize].date.year() == 2017
            }),
            year2016: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && meets[e.meet_id as usize].date.year() == 2016
            }),
            year2015: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && meets[e.meet_id as usize].date.year() == 2015
            }),
            year2014: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && meets[e.meet_id as usize].date.year() == 2014
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect_nonmonotonic() {
        let f = NonSortedNonUnique(vec![1, 2, 3, 5, 4]);
        assert!(!f.maintains_invariants());
        let f = NonSortedNonUnique(vec![1, 2, 3, 4, 5]);
        assert!(f.maintains_invariants());
    }

    #[test]
    fn test_union_basic() {
        let f1 = NonSortedNonUnique(vec![1, 2, 3]);
        assert_eq!(f1.union(&f1), f1);

        let f1 = NonSortedNonUnique(vec![0, 2, 6]);
        let f2 = NonSortedNonUnique(vec![1, 2, 7]);
        let expected = NonSortedNonUnique(vec![0, 1, 2, 6, 7]);
        assert_eq!(f1.union(&f2), expected);
        assert_eq!(f2.union(&f1), expected);
    }

    #[test]
    fn test_union_empty() {
        let empty = NonSortedNonUnique(vec![]);
        assert_eq!(empty.union(&empty), empty);

        let f2 = NonSortedNonUnique(vec![1, 2, 3]);
        assert_eq!(empty.union(&f2), f2);
        assert_eq!(f2.union(&empty), f2);
    }

    #[test]
    fn test_intersect_basic() {
        let f1 = NonSortedNonUnique(vec![1, 2, 3]);
        assert_eq!(f1.intersect(&f1), f1);

        let f1 = NonSortedNonUnique(vec![0, 2, 4, 6, 8]);
        let f2 = NonSortedNonUnique(vec![0, 3, 4, 8, 10, 12]);
        let expected = NonSortedNonUnique(vec![0, 4, 8]);
        assert_eq!(f1.intersect(&f2), expected);
        assert_eq!(f2.intersect(&f1), expected);
    }

    #[test]
    fn test_intersect_empty() {
        let empty = NonSortedNonUnique(vec![]);
        assert_eq!(empty.intersect(&empty), empty);

        let f2 = NonSortedNonUnique(vec![1, 2, 3]);
        assert_eq!(empty.intersect(&f2), empty);
        assert_eq!(f2.intersect(&empty), empty);
    }
}
