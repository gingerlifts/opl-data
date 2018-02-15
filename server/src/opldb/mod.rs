//! An in-memory data store for OpenPowerlifting data.
//!
//! Because our data is read-only at runtime, we can lay out data structures
//! better than a "real" database like SQLite3 or PostgreSQL. Additionally,
//! by storing all the data in formats native to Rust, we avoid copy overhead.

use csv;

use std::error::Error;
use std::mem;

pub mod fields;
use self::fields::*;

mod filter;
pub use self::filter::Filter;

mod filter_cache;
use self::filter_cache::FilterCache;
pub use self::filter_cache::CachedFilter;

/// The definition of a Lifter in the database.
#[derive(Serialize,Deserialize)]
pub struct Lifter {
    #[serde(rename(serialize = "name", deserialize = "Name"))]
    pub name: String,
    #[serde(rename(serialize = "username", deserialize = "Username"))]
    pub username: String,
    #[serde(rename(serialize = "instagram", deserialize = "Instagram"))]
    pub instagram: Option<String>,
}

/// The definition of a Meet in the database.
#[derive(Deserialize)]
pub struct Meet {
    #[serde(rename = "MeetPath")]
    pub path: String,
    #[serde(rename = "Federation")]
    pub federation: Federation,
    #[serde(rename = "Date")]
    pub date: Date,
    #[serde(rename = "MeetCountry")]
    pub country: String,
    #[serde(rename = "MeetState")]
    pub state: Option<String>,
    #[serde(rename = "MeetTown")]
    pub town: Option<String>,
    #[serde(rename = "MeetName")]
    pub name: String,
}

/// The definition of an Entry in the database.
#[derive(Deserialize)]
pub struct Entry {
    #[serde(rename = "MeetID")]
    pub meet_id: u32,
    #[serde(rename = "LifterID")]
    pub lifter_id: u32,
    #[serde(rename = "Sex")]
    pub sex: Sex,
    #[serde(rename = "Event")]
    pub event: Event,
    #[serde(rename = "Equipment")]
    pub equipment: Equipment,
    #[serde(rename = "Age")]
    pub age: Age,
    #[serde(rename = "Division")]
    pub division: Option<String>,
    #[serde(rename = "BodyweightKg")]
    #[serde(deserialize_with = "deserialize_f32_with_default")]
    pub bodyweightkg: f32,
    #[serde(rename = "WeightClassKg")]
    pub weightclasskg: WeightClassKg,
    #[serde(rename = "Squat1Kg")]
    pub squat1kg: WeightKg,
    #[serde(rename = "Squat2Kg")]
    pub squat2kg: WeightKg,
    #[serde(rename = "Squat3Kg")]
    pub squat3kg: WeightKg,
    #[serde(rename = "Squat4Kg")]
    pub squat4kg: WeightKg,
    #[serde(rename = "BestSquatKg")]
    pub bestsquatkg: WeightKg,
    #[serde(rename = "Bench1Kg")]
    pub bench1kg: WeightKg,
    #[serde(rename = "Bench2Kg")]
    pub bench2kg: WeightKg,
    #[serde(rename = "Bench3Kg")]
    pub bench3kg: WeightKg,
    #[serde(rename = "Bench4Kg")]
    pub bench4kg: WeightKg,
    #[serde(rename = "BestBenchKg")]
    pub bestbenchkg: WeightKg,
    #[serde(rename = "Deadlift1Kg")]
    pub deadlift1kg: WeightKg,
    #[serde(rename = "Deadlift2Kg")]
    pub deadlift2kg: WeightKg,
    #[serde(rename = "Deadlift3Kg")]
    pub deadlift3kg: WeightKg,
    #[serde(rename = "Deadlift4Kg")]
    pub deadlift4kg: WeightKg,
    #[serde(rename = "BestDeadliftKg")]
    pub bestdeadliftkg: WeightKg,
    #[serde(rename = "TotalKg")]
    pub totalkg: WeightKg,
    #[serde(rename = "Place")]
    pub place: Place,
    #[serde(rename = "Wilks")]
    #[serde(deserialize_with = "deserialize_f32_with_default")]
    pub wilks: f32,
    #[serde(rename = "McCulloch")]
    #[serde(deserialize_with = "deserialize_f32_with_default")]
    pub mcculloch: f32,
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

    /// The cache of filters on the vectors.
    filter_cache: FilterCache,
}

/// Reads the `lifters.csv` file into a Vec<Lifter>.
fn import_lifters_csv(file: &str) -> Result<Vec<Lifter>, Box<Error>> {
    let mut vec = Vec::with_capacity(140_000);

    let mut rdr = csv::Reader::from_path(file)?;
    for lifter in rdr.deserialize() {
        let lifter: Lifter = lifter?;
        vec.push(lifter);
    }

    vec.shrink_to_fit();
    Ok(vec)
}

/// Reads the `meet.csv` file into a Vec<Meet>.
fn import_meets_csv(file: &str) -> Result<Vec<Meet>, Box<Error>> {
    let mut vec = Vec::with_capacity(10_000);

    let mut rdr = csv::Reader::from_path(file)?;
    for meet in rdr.deserialize() {
        let meet: Meet = meet?;
        vec.push(meet);
    }

    vec.shrink_to_fit();
    Ok(vec)
}

/// Reads the `entries.csv` file into a Vec<Entry>.
fn import_entries_csv(file: &str) -> Result<Vec<Entry>, Box<Error>> {
    let mut vec = Vec::with_capacity(450_000);

    let mut rdr = csv::Reader::from_path(file)?;
    for entry in rdr.deserialize() {
        let entry: Entry = entry?;
        vec.push(entry);
    }

    // The entries database is sorted by lifter_id.
    // This invariant allows for extremely efficient lifter-uniqueness
    // filtering without constructing additional data structures.
    vec.sort_unstable_by_key(|e| e.lifter_id);

    vec.shrink_to_fit();
    Ok(vec)
}

impl OplDb {
    /// Constructs the `OplDb` from CSV files produces by the project
    /// build script.
    pub fn from_csv(lifters_csv: &str, meets_csv: &str, entries_csv: &str)
        -> Result<OplDb, Box<Error>>
    {
        let lifters = import_lifters_csv(lifters_csv)?;
        let meets = import_meets_csv(meets_csv)?;
        let entries = import_entries_csv(entries_csv)?;

        let filter_cache = FilterCache::new(&meets, &entries);

        Ok(OplDb { lifters, meets, entries, filter_cache })
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
            owned_strings += mem::size_of::<String>() + meet.country.len();
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
    pub fn get_lifters(&self) -> &Vec<Lifter> {
        &self.lifters
    }

    /// Borrows the meets vector.
    pub fn get_meets(&self) -> &Vec<Meet> {
        &self.meets
    }

    /// Borrows the entries vector.
    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    /// Borrows a `Lifter` by index.
    pub fn get_lifter(&self, n: u32) -> &Lifter {
        &self.lifters[n as usize]
    }

    /// Borrows a `Meet` by index.
    pub fn get_meet(&self, n: u32) -> &Meet {
        &self.meets[n as usize]
    }

    /// Borrows an `Entry` by index.
    pub fn get_entry(&self, n: u32) -> &Entry {
        &self.entries[n as usize]
    }

    /// Borrows a cached filter.
    pub fn get_filter(&self, c: CachedFilter) -> &Filter {
        &self.filter_cache.from_enum(c)
    }

    /// Look up the lifter_id by username.
    pub fn get_lifter_id(&self, username: &str) -> Option<u32> {
        for i in 0 .. self.lifters.len() {
            if self.lifters[i].username == username {
                return Some(i as u32)
            }
        }
        None
    }
}
