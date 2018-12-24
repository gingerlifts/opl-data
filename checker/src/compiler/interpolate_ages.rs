use opltypes::*;
use std::cmp::Ordering;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AgeData {
    pub age: Age,
    pub minage: Age,
    pub maxage: Age,
    pub date: Date,
    pub birthyear: Option<u32>,
    pub birthdate: Option<Date>,
    pub linenum: u32,
}


//FIXME
impl PartialOrd for AgeData {
    fn partial_cmp(&self, other: &AgeData) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AgeData {
    fn cmp(&self, other: &AgeData) -> Ordering {
    	self.date.cmp(&other.date)
    }
}


fn calc_age(birthdate: Date, date: Date){
    match birthdate.monthday() {
    	x > date.monthday() => date.year() - birthdate.year() -1,
    	_ => date.monthday() => date.year() - birthdate.year(),
    } 

}


fn is_birthyear_consistent(entries: &[AgeData]) -> bool {
    true
}




fn is_birthdate_consistent(entries: &[AgeData]) -> bool {
	let mut prev_birthdate = None;

	//almost certainly a crap way to do this
    let mut bd_data = Vec::with_capacity(entries.len());

    for entry in entries {
    	if entry.birthdate.is_some() && prev_birthdate.is_some() && entry.birthdate != prev_birthdate{
    		return false;
    	}
    	prev_birthdate = entry.birthdate;
 
        //wasn't sure about the proper way to do this, this works in the meantime
    	match entry.age {
            Age::Exact(age) => bd_data.push(entry),
            _ => continue,
        }
    }

    if bd_data.len() != 0{
    	bd_data.sort();

    	let init_year = bd_data[0].date.year();

    	for entry in bd_data{
	        if entry.birthdate.is_some() && calc_age(prev_birthdate,entry.date) != entry.age{
	        	return false;
	        }

	    //NOTE: there was some extra logic here in the Python version, but I think this was superflouous

    	}
    }

    true
}

fn is_agedata_consistent(entries: &[AgeData]) -> bool {
    is_birthdate_consistent(entries) && is_birthdate_consistent(entries)
} 

fn interpolate_array(entries: &mut [AgeData]) {
}

pub fn interpolate(entries: &mut [AgeData]) {
    if is_agedata_consistent(entries){
        interpolate_array(entries);
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alldata() {
        let a = AgeData { age: Age::Exact(20), minage: Age::Exact(20), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 1000 };
        let b = AgeData { age: Age::Exact(20), minage: Age::Exact(20), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };

        let c = AgeData { age: Age::Exact(20), minage: Age::Exact(20), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 1000 };
        let d = AgeData { age: Age::Exact(20), minage: Age::Exact(20), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };


        let mut interp_arr = [a,b];
        let old_arr = [c,d];

        interpolate(&mut interp_arr);

        assert!(interp_arr.iter().eq(old_arr.iter()));
    }

    // #[test]
    // fn test_interp() {
    //     let a = AgeData { age: Age::Exact(20), minage: Age::Exact(20), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };
    //     let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20011231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };

    //     let c = AgeData { age: Age::Exact(20), minage: Age::Exact(20), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };
    //     let d = AgeData { age: Age::Exact(21), minage: Age::Exact(21), maxage: Age::Exact(21), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };


    //     let mut interp_arr = [a,b];
    //     let old_arr = [c,d];

    //     interpolate(&mut interp_arr);

    //     assert!(interp_arr.iter().eq(old_arr.iter()));
    // }

    #[test]
    fn test_nodata() {
        let a = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20001231),birthyear:None, birthdate: None, linenum: 1000 };
        let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20011231),birthyear:None, birthdate: None, linenum: 2000 };

        let c = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20001231),birthyear:None, birthdate: None, linenum: 1000 };
        let d = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20011231),birthyear:None, birthdate: None, linenum: 2000 };


        let mut interp_arr = [a,b];
        let old_arr = [c,d];

        interpolate(&mut interp_arr);

        assert!(interp_arr.iter().eq(old_arr.iter()));
    }
}