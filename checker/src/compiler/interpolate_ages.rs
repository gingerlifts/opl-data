use opltypes::*;
use std::cmp::Ordering;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AgeData {
    pub age: Age,
    pub minage: Age,
    pub maxage: Age,
    pub birthyear: Option<u32>,
    pub birthdate: Option<Date>,
    pub date: Date,
    pub linenum: u32,
}

struct Bound {
    pub birthdate_min: Date,
    pub birthdate_max: Date,
}

struct KnownRegion {
    pub known_date_min: Date,
    pub known_date_max: Date,
    pub known_age: u8,
}

struct Birthdate_constraint{
    pub bound : Option<Bound>,
    pub knownregion: Option<KnownRegion>, 
}


// FIXME
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


// Get the difference in years between 2 dates
fn year_diff(date1: Date,date2: Date) -> u8 {
	if date1.year() < date2.year() {
	    if date1.monthday() < date2.monthday(){
	        return date1.year() - date2.year();
	    }
	    else{
	    return date1.year() - date2.year() -1;
	    } 
	}
	else if date1.year() > date2.year() {
	    if date2.monthday() < date1.monthday(){
	        return date2.year() - date1.year();
	    }
	    else{
	    return date2.year() - date1.year() -1;
	    } 
	}
	return 0
}

// TODO: Make use of the fuzz factor
// TODO: See if you can switch to using dates throughout
// Estimates the range that a lifters birthday lies within
// Dates are either a range in which we know the birthdate lies in (if the bool is true) & an exclusion zone otherwise
// This is also probably very buggy right now
fn estimate_birthdate(entries: &[entry]) -> Birthdate_constraint
{
    //Fuzz factor for long meets incase the lifter has their birthday over the meet
    let MAX_MEETLENGTH = 12;

    // Dates used to bound the birthdate    
    let bd1_range_min = Option<Date>;
    let bd1_range_max = Option<Date>;

    let bd2_range_min = Option<Date>;
    let bd2_range_max = Option<Date>;

    // Used to offset the ages to be from the same year
    let init_year = Option<u32>;
    let init_age = Option<u8>;

    for entry in entries{
        // If the lifter has a recorded birthdate use that
    	if entry.birthdate.is_some(){
    		return BirthDateConstraint{Bound{entry.birthdate,entry.birthdate},None};
    	}

    	if entry.birthyear.is_some(){
            bd1_range_min = Date(entry.birthyear*1000+0101);
            bd1_range_max = Date(entry.birthyear*1000+0101);

            bd2_range_min = Date(entry.birthyear*1000+1231);
            bd2_range_max = Date(entry.birthyear*1000+1231);
    	}

        // Use age to tighten our birthdate bound
    	match entry.age{
			Age::Exact(age) => {
                if bd1_range_min.is_none(){
                    bd1_range_min = Date(entry.date);
                    bd1_range_max = Date(entry.date);
                    init_year = entry.date.year();
                    init_age = entry.age();                    
                }

                // Another instance of the first age
				if entry.age - (entry.date.year-init_year) == init_age{
                    if entry.date.monthday < bd1_range_min.monthday(){
                    	bd1_range_min = Date(bd1_range_min.year()*1000+entry.date.monthday());
                    }
                    else if entry.date.monthday > bd1_range_max.monthday(){
                        bd1_range_max = Date(bd1_range_max.year()*1000+entry.date.monthday());
                    }
				}
				else{
					// If we've found an age change, setup the age range & update the birthyear range
					if bd2_range_min.is_none() {
						if entry.age - (entry.date.year-init_year) < init_age{
                            bd2_range_min = Date((entry.date.year() - age -1)*1000 + entry.date.monthday());
                            bd2_range_max = Date((entry.date.year() - age -1)*1000 + entry.date.monthday());
						}
						else{
                            bd2_range_min = Date((entry.date.year() - age)*1000 + entry.date.monthday());
                            bd2_range_max = Date((entry.date.year() - age)*1000 + entry.date.monthday());
						}
					}
					if entry.date.monthday < bd2_range_min.monthday(){
                        bd2_range_min = Date(bd2_range_min.year()*1000+entry.date.monthday());
                    }
                    else if entry.date.monthday > bd2_range_max.monthday(){
                        bd2_range_max = Date(bd2_range_max.year()*1000+entry.date.monthday());
                    }
				}
			}
			Age::Approximate(age) => { 
                if bd1_range_min.is_none(){
                    bd1_range_min = Date((entry.date.year()-age)*1000+entry.date.monthday());
                    bd1_range_max = Date((entry.date.year()-age+1)*1000+entry.date.monthday());
                    init_year = entry.date.year();
                    init_age = entry.age();                    
                }
				if bd1_range_min < entry.date.year() - age
				{
                    bd1_range_min = Date((entry.date.year()-age)*1000+entry.date.monthday());
				}
				if bd1_range_max > entry.date.year() - age
				{
                    bd1_range_max = Date((entry.date.year()-age+1)*1000+entry.date.monthday());
				}
			}
			Age::None =>,
    	}

        // Use minage to tighten our birthdate bound slightly
    	match entry.minage{ //FIXME
    		Age::Exact(minage) => {
    			if entry.year - minage < bd1_range_max.year(){
    				bd1_range_max = Date((entry.year - minage)*1000+1231);
    			}
                if init_age.is_some(){
        			if minage > init_age { //Then we can bound their birthdate from the division
        				birthyear_range = [entry.year-minage,entry.year-minage] 
        				if age2_monthday_range[0].is_none(){
        					age2_monthday_range = [entry.date,entry.date];
        				}
        				if entry.date < age2_monthday_range[0] {
        					age2_monthday_range[0] = entry.date;
        				}
        				else{
        					age2_monthday_range[1] = entry.date;
        				}
        			}
        			else if minage == init_age{
    					if entry.date < age1_monthday_range[0] {
        					age1_monthday_range[0] = entry.date;
        				}
        				else{
        					age1_monthday_range[1] = entry.date;
        				}
        			}
                }
    		}
    		Age::Approximate(minage) => {
    			if entry.year - minage < bd1_range_max.year(){
                    bd1_range_max = Date((entry.year - minage)*1000+1231);
                }
    			if init_age.is_some() && maxage - 1 == init_age && age2_monthday_range[0].is_none(){ // Then they must have had their birthday after the init_age entry, but we don't know when
                    birthyear_range = [entry.year - maxage,entry.year - maxage]
	                age2_monthday_range = [age1_monthday_range[1],1231]; 
    			}		
    		}
    		Age:None =>,
    	}

        // Use maxage to tighten our birthdate bound slightly
    	match entry.maxage{
    		Age::Exact(maxage) => {
	  			if entry.year - maxage > birthyear_range[0]{
	    				 birthyear_range[0] = entry.year - maxage;
    			} 
                if init_age.is_some(){
        			if maxage < init_age {
        				if age2_monthday_range.is_none(){
        					age2_monthday_range = [entry.date,entry.date];
        				}
        				if entry.date < age2_monthday_range[0] {
        					age2_monthday_range[0] = entry.date;
        				}
        				else{
        					age2_monthday_range[1] = entry.date;
        				}
        			}
        			else if maxage == init_age{
    					if entry.date < age1_monthday_range[0] {
        					age1_monthday_range[0] = entry.date;
        				}
        				else{
        					age1_monthday_range[1] = entry.date;
        				}
        			}
                }
    		}
			Age::Approximate(maxage) => {
    			if entry.year - maxage > birthyear_range[0]{
    				 birthyear_range[0] = entry.year - maxage;
    			}
    			if init_age.is_some() && maxage == init_age && age2_monthday_range[0].is_none() { // Then they must have had their birthday before the init_age entry, but we don't know when
                    birthyear_range = [entry.year - maxage,entry.year - maxage]
                    age2_monthday_range = [0101,age1_monthday_range[0]]; 
    			}    		
    		}
    		Age:None =>,
    	}
    }

    // Bounded, first age range is before second
    if age1_monthday_range.is_some() && age2_monthday_range.is_some(){
    	if age1_monthday_range[1] < age2_monthday_range[0]{
            return Birthdate_constraint{Bound{birthyear_range[0]*1000+age1_monthday_range[1],birthyear_range[1]*1000+age2_monthday_range[0]},None}
	    } // Bounded, second age range is before first
	    else if age2_monthday_range[1] < age2_monthday_range[0]{
            return Birthdate_constraint{Bound{birthyear_range[0]*1000+age2_monthday_range[1],birthyear_range[1]*1000+age1_monthday_range[0]},None}
	    }
    }// Not bounded, return exclusion zone
    else age1_monthday_range.is_some() {
        return Birthdate_constraint{None,KnownRegion{birthyear_range[0]*1000+age1_monthday_range[0],birthyear_range[1]*1000+age1_monthday_range[1],init_age.unwrap()}}
    }

    // We haven't successfully bounded the birthday, return a birthyear range
    Birthdate_constraint{Bound{birthyear_range[0]*1000+0101,birthyear_range[1]*1000+1231},None}
 
}


// Check if two AgeData are consistent with one another
// Apologies for how long this function is...
fn are_entries_consistent(entry1 : &AgeData, entry2: &AgeData) -> bool {
    let yd = year_diff(entry1.date,entry2.date);

    // Check that entry1.age is consistent with the data in entry2
    match entry1.age {
        Age::Exact(age1) => {
		    match entry2.age {
	    		Age::Exact(age2) => if abs(age1-age2) != yd {return false;},
	    		Age::Approximate(age2) => if abs(age1-age2) - yd != (0|1) {return false;},
	    		Age::None =>,
	    	}
	    	match entry2.minage { 
	    		Age::Exact(minage2)       => if abs(age1-minage2) < yd {return false;},
	    		Age::Approximate(minage2) => if abs(age1-minage2) < yd - 1 {return false;},
	    		Age::None =>,
	    	}
	    	match entry2.maxage {
	    		Age::Exact(maxage2)       => if abs(age1-maxage2) > yd {return false;},
	    		Age::Approximate(maxage2) => if abs(age1-maxage2) > yd - 1 {return false;},
	    		Age::None =>,
	    	}
	    	if entry2.birthdate.is_some()  && age1 != entry2.birthdate.age_on(entry2.date) {return false}
	    	if entry2.birthyear.is_some()  && if (entry1.date.year() - entry2.birthyear.unwrap()) as u8 - age != (0 | 1) {return false;}
        },
        Age::Approximate(age1) => {
			match entry2.age {
	    		Age::Exact(age2) => if abs(age1-age2) - yd != (0|1),
	    		Age::Approximate(age2) => if abs(age1-age2) != yd {return false;},
	    		Age::None =>,
	    	}
	    	match entry2.minage {
	    		Age::Exact(minage2)       => if abs(age1-minage2) < yd + 1 {return false;},
	    		Age::Approximate(minage2) => if abs(age1-minage2) < yd {return false;},
	    		Age::None =>,
	    	}
	    	match entry2.maxage {
	    		Age::Exact(maxage2)       => if abs(age1-maxage2) > yd + 1 {return false;},
	    		Age::Approximate(maxage2) => if abs(age1-maxage2) > yd,
	    		Age::None =>,
	    	}
	    	if entry2.birthdate.is_some()  && if (age1 - entry2.birthdate.age_on(entry2.date)) != (0 | 1) {return false;} 
	    	if entry2.birthyear.is_some()  && if (entry1.date.year() - entry2.birthyear.unwrap()) as u8 - age != 0 {return false;}
        },
        Age::None =>,
    }

    // Check that entry1.minage is consistent with the data in entry2
    match entry1.minage{
            Age::Exact(minage1) => {
                match entry2.age{
                	Age::Exact(age2)       => if abs(age2-minage1) < yd {return false;},
                	Age::Approximate(age2) => if abs(age2-minage1) < yd + 1 {return false;},
                	Age::None(age2) =>,

                }
                
                match entry2.maxage{
                	Age::Exact(maxage2) => if entries1.date < entries2.date && minage1+yd > maxage2 {return false;},
                	Age::Approximate(maxage2)=> if entries1.date < entries2.date && minage1 + yd > maxage2 {return false;},
                	Age::None(maxage2) => ,

                }
            	if entry2.birthyear.is_some() && (entry1.date.year() - entry2.birthyear.unwrap()) as u8 < minage {return false;}
            	if entry2.birthdate.is_some()  && minage1 > entry2.birthdate.age_on(entry2.date) {return false;}
            },
            Age::Approximate(minage1) => {
                match entry2.age{
                	Age::Exact(age2)       => if abs(age2-minage1) < yd - 1 {return false;},
                	Age::Approximate(age2) => if abs(age2-minage1) < yd {return false;},
                	Age::None(age2) =>,

                }
 
                match entry2.maxage{
                	Age::Exact(maxage2) => if entries1.date < entries2.date && minage1+yd -1 > maxage2 {return false;},
                	Age::Approximate(maxage2) => if entries1.date < entries2.date && minage1+yd > maxage2 {return false;},
                	Age::None(maxage2) =>

                }
            	if entry2.birthyear.is_some() && (entry1.date.year() - entry2.birthyear.unwrap()) as u8 < minage {return false;},
            	if entry2.birthdate.is_some()  && minage1 -1 > entry2.birthdate.age_on(entry2.date) {return false;}
            },
            Age::None =>,
    }

    // Check that entry1.maxage is consistent with the data in entry2
    match entry1.maxage{
            Age::Exact(maxage1) => {
                match entry2.age{
                	Age::Exact(age2)       => if abs(age2-maxage1) > yd {return false;},
                	Age::Approximate(age2) => if abs(age2-maxage1) > yd + 1 {return false;},
                	Age::None(age2) =>,

                }
                match entry2.minage{
                	Age::Exact(minage2) => if entries2.date < entries1.date && minage2+yd > maxage1 {return false;},
                	Age::Approximate(minage2)=> if entries2.date < entries1.date && minage2 + yd -1 > maxage1 {return false;},
                	Age::None(minage2) =>

                }

            	if entry2.birthyear.is_some() && (entry1.date.year() - entry2.birthyear.unwrap()-1) as u8 > maxage1 {return false;}
            	if entry2.birthdate.is_some()  && maxage1 < entry2.birthdate.age_on(entry2.date) {return false;}
            },
            Age::Approximate(maxage1) => {
				match entry2.age{
                	Age::Exact(age2)       => if abs(age1-maxage2) > yd - 1 {return false;},
                	Age::Approximate(age2) => if abs(age2-maxage1) > yd,
                	Age::None(age2) =>,

                }
                match entry2.minage{
                	Age::Exact(minage2)=> if entries2.date < entries1.date && minage2 + yd > maxage1 {return false;},
                	Age::Approximate(minage2) => if entries2.date < entries1.date && minage2+yd > maxage1 {return false;},
                	Age::None(minage2) =>, 

                }

            	if entry2.birthyear.is_some() && (entry1.date.year() - entry2.birthdate.unwrap()-1) as u8 > maxage1 {return false;}
            	if entry2.birthdate.is_some()  && maxage1 < entry2.birthdate.age_on(entry2.date) {return false;}
            },
            Age::None =>,
    }
    	
    // Check that entry1.birthyear is consistent with the data in entry2
    if entry1.birthyear.is_some() {
    	match entry2.age {
	        Age::Exact(age)       => if (entry2.date.year() - entry1.birthyear.unwrap()-1) as u8 - age != (0 | 1) {return false;},
	        Age::Approximate(age) => if (entry2.date.year() - entry1.birthdate.unwrap()-1) as u8 - age != 0 {return false;},
	        Age::None => (),
    	}
    	match entry2.minage {
	        Age::Exact(minage)       => if (entry2.date.year() - entry1.birthyear.unwrap()) as u8 < minage {return false;},
	        Age::Approximate(minage) => if (entry2.date.year() - entry1.birthdate.unwrap()) as u8 < minage {return false;},
	        Age::None => (),
    	}
    	match entry2.maxage {
	        Age::Exact(maxage)       => if (entry2.date.year() - entry1.birthyear.unwrap()-1) as u8 > maxage {return false;},
	        Age::Approximate(maxage) => if (entry2.date.year() - entry1.birthdate.unwrap()-1) as u8 > maxage {return false;},
	        Age::None => (),
    	}
    	if entry2.birthyear.is_some() && entry1.birthyear.unwrap() != entry2.birthyear.unwrap() {return false}
    	if entry2.birthdate.is_some() && entry1.birthyear.unwrap() != entry2.birthdate.unwrap().year() {return false}
    }

    // Check that entry1.birthdate is consistent with the data in entry2
    if entry1.birthdate.is_some() {
    	if entry2.age != Age::None  && entry1.birthdate.age_on(entry1.date) != entry2.age {return false}
    	if entry2.minage != Age::None  && entry1.birthdate.age_on(entry1.date) < entry2.minage {return false}
    	if entry2.maxage != Age::None && entry1.birthdate.age_on(entry1.date) > entry2.maxage {return false}
    	if entry2.birthyear.is_some() && entry1.birthdate.unwrap().year() != entry2.birthyear.unwrap() {return false}
    	if entry2.birthdate.is_some() && entry1.birthdate.unwrap() != entry2.birthdate.unwrap() {return false}
    }
  
    true
}



// Check that lifter age data is consistent across several entries
fn is_agedata_consistent(entries: &[AgeData]) -> bool {
	if entries.is_empty() { return true;}

	// This is O(N^2), there is probably a more efficient way if doing this...
	for ii in range(0,entries.len()){
		for jj in range(0,entries.len()){
			if !are_entries_consistent(&entries[ii],&entries[jj]) { return false;}
		}
	}

	true
}



fn interpolate_array(entries: &mut [AgeData]) {
	let bd_constraint = estimate_birthdate(entries);

	for entry in entries{
        if bd_constraint.bound.is_some(){
            let birthdate_min = bd_constraint.bound.birthdate_min;
            let birthdate_max = bd_constraint.bound.birthdate_max;

    		//Then we know the lifters birthyear
    		if birthdate_min.year() == birthdate_max.year() {
    			entry.birthyear = birthdate_min.year();
    			// Then we know the lifters birthdate exactly
    			if birthdate_min.monthday() == birthdate_max.monthday(){
    				entry.birthdate = birthdate_min;
    				entry.age = entry.birthdate.age_on(entry.date);
    				entry.minage = entry.age;
    				entry.maxage = entry.age;
    			}
    			else{ //Assign an approximate age range
    				entry.age = Age::Approximate(entry.birthyear-entry.date.year()-1);
                    entry.minage = entry.date.year()-entry.birthyear-1;
                    entry.maxage = entry.date.year()-entry.birthyear;
                }
    		}
    		else{ //Assign an age range
    			entry.minage = entry.date.year()-birthdate_max.year() -1;
    			entry.maxage = entry.date.year()-birthdate_min.year();
    		}
        }
       //Haven't got a monthday bound for the birthdate, just a region where the lifter doen't have a birthday
        else if bd_constraint.knownregion.is_some(){
            let known_date_min = bd_constraint.knownregion.known_date_min;
            let known_date_max = bd_constraint.knownregion.known_date_max;
            let known_age = bd_constraint.knownregion.known_age;
            if entry.date.monthday() < known_date_min.monthday(){ // We have an upper bound on the age
                 entry.age = Age::Aproximate(known_age - (known_date_min.year()-entry.date.year())-1);
                 entry.minage = Age::Exact(known_age - (known_date_min.year()-entry.date.year())-1);
                 entry.maxage = Age::Exact(known_age - (known_date_min.year()-entry.date.year()));
            }
            else if entry.date.monthday() > known_date_max.monthday(){ // We have a lower bound on the age
                 entry.age = Age::Aproximate(known_age - (known_date_min.year()-entry.date.year()));
                 entry.minage = Age::Exact(known_age - (known_date_min.year()-entry.date.year()));
                 entry.maxage = Age::Exact(known_age - (known_date_min.year()-entry.date.year())+1);
            }
            else{ // We can give the age exactly for this date
                 entry.age = Age::Exact(known_age - (known_date_min.year()-entry.date.year()));
                 entry.minage = entry.age;
                 entry.maxage = entry.age;
            }
        }
	}
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
    // fn test_basic_interp() {
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