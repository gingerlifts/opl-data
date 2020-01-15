//! An in-memory data store for OpenPowerlifting data.
//!
//! Because our data is read-only at runtime, we can lay out data structures
//! better than a "real" database like SQLite3 or PostgreSQL. Additionally,
//! by storing all the data in formats native to Rust, we avoid copy overhead.

use coefficients;
use csv;
use itertools::Itertools;
use opltypes::*;

use std::error::Error;
use std::mem;

// Exports.
pub mod algorithms;
mod metafederation;
pub use self::metafederation::*;
mod static_cache;
pub use self::static_cache::*;

/// The definition of a Lifter in the database.
#[derive(Serialize, Deserialize)]
pub struct Lifter {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "CyrillicName"))]
    pub cyrillic_name: Option<String>,
    #[serde(rename(deserialize = "JapaneseName"))]
    pub japanese_name: Option<String>,
    #[serde(rename(deserialize = "GreekName"))]
    pub greek_name: Option<String>,
    #[serde(rename(deserialize = "Username"))]
    pub username: String,
    #[serde(rename(deserialize = "Instagram"))]
    pub instagram: Option<String>,
    #[serde(rename(deserialize = "VKontakte"))]
    pub vkontakte: Option<String>,
    #[serde(rename(deserialize = "Color"))]
    pub color: Option<String>,
    #[serde(rename(deserialize = "Flair"))]
    pub flair: Option<String>,
}

/// The definition of a Meet in the database.
#[derive(Deserialize)]
pub struct Meet {
    #[serde(rename(deserialize = "MeetPath"))]
    pub path: String,
    #[serde(rename(deserialize = "Federation"))]
    pub federation: Federation,
    #[serde(rename(deserialize = "Date"))]
    pub date: Date,
    #[serde(rename(deserialize = "MeetCountry"))]
    pub country: Country,
    #[serde(rename(deserialize = "MeetState"))]
    pub state: Option<String>,
    #[serde(rename(deserialize = "MeetTown"))]
    pub town: Option<String>,
    #[serde(rename(deserialize = "MeetName"))]
    pub name: String,
    #[serde(rename(deserialize = "RuleSet"))]
    pub ruleset: RuleSet,

    /// Number of unique competitors, by LifterID.
    /// Calculated at load-time.
    #[serde(default)]
    pub num_unique_lifters: u32,
}

/// The definition of an Entry in the database.
#[derive(Debug, Deserialize)]
pub struct Entry {
    #[serde(rename(deserialize = "MeetID"))]
    pub meet_id: u32,
    #[serde(rename(deserialize = "LifterID"))]
    pub lifter_id: u32,
    #[serde(rename(deserialize = "Sex"))]
    pub sex: Sex,
    #[serde(rename(deserialize = "Event"))]
    pub event: Event,
    #[serde(rename(deserialize = "Equipment"))]
    pub equipment: Equipment,
    #[serde(rename(deserialize = "Age"))]
    pub age: Age,
    #[serde(rename(deserialize = "Division"))]
    pub division: Option<String>,
    #[serde(rename(deserialize = "BodyweightKg"))]
    pub bodyweightkg: WeightKg,
    #[serde(rename(deserialize = "WeightClassKg"))]
    pub weightclasskg: WeightClassKg,
    #[serde(rename(deserialize = "Squat1Kg"))]
    pub squat1kg: WeightKg,
    #[serde(rename(deserialize = "Squat2Kg"))]
    pub squat2kg: WeightKg,
    #[serde(rename(deserialize = "Squat3Kg"))]
    pub squat3kg: WeightKg,
    #[serde(rename(deserialize = "Squat4Kg"))]
    pub squat4kg: WeightKg,
    #[serde(rename(deserialize = "Best3SquatKg"))]
    pub best3squatkg: WeightKg,
    #[serde(rename(deserialize = "Bench1Kg"))]
    pub bench1kg: WeightKg,
    #[serde(rename(deserialize = "Bench2Kg"))]
    pub bench2kg: WeightKg,
    #[serde(rename(deserialize = "Bench3Kg"))]
    pub bench3kg: WeightKg,
    #[serde(rename(deserialize = "Bench4Kg"))]
    pub bench4kg: WeightKg,
    #[serde(rename(deserialize = "Best3BenchKg"))]
    pub best3benchkg: WeightKg,
    #[serde(rename(deserialize = "Deadlift1Kg"))]
    pub deadlift1kg: WeightKg,
    #[serde(rename(deserialize = "Deadlift2Kg"))]
    pub deadlift2kg: WeightKg,
    #[serde(rename(deserialize = "Deadlift3Kg"))]
    pub deadlift3kg: WeightKg,
    #[serde(rename(deserialize = "Deadlift4Kg"))]
    pub deadlift4kg: WeightKg,
    #[serde(rename(deserialize = "Best3DeadliftKg"))]
    pub best3deadliftkg: WeightKg,
    #[serde(rename(deserialize = "TotalKg"))]
    pub totalkg: WeightKg,
    #[serde(rename(deserialize = "Place"))]
    pub place: Place,
    #[serde(rename(deserialize = "Wilks"))]
    pub wilks: Points,
    #[serde(rename(deserialize = "McCulloch"))]
    pub mcculloch: Points,
    #[serde(rename(deserialize = "Glossbrenner"))]
    pub glossbrenner: Points,
    #[serde(rename(deserialize = "IPFPoints"))]
    pub ipfpoints: Points,
    #[serde(rename(deserialize = "Dots"))]
    pub dots: Points,
    #[serde(
        rename(deserialize = "Tested"),
        deserialize_with = "deserialize_yes_no"
    )]
    pub tested: bool,
    #[serde(rename(deserialize = "AgeClass"))]
    pub ageclass: AgeClass,
    #[serde(rename(deserialize = "BirthYearClass"))]
    pub birthyearclass: BirthYearClass,
    #[serde(rename(deserialize = "Country"))]
    pub lifter_country: Option<Country>,
    #[serde(rename(deserialize = "State"))]
    pub lifter_state: Option<State>,
}

impl Entry {
    /// Returns max(best3squatkg, squat4kg).
    #[inline]
    pub fn highest_squatkg(&self) -> WeightKg {
        self.best3squatkg.max(self.squat4kg)
    }

    /// Returns max(best3benchkg, bench4kg).
    #[inline]
    pub fn highest_benchkg(&self) -> WeightKg {
        self.best3benchkg.max(self.bench4kg)
    }

    /// Returns max(best3deadliftkg, deadlift4kg).
    #[inline]
    pub fn highest_deadliftkg(&self) -> WeightKg {
        self.best3deadliftkg.max(self.deadlift4kg)
    }

    /// Borrows the Division string.
    #[inline]
    pub fn get_division(&self) -> Option<&str> {
        self.division.as_ref().map(|s| s.as_str())
    }

    /// Calculate points.
    pub fn points(&self, system: PointsSystem, units: WeightUnits) -> Points {
        let sex = self.sex;
        let bw = self.bodyweightkg;
        let total = self.totalkg;

        match system {
            PointsSystem::AH => coefficients::ah(sex, bw, total),
            PointsSystem::Dots => self.dots,
            PointsSystem::Glossbrenner => self.glossbrenner,
            PointsSystem::IPFPoints => self.ipfpoints,
            PointsSystem::McCulloch => self.mcculloch,
            PointsSystem::NASA => coefficients::nasa(bw, total),
            PointsSystem::Reshel => coefficients::reshel(sex, bw, total),
            PointsSystem::SchwartzMalone => coefficients::schwartzmalone(sex, bw, total),
            PointsSystem::Total => self.totalkg.as_type(units).as_points(),
            PointsSystem::Wilks => self.wilks,
        }
    }
}

/// The collection of data stores that constitute the complete dataset.
///
/// The data structure is immutable. To prevent the owner from modifying
/// owned data, the struct contents are private and accessed through getters.
pub struct OplDb {
    /// The LifterID is implicit in the backing vector, as the index.
    ///
    /// The order of the lifters is arbitrary.
    lifters: Vec<Lifter>,

    /// The MeetID is implicit in the backing vector, as the index.
    ///
    /// The order of the meets is arbitrary.
    meets: Vec<Meet>,

    /// The EntryID is implicit in the backing vector, as the index.
    ///
    /// The order of the entries is by increasing lifter_id.
    /// Within the entries of a single lifter_id, the order is arbitrary.
    entries: Vec<Entry>,

    /// Precalculated caches.
    static_cache: StaticCache,
    metafed_cache: MetaFederationCache,
}

/// Reads the `lifters.csv` file into a Vec<Lifter>.
fn import_lifters_csv(file: &str) -> Result<Vec<Lifter>, Box<dyn Error>> {
    let mut vec = Vec::with_capacity(250_000);

    let mut rdr = csv::Reader::from_path(file)?;
    for lifter in rdr.deserialize() {
        let lifter: Lifter = lifter?;
        vec.push(lifter);
    }

    vec.shrink_to_fit();
    Ok(vec)
}

/// Reads the `meet.csv` file into a Vec<Meet>.
fn import_meets_csv(file: &str) -> Result<Vec<Meet>, Box<dyn Error>> {
    let mut vec = Vec::with_capacity(15_000);

    let mut rdr = csv::Reader::from_path(file)?;
    for meet in rdr.deserialize() {
        let meet: Meet = meet?;
        vec.push(meet);
    }

    vec.shrink_to_fit();
    Ok(vec)
}

/// Reads the `entries.csv` file into a Vec<Entry>.
///
/// Also fills in metadata about each Meet.
fn import_entries_csv(
    file: &str,
    meets: &mut Vec<Meet>,
) -> Result<(Vec<Entry>, MetaFederationCache), Box<dyn Error>> {
    let mut vec = Vec::with_capacity(700_000);

    let mut rdr = csv::Reader::from_path(file)?;
    for entry in rdr.deserialize() {
        let entry: Entry = entry?;
        vec.push(entry);
    }

    // Initially, the entries CSV is sorted by meet_id.
    // This ordering can be used to efficiently calculate meet metadata.
    let metafed_cache = MetaFederationCache::make(&meets, &vec);

    // Calculate num_unique_lifters.
    for (meet_id, meet) in meets.iter_mut().enumerate() {
        meet.num_unique_lifters = precompute_num_unique_lifters(&vec, meet_id as u32);
    }

    // Sort the entries database by lifter_id.
    // This invariant allows for extremely efficient lifter-uniqueness
    // filtering without constructing additional data structures.
    vec.sort_unstable_by_key(|e| e.lifter_id);

    vec.shrink_to_fit();
    Ok((vec, metafed_cache))
}

/// Counts how many unique LifterIDs competed in a given meet.
///
/// Assumes that the entries vector is sorted by meet_id --
/// so this is only callable from within `import_entries_csv()`.
fn precompute_num_unique_lifters(entries: &[Entry], meet_id: u32) -> u32 {
    let found_index = entries
        .binary_search_by_key(&meet_id, |e| e.meet_id)
        .unwrap();

    // All entries for a meet are contiguous, so scan linearly to find the first.
    let mut first_index = found_index;
    for index in (0..found_index).rev() {
        if entries[index].meet_id == meet_id {
            first_index = index;
        } else {
            break;
        }
    }

    // Scan to find the last.
    let mut last_index = found_index;
    for (i, entry) in entries.iter().enumerate().skip(found_index) {
        if entry.meet_id == meet_id {
            last_index = i;
        } else {
            break;
        }
    }
    assert!(first_index <= last_index);

    // Gather all the lifter_ids.
    let mut lifter_ids: Vec<u32> = (first_index..last_index + 1)
        .map(|i| entries[i].lifter_id)
        .collect();

    lifter_ids.sort_unstable();
    lifter_ids.into_iter().group_by(|x| *x).into_iter().count() as u32
}

impl OplDb {
    /// Constructs the `OplDb` from CSV files produces by the project
    /// build script.
    pub fn from_csv(
        lifters_csv: &str,
        meets_csv: &str,
        entries_csv: &str,
    ) -> Result<OplDb, Box<dyn Error>> {
        let lifters = import_lifters_csv(lifters_csv)?;
        let mut meets = import_meets_csv(meets_csv)?;
        let (entries, metafed_cache) = import_entries_csv(entries_csv, &mut meets)?;

        let static_cache = StaticCache::new(&lifters, &meets, &entries);

        Ok(OplDb {
            lifters,
            meets,
            entries,
            static_cache,
            metafed_cache,
        })
    }

    /// Returns the size of owned data structures.
    pub fn size_bytes(&self) -> usize {
        // Size of owned vectors.
        let lifters_size = mem::size_of::<Lifter>() * self.lifters.len();
        let meets_size = mem::size_of::<Meet>() * self.meets.len();
        let entries_size = mem::size_of::<Entry>() * self.entries.len();
        let owned_vectors = lifters_size + meets_size + entries_size;

        // Size of owned Strings in those objects.
        let mut owned_strings: usize = 0;
        for lifter in &self.lifters {
            owned_strings += mem::size_of::<String>() + lifter.name.len();
            owned_strings += mem::size_of::<String>() + lifter.username.len();
            if let Some(ref instagram) = lifter.instagram {
                owned_strings += mem::size_of::<String>() + instagram.len();
            }
        }
        for meet in &self.meets {
            owned_strings += mem::size_of::<String>() + meet.path.len();
            if let Some(ref state) = meet.state {
                owned_strings += mem::size_of::<String>() + state.len();
            }
            if let Some(ref town) = meet.town {
                owned_strings += mem::size_of::<String>() + town.len();
            }
            owned_strings += mem::size_of::<String>() + meet.name.len();
        }
        for entry in &self.entries {
            if let Some(ref division) = entry.division {
                owned_strings += mem::size_of::<String>() + division.len();
            }
        }

        mem::size_of::<OplDb>() + owned_vectors + owned_strings
    }

    /// Borrows the lifters vector.
    #[inline]
    pub fn get_lifters(&self) -> &[Lifter] {
        &self.lifters.as_slice()
    }

    /// Borrows the meets vector.
    #[inline]
    pub fn get_meets(&self) -> &[Meet] {
        &self.meets.as_slice()
    }

    /// Borrows the entries vector.
    #[inline]
    pub fn get_entries(&self) -> &[Entry] {
        &self.entries.as_slice()
    }

    /// Borrows a `Lifter` by index.
    #[inline]
    pub fn get_lifter(&self, n: u32) -> &Lifter {
        &self.lifters[n as usize]
    }

    /// Borrows a `Meet` by index.
    #[inline]
    pub fn get_meet(&self, n: u32) -> &Meet {
        &self.meets[n as usize]
    }

    /// Borrows an `Entry` by index.
    #[inline]
    pub fn get_entry(&self, n: u32) -> &Entry {
        &self.entries[n as usize]
    }

    /// Borrows the static cache. It's static!
    #[inline]
    pub fn get_static_cache(&self) -> &StaticCache {
        &self.static_cache
    }

    /// Borrows the MetaFederationCache.
    #[inline]
    pub fn get_metafed_cache(&self) -> &MetaFederationCache {
        &self.metafed_cache
    }

    /// Look up the lifter_id by username.
    pub fn get_lifter_id(&self, username: &str) -> Option<u32> {
        self.static_cache.username_map.get(username).cloned()
    }

    /// Get a list of all lifters that have the same username base,
    /// which doesn't include numbers for disambiguation.
    ///
    /// For example, "johndoe" matches "johndoe" and "johndoe1",
    /// but does not match "johndoenut".
    pub fn get_lifters_under_username(&self, base: &str) -> Vec<u32> {
        let mut acc = vec![];
        for i in 0..self.lifters.len() {
            let username = &self.lifters[i].username;
            if username.starts_with(base) {
                // If the base is shared, the remainder of the string
                // should be empty or a number for disambiguation.
                let (_, remainder) = username.split_at(base.len());
                if remainder.is_empty() || remainder.parse::<u8>().is_ok() {
                    acc.push(i as u32);
                }
            }
        }
        acc
    }

    /// Look up the lifter_id by Name.
    ///
    /// This function exists for compatibility for the old site.
    /// Outside of that, usernames or id numbers should be used.
    pub fn get_lifter_id_by_name(&self, name: &str) -> Option<u32> {
        for i in 0..self.lifters.len() {
            if self.lifters[i].name == name {
                return Some(i as u32);
            }
        }
        None
    }

    /// Look up the meet_id by MeetPath.
    pub fn get_meet_id(&self, meetpath: &str) -> Option<u32> {
        for i in 0..self.meets.len() {
            if self.meets[i].path == meetpath {
                return Some(i as u32);
            }
        }
        None
    }

    /// Returns all entries with the given lifter_id.
    ///
    /// The vector of entries is sorted by lifter_id. This function uses binary
    /// search followed by a bi-directional linear scan.
    ///
    /// Panics if the lifter_id is not found.
    pub fn get_entries_for_lifter<'a>(&'a self, lifter_id: u32) -> Vec<&'a Entry> {
        // Perform a binary search on lifter_id.
        let found_index = self
            .get_entries()
            .binary_search_by_key(&lifter_id, |e| e.lifter_id)
            .unwrap();

        // All entries for a lifter are contiguous, so scan linearly to find the first.
        let mut first_index = found_index;
        for index in (0..found_index).rev() {
            if self.get_entry(index as u32).lifter_id == lifter_id {
                first_index = index;
            } else {
                break;
            }
        }

        // Scan to find the last.
        let mut last_index = found_index;
        for index in found_index..self.get_entries().len() {
            if self.get_entry(index as u32).lifter_id == lifter_id {
                last_index = index;
            } else {
                break;
            }
        }
        assert!(first_index <= last_index);

        // Collect entries between first_index and last_index, inclusive.
        (first_index..last_index + 1)
            .map(|i| self.get_entry(i as u32))
            .collect()
    }

    /// Returns all entries with the given meet_id.
    ///
    /// Those entries could be located anywhere in the entries vector,
    /// so they are found using a linear scan.
    pub fn get_entries_for_meet<'a>(&'a self, meet_id: u32) -> Vec<&'a Entry> {
        self.get_entries()
            .into_iter()
            .filter(|&e| e.meet_id == meet_id)
            .collect()
    }
}
