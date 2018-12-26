use crate::check_entries::Entry;
use opltypes::*;

fn get_country(entries: &[Entry]) -> Option<Country> {
    for entry in entries {
        if entry.country.is_some() {
            return entry.country;
        }
    }
    None
}

fn is_country_consistent(entries: &[Entry]) -> bool {
    let mut curr_country = None;
    for entry in entries {
        if entry.country.is_some() {
            if curr_country.is_some() && entry.country != curr_country {
                return false;
            }
            curr_country = entry.country;
        }
    }

    true
}

fn interpolate_array(entries: &mut [Entry]) {
    let lifter_country = get_country(entries);
    for entry in entries {
        entry.country = lifter_country;
    }
}

pub fn interpolate(entries: &mut [Entry]) {
    if is_country_consistent(entries) {
        interpolate_array(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interp_start() {
        let a = Entry {
            country: None,
            ..Entry::default()
        };
        let b = Entry {
            country: None,
            ..Entry::default()
        };
        let c = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let d = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };

        let e = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let f = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let g = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let h = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };

        let mut interp_arr = [a, b, c, d];
        let old_arr = [e, f, g, h];

        interpolate(&mut interp_arr);

        assert!(interp_arr.iter().eq(old_arr.iter()));
    }

    #[test]
    fn test_interp_end() {
        let a = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let b = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let c = Entry {
            country: None,
            ..Entry::default()
        };
        let d = Entry {
            country: None,
            ..Entry::default()
        };

        let e = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let f = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let g = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let h = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };

        let mut interp_arr = [a, b, c, d];
        let old_arr = [e, f, g, h];

        interpolate(&mut interp_arr);

        assert!(interp_arr.iter().eq(old_arr.iter()));
    }

    #[test]
    fn test_interp_gaps() {
        let a = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let b = Entry {
            country: None,
            ..Entry::default()
        };
        let c = Entry {
            country: None,
            ..Entry::default()
        };
        let d = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };

        let e = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let f = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let g = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let h = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };

        let mut interp_arr = [a, b, c, d];
        let old_arr = [e, f, g, h];

        interpolate(&mut interp_arr);

        assert!(interp_arr.iter().eq(old_arr.iter()));
    }

    #[test]
    fn test_invalid_interp() {
        let a = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let b = Entry {
            country: Some(Country::Estonia),
            ..Entry::default()
        };
        let c = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };

        let d = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let e = Entry {
            country: Some(Country::Estonia),
            ..Entry::default()
        };
        let f = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };

        let mut interp_arr = [a, b, c];
        let old_arr = [d, e, f];

        interpolate(&mut interp_arr);

        assert!(interp_arr.iter().eq(old_arr.iter()));
    }

}
