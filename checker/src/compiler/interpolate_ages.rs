use opltypes::*;
use std::cmp::Ordering;

use crate::check_entries::Entry;


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

enum BirthdateConstraint{
	// Region that a lifters birthdate lies in
	Bound {min_date: Date, max_date: Date},

	//Region in which we know that the lifter was 0
	KnownRegion {min_date: Date, max_date: Date},

	// No known age information
	None,
}

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
	if date1.year() > date2.year() {
	    if date1.monthday() >= date2.monthday(){
	        return (date1.year() - date2.year()) as u8;
	    }
	    else{
	    return (date1.year() - date2.year() -1) as u8;
	    } 
	}
	else if date1.year() < date2.year() {
	    if date2.monthday() >= date1.monthday(){
	        return (date2.year() - date1.year()) as u8;
	    }
	    else{
		    return (date2.year() - date1.year() -1) as u8;
	    } 
	}
	return 0
}

// TODO: Make use of the fuzz factor
// Estimates the range that a lifters birthday lies within
// Dates are either a range in which we know the birthdate lies in or a range in which we know the lifters age
// This is also probably very buggy right now
fn estimate_birthdate(entries: &[AgeData]) -> BirthdateConstraint
{
    //Fuzz factor for long meets incase the lifter has their birthday over the meet
    let MAX_MEETLENGTH = 12;

    // Ranges used to bound the birthdate    
    let mut bd1_range : Option<(Date,Date)> = None;
    let mut bd2_range : Option<(Date,Date)> = None;

    // Used to offset the ages to be from the same year
    let mut init_year : Option<u32> = None;
    let mut init_age : Option<u8> = None;

    for entry in entries
    {
        // If the lifter has a recorded birthdate use that
    	if entry.birthdate.is_some(){
    		return BirthdateConstraint::Bound{min_date:entry.birthdate.unwrap(), max_date: entry.birthdate.unwrap()};
    	}

    	if entry.birthyear.is_some(){
            bd1_range = Some((Date::from_u32(entry.birthyear.unwrap()*1000+0101),Date::from_u32(entry.birthyear.unwrap()*1000+0101)));
            bd2_range = Some((Date::from_u32(entry.birthyear.unwrap()*1000+1231),Date::from_u32(entry.birthyear.unwrap()*1000+1231)));
    	}

        // Use age to tighten our birthdate bound
    	match entry.age{
			Age::Exact(age) => {
                if init_age.is_none()
                {
                    bd1_range = Some((Date::from_u32((entry.date.year()-age as u32)*1000+entry.date.monthday()),Date::from_u32((entry.date.year()-age as u32)*1000+entry.date.monthday())));
                    init_year = Some(entry.date.year());
                    init_age = Some(age);                    
                }

                // Another instance of the first age
				if age - (entry.date.year()-init_year.unwrap()) as u8 == init_age.unwrap(){
                    if entry.date.monthday() < bd1_range.unwrap().0.monthday(){
                    	bd1_range.unwrap().0 = Date::from_u32(bd1_range.unwrap().0.year()*1000+entry.date.monthday());
                    }
                    else if entry.date.monthday() > bd1_range.unwrap().1.monthday(){
                        bd1_range.unwrap().1 = Date::from_u32(bd1_range.unwrap().1.year()*1000+entry.date.monthday());
                    }
				}
				else{
					// If we've found an age change, setup the age range & update the birthyear range
					if bd2_range.is_none() {
						if age - ((entry.date.year()-init_year.unwrap()) as u8) < init_age.unwrap(){
                            bd2_range = Some((Date::from_u32((entry.date.year() - age  as u32 -1)*1000 + entry.date.monthday()),Date::from_u32((entry.date.year()  - age  as u32 -1)*1000 + entry.date.monthday())));
						}
						else{
                            bd2_range = Some((Date::from_u32((entry.date.year() - age  as u32)*1000 + entry.date.monthday()),Date::from_u32((entry.date.year()  - age  as u32)*1000 + entry.date.monthday())));
						}
					}
					if entry.date.monthday() < bd2_range.unwrap().0.monthday(){
                        bd2_range.unwrap().0 = Date::from_u32(bd2_range.unwrap().0.year()*1000+entry.date.monthday());
                    }
                    else if entry.date.monthday() > bd2_range.unwrap().1.monthday(){
                        bd2_range.unwrap().1 = Date::from_u32(bd2_range.unwrap().1.year()*1000+entry.date.monthday());
                    }
				}
			},
			Age::Approximate(age) => { 
                if bd1_range.is_none(){
                    bd1_range = Some((Date::from_u32((entry.date.year()-age as u32)*1000+entry.date.monthday()),Date::from_u32((entry.date.year()-age  as u32+1)*1000+entry.date.monthday())));
                    init_year = Some(entry.date.year());
                    init_age = Some(age);                    
                }
				if bd1_range.unwrap().0.year() == entry.date.year()  - age  as u32 //Then we know that there birthday is after the initially seen age
				{
					if bd2_range.is_none(){
						bd2_range = Some((Date::from_u32((entry.date.year()  - age as u32-1)*1000+0101),Date::from_u32((entry.date.year()  - age  as u32-1)*1000+1231)));
					}
                    
				}
				if bd1_range.unwrap().1.year() == entry.date.year()  - age as u32 - 1 //Then we know that there birthday is before the initially seen age
				{
					if bd2_range.is_none(){
						bd2_range = Some((Date::from_u32((entry.date.year()- age as u32)*1000+0101),Date::from_u32((entry.date.year() - age  as u32)*1000+1231)));
					}				
				}
			},
			Age::None =>(),
    	}

        // Use minage to tighten our birthdate bound slightly
    	match entry.minage{ 
    		Age::Exact(minage) => {
    			if bd1_range.is_none(){
    				bd1_range = Some((Date::from_u32(00000101),Date::from_u32((entry.date.year()-minage as u32)*1000+entry.date.monthday())));
    			}
    			else{
	    			if (entry.date.year()  - minage  as u32) < bd1_range.unwrap().1.year(){
	    				bd1_range.unwrap().1 = Date::from_u32((entry.date.year()   - minage as u32)*1000+1231);
	    			}
        			if minage > init_age.unwrap() { //Then we can bound their birthdate from the division
        				if bd2_range.is_none(){
                            bd2_range = Some((Date::from_u32((entry.date.year() -minage as u32)*1000+entry.date.monthday()),Date::from_u32((entry.date.year() -minage as u32)*1000+entry.date.monthday())));
        				}
                        else{
            				if entry.date.monthday() < bd2_range.unwrap().0.monthday() {
                                bd2_range.unwrap().0 = Date::from_u32((entry.date.year()-minage as u32)*1000+entry.date.monthday());
            				}
            				else{
                                bd2_range.unwrap().1 = Date::from_u32((entry.date.year()-minage as u32)*1000+entry.date.monthday());
            				}
                        }
        			}
        			else if minage == init_age.unwrap(){
    					if entry.date.monthday() < bd1_range.unwrap().0.monthday() {
                            bd1_range.unwrap().0 = Date::from_u32((entry.date.year() -minage as u32)*1000+entry.date.monthday());
        				}
        				else{
                            bd1_range.unwrap().1 = Date::from_u32((entry.date.year() -minage as u32)*1000+entry.date.monthday());
        				}
        			}
                }
    		},
    		Age::Approximate(minage) => {
    			if (entry.date.year()   - minage  as u32) < bd1_range.unwrap().1.year(){ // For when a lower bound on the age has already been obtained
                    bd1_range.unwrap().1 = Date::from_u32((entry.date.year() - minage as u32)*1000+1231);
                }
                // Then they must have had their birthday after the init_age entry, but we don't know when
    			if init_age.is_some() && minage  == init_age.unwrap() && bd2_range.is_none(){ 
                    bd1_range =  Some((Date::from_u32((entry.date.year() -minage  as u32-1)*1000+0101),Date::from_u32((entry.date.year() -minage as u32-1)*1000+bd1_range.unwrap().1.monthday())));
	                bd2_range = Some((bd1_range.unwrap().1,Date::from_u32((entry.date.year() -minage  as u32-1)*1000+1231)));
    			}		
    		},
    		Age::None =>(),
    	}

        // Use maxage to tighten our birthdate bound slightly
    	match entry.maxage{ 
    		Age::Exact(maxage) => {
    			if bd1_range.is_none(){
    				bd1_range = Some((Date::from_u32((entry.date.year() -maxage as u32)*1000+entry.date.monthday()),entry.date));
    			}
    			else{
		  			if entry.date.year()  - maxage  as u32> bd1_range.unwrap().0.year(){
	    				bd1_range.unwrap().0 = Date::from_u32((entry.date.year()  - maxage  as u32)*1000+0101);
	    			} 

        			if maxage < init_age.unwrap() { //Then we can bound their birthdate from the division
        				if bd2_range.is_none(){
                            bd2_range = Some((Date::from_u32((entry.date.year() -init_age.unwrap() as u32)*1000+entry.date.monthday()),Date::from_u32((entry.date.year() -init_age.unwrap() as u32)*1000+entry.date.monthday())));
        				}
        				else{
            				if entry.date.monthday() < bd2_range.unwrap().0.monthday() {
                                bd2_range.unwrap().0 = Date::from_u32((entry.date.year()-maxage as u32)*1000+entry.date.monthday());
            				}
            				else{
                                bd2_range.unwrap().1 = Date::from_u32((entry.date.year() -maxage as u32)*1000+entry.date.monthday());
            				}
	        			}
        			}
        			else if maxage == init_age.unwrap(){
    					if entry.date.monthday() < bd1_range.unwrap().0.monthday() {
                            bd1_range.unwrap().0 = Date::from_u32((entry.date.year() -maxage as u32)*1000+entry.date.monthday());
        				}
        				else{
                            bd1_range.unwrap().1 = Date::from_u32((entry.date.year() -maxage as u32)*1000+entry.date.monthday());
        				}
        			}
                }
    		},
			Age::Approximate(maxage) => {
    			if entry.date.year()   - maxage as u32 -1 > bd1_range.unwrap().0.year(){ // For when a lower bound on the age has been obtained from maxage
                    bd1_range.unwrap().0 = Date::from_u32((entry.date.year()  - maxage as u32)*1000+0101);
    			}
    			// Then they must have had their birthday before the init_age entry, but we don't know when
    			if init_age.is_some() && maxage +1 == init_age.unwrap().into() && bd2_range.is_none() { 
                    bd1_range.unwrap().0 =  Date::from_u32((entry.date.year() -maxage as u32)*1000+bd1_range.unwrap().0.monthday());
                    bd1_range.unwrap().1 =  Date::from_u32((entry.date.year() -maxage as u32)*1000+1231);

	                bd2_range = Some((Date::from_u32((entry.date.year()-maxage as u32)*1000+0101),bd1_range.unwrap().0));
    			}    		
    		},
    		Age::None =>(),
    	}
    }



    // Bounded, first age range is before second
    if bd1_range.is_some() && bd2_range.is_some(){
    	if bd1_range.unwrap().1.monthday() < bd2_range.unwrap().0.monthday(){
            return BirthdateConstraint::Bound{min_date: bd1_range.unwrap().1,max_date:bd2_range.unwrap().0};
	    } // Bounded, second age range is before first
	    else{
            return BirthdateConstraint::Bound{min_date:bd2_range.unwrap().1,max_date:bd1_range.unwrap().0};
	    }
    }// Not bounded, return exclusion zone
    else if bd1_range.is_some() {
        return BirthdateConstraint::KnownRegion{min_date:Date::from_u32((bd1_range.unwrap().0.year()-init_age.unwrap() as u32)+bd1_range.unwrap().0.monthday()),
        	max_date:Date::from_u32((bd1_range.unwrap().0.year()-init_age.unwrap() as u32)+bd1_range.unwrap().1.monthday())};
    }

    // We haven't successfully bounded the birthday, return a birthyear range
    if bd1_range.is_some(){
    	return BirthdateConstraint::Bound{min_date:bd1_range.unwrap().0,max_date:bd1_range.unwrap().1};
    }
    BirthdateConstraint::None
 
}


// Check if two AgeData are consistent with one another
// Apologies for how long this function is...
fn are_entries_consistent(entry1 : &AgeData, entry2: &AgeData) -> bool {
    let yd = year_diff(entry1.date,entry2.date);

    // Check that entry1.age is consistent with the data in entry2
    match entry1.age {
        Age::Exact(age1) => {
		    match entry2.age {
	    		Age::Exact(age2) => if (age1 as  i8-age2 as i8).abs() as u8 != yd {return false;},
	    		Age::Approximate(age2) => if (age1 as i8-age2 as i8).abs() as u8 !=yd {return false;},
	    		Age::None =>(),
	    	}
	    	match entry2.minage { 
	    		Age::Exact(minage2)       => if ((age1 as i8-minage2 as i8).abs() as u8) > yd {return false;},
	    		Age::Approximate(minage2) => if ((age1 as i8-minage2 as i8).abs() as u8) > yd {return false;},
	    		Age::None =>(),
	    	}
	    	match entry2.maxage {
	    		Age::Exact(maxage2)       => if ((age1 as i8-maxage2 as i8).abs() as u8) < yd {return false;},
	    		Age::Approximate(maxage2) => if ((age1 as i8-maxage2 as i8).abs() as u8) < yd {return false;},
	    		Age::None =>(),
	    	}
	   	if entry2.birthdate.is_some()  && entry1.age != entry2.birthdate.unwrap().age_on(entry1.date).unwrap() {return false}
	   	if entry2.birthyear.is_some()  && (entry1.date.year() - entry2.birthyear.unwrap()) as u8 != age1 && (entry1.date.year() - entry2.birthyear.unwrap()) as u8 != age1 +1 {return false;}
        },
        Age::Approximate(age1) => {
			match entry2.age {
	    		Age::Exact(age2) => if (age1 as i8-age2 as i8).abs() as u8 != yd {return false;},
	    		Age::Approximate(age2) => if ((age1 as i8-age2 as i8).abs() as u8) != yd {return false;},
	    		Age::None =>(),
	    	}
	    	match entry2.minage {
	    		Age::Exact(minage2)       => if ((age1 as i8-minage2 as i8).abs() as u8) > yd {return false;},
	    		Age::Approximate(minage2) => if ((age1 as i8-minage2 as i8).abs() as u8) > yd {return false;},
	    		Age::None =>(),
	    	}
	    	match entry2.maxage {
	    		Age::Exact(maxage2)       => if ((age1 as i8-maxage2 as i8).abs() as u8) < yd + 1 {return false;},
	    		Age::Approximate(maxage2) => if ((age1 as i8-maxage2 as i8).abs() as u8) < yd {return false;},
	    		Age::None =>(),
	    	}
	   	if entry2.birthdate.is_some(){
	   		let age_on = entry2.birthdate.unwrap().age_on(entry1.date).unwrap().to_u8_option().unwrap();
	   		if age_on != age1 && age_on != age1+1{
	   			return false;
	   		}
	   	} 
	   	if entry2.birthyear.is_some()  && (entry1.date.year() - entry2.birthyear.unwrap()) as u8 != age1 +1 {return false;}
        },
        Age::None =>(),
    }

    // Check that entry1.minage is consistent with the data in entry2
    match entry1.minage{
            Age::Exact(minage1) => {
                match entry2.age{
                	Age::Exact(age2)       => if ((age2 as  i8-minage1 as i8).abs() as u8) > yd {return false;},
                	Age::Approximate(age2) => if ((age2 as i8-minage1 as i8).abs() as  u8) > yd {return false;},
                	Age::None =>(),

                }
                match entry2.maxage{
                	Age::Exact(maxage2) => if entry1.date < entry2.date && minage1+yd > maxage2 {return false;},
                	Age::Approximate(maxage2)=> if entry1.date < entry2.date && minage1 + yd > maxage2 {return false;},
                	Age::None => (),

                }
           	if entry2.birthyear.is_some() && ((entry1.date.year() - entry2.birthyear.unwrap()) as u8) < minage1 {return false;}
           	if entry2.birthdate.is_some()  && entry1.minage > entry2.birthdate.unwrap().age_on(entry1.date).unwrap() {return false;}
            },
            Age::Approximate(minage1) => {
                match entry2.age{
                	Age::Exact(age2)       => if ((age2 as i8-minage1 as i8).abs() as  u8) > yd{return false;},
                	Age::Approximate(age2) => if ((age2 as i8-minage1 as i8).abs() as u8) > yd {return false;},
                	Age::None =>(),

                }
 
                match entry2.maxage{
                	Age::Exact(maxage2) => if entry1.date < entry2.date && minage1+yd -1 > maxage2 {return false;},
                	Age::Approximate(maxage2) => if entry1.date < entry2.date && minage1+yd > maxage2 {return false;},
                	Age::None => (),

                }
           	if entry2.birthyear.is_some() && ((entry1.date.year() - entry2.birthyear.unwrap()) as u8) < minage1 {return false;}
           	if entry2.birthdate.is_some()  && entry1.minage > entry2.birthdate.unwrap().age_on(entry1.date).unwrap() {return false;}
            },
            Age::None =>(),
    }

    // Check that entry1.maxage is consistent with the data in entry2
    match entry1.maxage{
            Age::Exact(maxage1) => {
                match entry2.age{
                	Age::Exact(age2)       => if ((age2 as i8-maxage1 as i8).abs() as u8) < yd {return false;},
                	Age::Approximate(age2) => if ((age2 as i8-maxage1 as i8).abs() as u8) < yd + 1 {return false;},
                	Age::None =>(),

                }
                match entry2.minage{
                	Age::Exact(minage2) => if entry2.date < entry1.date && minage2+yd > maxage1 {return false;},
                	Age::Approximate(minage2)=> if entry2.date < entry1.date && minage2 + yd -1 > maxage1 {return false;},
                	Age::None =>(),

                }

           	if entry2.birthyear.is_some() && (entry1.date.year() - entry2.birthyear.unwrap()-1) as u8 > maxage1 {return false;}
           	if entry2.birthdate.is_some()  && entry1.maxage < entry2.birthdate.unwrap().age_on(entry1.date).unwrap() {return false;}
            },
            Age::Approximate(maxage1) => {
				match entry2.age{
                	Age::Exact(age2)       => if ((age2 as i8-maxage1 as i8).abs() as u8) < yd {return false;},
                	Age::Approximate(age2) => if ((age2 as i8 -maxage1 as i8).abs() as u8) < yd {return false;},
                	Age::None =>(),

                }
                match entry2.minage{
                	Age::Exact(minage2)=> if entry2.date < entry1.date && minage2 + yd > maxage1 {return false;},
                	Age::Approximate(minage2) => if entry2.date < entry1.date && minage2+yd > maxage1 {return false;},
                	Age::None =>(), 

                }

           	if entry2.birthyear.is_some() && (entry1.date.year() - entry2.birthyear.unwrap()-1) as u8 > maxage1 {return false;}
           	if entry2.birthdate.is_some()  && entry1.maxage < entry2.birthdate.unwrap().age_on(entry1.date).unwrap() {return false;}
            },
            Age::None =>(),
    }
    	
    // Check that entry1.birthyear is consistent with the data in entry2
    if entry1.birthyear.is_some() {
    	match entry2.age {
	        Age::Exact(age2)       => if (entry2.date.year() - entry1.birthyear.unwrap()) as u8 != age2 && (entry2.date.year() - entry1.birthyear.unwrap()) as u8 != age2 +1 {return false;}
	        Age::Approximate(age2) => if (entry2.date.year() - entry1.birthyear.unwrap()) as u8 != age2+1 {return false;},
	        Age::None => (),
    	}
    	match entry2.minage {
	        Age::Exact(minage2)       => if ((entry2.date.year() - entry1.birthyear.unwrap()) as u8) < minage2 {return false;},
	        Age::Approximate(minage2) => if ((entry2.date.year() - entry1.birthyear.unwrap()) as u8) < minage2 {return false;},
	        Age::None => (),
    	}
    	match entry2.maxage {
	        Age::Exact(maxage2)       => if ((entry2.date.year() - entry1.birthyear.unwrap()) as u8) > maxage2 {return false;},
	        Age::Approximate(maxage2) => if ((entry2.date.year() - entry1.birthyear.unwrap()-1) as u8) > maxage2 {return false;},
	        Age::None => (),
    	}
    	if entry2.birthyear.is_some() && entry1.birthyear.unwrap() != entry2.birthyear.unwrap() {return false}
    	if entry2.birthdate.is_some() && entry1.birthyear.unwrap() != entry2.birthdate.unwrap().year() {return false}
    }

    // Check that entry1.birthdate is consistent with the data in entry2
    if entry1.birthdate.is_some() {
    	match entry2.age {
	        Age::Exact(age2)       => if entry1.birthdate.unwrap().age_on(entry2.date).unwrap() != entry2.age {return false;}
	        Age::Approximate(age2) =>{
		   		let age_on = entry1.birthdate.unwrap().age_on(entry2.date).unwrap().to_u8_option().unwrap();
		   		if age_on != age2 && age_on != age2+1{
		   			return false;
		   		}
		   	},
		   	Age::None =>(),
		}

    	if entry2.minage != Age::None  && entry1.birthdate.unwrap().age_on(entry2.date).unwrap() < entry2.minage {return false}
    	if entry2.maxage != Age::None && entry1.birthdate.unwrap().age_on(entry2.date).unwrap() > entry2.maxage {return false}
    	if entry2.birthyear.is_some() && entry1.birthdate.unwrap().year() != entry2.birthyear.unwrap() {return false}
    	if entry2.birthdate.is_some() && entry1.birthdate.unwrap() != entry2.birthdate.unwrap() {return false}
    }
  
    true
}



// Check that lifter age data is consistent across several entries
fn is_agedata_consistent(entries: &[AgeData]) -> bool {
	if entries.is_empty() { return true;}

	// This is O(N^2), there is probably a more efficient way if doing this...
	for ii in 0..entries.len(){
		for jj in ii..entries.len(){
			if !are_entries_consistent(&entries[ii],&entries[jj]) { return false;}
		}
	}

	true
}



fn interpolate_array(entries: &mut [AgeData]) {
	let bd_constraint = estimate_birthdate(entries);

	for entry in entries{
		match bd_constraint{
			BirthdateConstraint::Bound{min_date,max_date} =>{

	    		//Then we know the lifters birthyear
	    		if min_date.year() == max_date.year() {
	    			entry.birthyear = Some(min_date.year());
	    			// Then we know the lifters birthdate exactly
	    			if min_date.monthday() == max_date.monthday(){
	    				entry.birthdate = Some(min_date);
	    				entry.age = min_date.age_on(entry.date).unwrap();
	    				entry.minage = entry.age;
	    				entry.maxage = entry.age;
	    			}
	    			else{ //Assign an approximate age range
	    				entry.age = Age::Approximate((max_date.year()-entry.date.year()-1) as u8);
	                    entry.minage = Age::Exact((entry.date.year() - max_date.year()-1) as u8);
	                    entry.maxage = Age::Exact((entry.date.year() - max_date.year()) as u8);
	                }
	    		}
	    		else{ //Assign an age range
	    			entry.minage = Age::Exact((entry.date.year() - max_date.year() -1) as u8);
	    			entry.maxage = Age::Exact((entry.date.year() - min_date.year()) as u8);
	    		}
			},
			BirthdateConstraint::KnownRegion{min_date,max_date} =>{

	            if entry.date.monthday() < min_date.monthday(){ // We have an upper bound on the age
	                 entry.age = Age::Approximate((entry.date.year() - min_date.year()-1) as u8);
	                 entry.minage = Age::Exact((entry.date.year() - min_date.year()-1)as u8);
	                 entry.maxage = Age::Exact((entry.date.year() - min_date.year()) as u8);
	            }
	            else if entry.date.monthday() > max_date.monthday(){ // We have a lower bound on the age
	                 entry.age = Age::Approximate((entry.date.year() - min_date.year())as u8);
	                 entry.minage = Age::Exact((entry.date.year() - min_date.year()) as u8);
	                 entry.minage = Age::Exact((entry.date.year() - min_date.year()+1) as u8);
	            }
	            else{ // We can give the age exactly for this date
	                 entry.minage = Age::Exact((entry.date.year() - min_date.year()) as u8);
	                 entry.minage = entry.age;
	                 entry.maxage = entry.age;
	            }
			},
			BirthdateConstraint::None =>(),
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
    fn test_invalid_exact_age() {
    	// Age <-> Age
        let a = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let b = AgeData { age: Age::Exact(41), minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 56 };
        let interp_arr = [a,b];
        let interp_arr2 = [b,a];

    	// Age <-> Approx Age
        let c = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let d = AgeData { age: Age::Approximate(41), minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 56 };
        let interp_arr3 = [c,d];
        let interp_arr4 = [d,c];

    	// Age <-> Approx Minage
        let e = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let f = AgeData { age: Age::None, minage: Age::Approximate(41), maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 56 };
        let interp_arr5 = [e,f];
        let interp_arr6 = [f,e];

    	// Age <-> Exact Minage
        let g = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let h = AgeData { age: Age::None, minage: Age::Exact(41), maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 56 };
        let interp_arr7 = [g,h];
        let interp_arr8 = [h,g];

    	// Age <-> Approx Maxage
        let i = AgeData { age: Age::Exact(18), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let j = AgeData { age: Age::None, minage: Age::None, maxage: Age::Approximate(40), date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 56 };
        let interp_arr9 = [i,j];
        let interp_arr10 = [j,i];

    	// Age <-> Exact Maxage
        let k = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let l = AgeData { age: Age::None, minage: Age::None, maxage: Age::Exact(40), date: Date::from_u32(20040705),birthyear:None, birthdate: None, linenum: 56 };
        let interp_arr11 = [k,l];
        let interp_arr12 = [l,k];

    	// Age <-> Birthyear
        let m = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let n = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:Some(1964), birthdate: None, linenum: 56 };
        let interp_arr13 = [m,n];
        let interp_arr14 = [n,m];

    	// Age <-> Birthdate
        let o = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let p = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: Some(Date::from_u32(19630705)), linenum: 56 };
        let interp_arr15 = [o,p];
        let interp_arr16 = [p,o];

        assert!(!is_agedata_consistent(&interp_arr));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));
        assert!(!is_agedata_consistent(&interp_arr5));
        assert!(!is_agedata_consistent(&interp_arr6));
        assert!(!is_agedata_consistent(&interp_arr7));
        assert!(!is_agedata_consistent(&interp_arr8));
        assert!(!is_agedata_consistent(&interp_arr9));
        assert!(!is_agedata_consistent(&interp_arr10));
        assert!(!is_agedata_consistent(&interp_arr11));
        assert!(!is_agedata_consistent(&interp_arr12));
        assert!(!is_agedata_consistent(&interp_arr13));
        assert!(!is_agedata_consistent(&interp_arr14));
        assert!(!is_agedata_consistent(&interp_arr15));
        assert!(!is_agedata_consistent(&interp_arr16));

    }



 #[test]
    fn test_invalid_approx_age() {


    	// Age <-> Approx Age
        let a = AgeData { age: Age::Approximate(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let b = AgeData { age: Age::Approximate(41), minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 56 };
        let interp_arr1 = [a,b];
        let interp_arr2 = [b,a];

    	// Age <-> Approx Minage
        let c = AgeData { age: Age::Approximate(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let d = AgeData { age: Age::None, minage: Age::Approximate(41), maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 56 };
        let interp_arr3 = [c,d];
        let interp_arr4 = [d,c];

    	// Age <-> Exact Minage
        let e = AgeData { age: Age::Approximate(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let f = AgeData { age: Age::None, minage: Age::Exact(42), maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 56 };
        let interp_arr5 = [e,f];
        let interp_arr6 = [f,e];

    	// Age <-> Approx Maxage
        let g = AgeData { age: Age::Approximate(18), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let h = AgeData { age: Age::None, minage: Age::None, maxage: Age::Approximate(40), date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 56 };
        let interp_arr7 = [g,h];
        let interp_arr8 = [h,g];

    	// Age <-> Exact Maxage
        let i = AgeData { age: Age::Approximate(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let j = AgeData { age: Age::None, minage: Age::None, maxage: Age::Exact(40), date: Date::from_u32(20040705),birthyear:None, birthdate: None, linenum: 56 };
        let interp_arr9 = [i,j];
        let interp_arr10 = [j,i];

    	// Age <-> Birthyear
        let k = AgeData { age: Age::Approximate(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let l = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:Some(1963), birthdate: None, linenum: 56 };
        let interp_arr11 = [k,l];
        let interp_arr12 = [l,k];

    	// Age <-> Birthdate
        let m = AgeData { age: Age::Approximate(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 7 };
        let n = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: Some(Date::from_u32(19630705)), linenum: 56 };
        let interp_arr13 = [m,n];
        let interp_arr14 = [n,m];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));
        assert!(!is_agedata_consistent(&interp_arr5));
        assert!(!is_agedata_consistent(&interp_arr6));
        assert!(!is_agedata_consistent(&interp_arr7));
        assert!(!is_agedata_consistent(&interp_arr8));
        assert!(!is_agedata_consistent(&interp_arr9));
        assert!(!is_agedata_consistent(&interp_arr10));
        assert!(!is_agedata_consistent(&interp_arr11));
        assert!(!is_agedata_consistent(&interp_arr12));
        assert!(!is_agedata_consistent(&interp_arr13));
        assert!(!is_agedata_consistent(&interp_arr14));

    }


    #[test]
    fn test_invalid_exact_minage() {
    	// Exact Minage <-> Exact Maxage
        let a = AgeData { age: Age::None, minage: Age::Exact(40), maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 53 };
        let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::Exact(53), date: Date::from_u32(20040705),birthyear:None, birthdate: None, linenum: 24 };

        let interp_arr1 = [a,b];
        let interp_arr2 = [b,a];

    	// Exact Minage <-> Approx Maxage
        let c = AgeData { age: Age::None, minage: Age::Exact(40), maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 53 };
        let d = AgeData { age: Age::None, minage: Age::None, maxage: Age::Approximate(52), date: Date::from_u32(20040705),birthyear:None, birthdate: None, linenum: 24 };

        let interp_arr3 = [c,d];
        let interp_arr4 = [d,c];

    	// Exact Minage <-> Birthyear
        let e = AgeData { age: Age::None, minage: Age::Exact(40), maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 53 };
        let f = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040705),birthyear:Some(1941), birthdate: None, linenum: 24 };

        let interp_arr5 = [e,f];
        let interp_arr6 = [f,e];

    	// Exact Minage <-> Birthdate
        let g = AgeData { age: Age::None, minage: Age::Exact(40), maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 53 };
        let h = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040705),birthyear:None, birthdate: Some(Date::from_u32(19400705)), linenum: 24 };

        let interp_arr7 = [g,h];
        let interp_arr8 = [h,g];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));
        assert!(!is_agedata_consistent(&interp_arr5));
        assert!(!is_agedata_consistent(&interp_arr6));
        assert!(!is_agedata_consistent(&interp_arr7));
        assert!(!is_agedata_consistent(&interp_arr8));

    }
    
  #[test]
    fn test_invalid_approx_minage() {
    	// Exact Minage <-> Exact Maxage
        let a = AgeData { age: Age::None, minage: Age::Approximate(40), maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 53 };
        let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::Exact(53), date: Date::from_u32(20040705),birthyear:None, birthdate: None, linenum: 24 };

        let interp_arr1 = [a,b];
        let interp_arr2 = [b,a];

    	// Exact Minage <-> Approx Maxage
        let c = AgeData { age: Age::None, minage: Age::Approximate(40), maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 53 };
        let d = AgeData { age: Age::None, minage: Age::None, maxage: Age::Approximate(53), date: Date::from_u32(20040705),birthyear:None, birthdate: None, linenum: 24 };

        let interp_arr3 = [c,d];
        let interp_arr4 = [d,c];

    	// Exact Minage <-> Birthyear
        let e = AgeData { age: Age::None, minage: Age::Approximate(40), maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 53 };
        let f = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040705),birthyear:Some(1941), birthdate: None, linenum: 24 };

        let interp_arr5 = [e,f];
        let interp_arr6 = [f,e];

    	// Exact Minage <-> Birthdate
        let g = AgeData { age: Age::None, minage: Age::Approximate(40), maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 53 };
        let h = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040705),birthyear:None, birthdate: Some(Date::from_u32(19400705)), linenum: 24 };

        let interp_arr7 = [g,h];
        let interp_arr8 = [h,g];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));
        assert!(!is_agedata_consistent(&interp_arr5));
        assert!(!is_agedata_consistent(&interp_arr6));
        assert!(!is_agedata_consistent(&interp_arr7));
        assert!(!is_agedata_consistent(&interp_arr8));

    }


    #[test]
    fn test_invalid_exact_maxage() {

    	// Exact Maxage <-> Birthyear
        let a = AgeData { age: Age::None, minage: Age::None, maxage: Age::Exact(18), date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 53 };
        let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040705),birthyear:Some(1960), birthdate: None, linenum: 24 };

        let interp_arr1 = [a,b];
        let interp_arr2 = [b,a];

    	// Exact Maxage <-> Birthdate
        let c = AgeData { age: Age::None, minage: Age::None, maxage: Age::Exact(18), date: Date::from_u32(19800705),birthyear:None, birthdate: None, linenum: 53 };
        let d = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040705),birthyear:None, birthdate: Some(Date::from_u32(19610703)), linenum: 24 };

        let interp_arr3 = [c,d];
        let interp_arr4 = [d,c];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));

    }


    #[test]
    fn test_invalid_approx_maxage() {

    	// Approx Maxage <-> Birthyear
        let a = AgeData { age: Age::None, minage: Age::None, maxage: Age::Approximate(18), date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 53 };
        let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040705),birthyear:Some(1960), birthdate: None, linenum: 24 };

        let interp_arr1 = [a,b];
        let interp_arr2 = [b,a];

    	// Approx Maxage <-> Birthdate
        let c = AgeData { age: Age::None, minage: Age::None, maxage: Age::Approximate(18), date: Date::from_u32(19800705),birthyear:None, birthdate: None, linenum: 53 };
        let d = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040705),birthyear:None, birthdate: Some(Date::from_u32(19600703)), linenum: 24 };

        let interp_arr3 = [c,d];
        let interp_arr4 = [d,c];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));

    }

    #[test]
    fn test_invalid_birthyear() {

    	// Birthyear <-> Birthyear
        let a = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:Some(1961), birthdate: None, linenum: 53 };
        let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040705),birthyear:Some(1960), birthdate: None, linenum: 24 };

        let interp_arr1 = [a,b];
        let interp_arr2 = [b,a];

    	// Birthyear <-> Birthdate
        let c = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(19800705),birthyear:Some(1961), birthdate: None, linenum: 53 };
        let d = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040705),birthyear:None, birthdate: Some(Date::from_u32(19600703)), linenum: 24 };

        let interp_arr3 = [c,d];
        let interp_arr4 = [d,c];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));

    }

   #[test]
    fn test_invalid_birthdate() {

    	// Birthdate <-> Birthdate
        let a = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(19800705),birthyear:None, birthdate: Some(Date::from_u32(19600704)), linenum: 53 };
        let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040705),birthyear:None, birthdate: Some(Date::from_u32(19600703)), linenum: 24 };

        let interp_arr1 = [a,b];
        let interp_arr2 = [b,a];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));

    }

    #[test]
    fn test_valid_justages() {
        let a = AgeData { age: Age::Exact(20), minage: Age::None, maxage: Age::None, date: Date::from_u32(20001231),birthyear:None, birthdate: None, linenum: 100 };
        let b = AgeData { age: Age::Exact(24), minage: Age::None, maxage: Age::None, date: Date::from_u32(20041231),birthyear:None, birthdate: None, linenum: 53 };
        let c = AgeData { age: Age::Exact(29), minage: Age::None, maxage: Age::None, date: Date::from_u32(20091231),birthyear:None, birthdate: None, linenum: 24 };

        let interp_arr = [a,b,c];

        assert!(is_agedata_consistent(&interp_arr));

    }

    // #[test]
    // fn test_alldata() {
    //     let a = AgeData { age: Age::Exact(20), minage: Age::Exact(19), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 1000 };
    //     let b = AgeData { age: Age::Exact(21), minage: Age::Exact(20), maxage: Age::Exact(21), date: Date::from_u32(20011231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };

    //     let c = AgeData { age: Age::Exact(20), minage: Age::Exact(19), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 1000 };
    //     let d = AgeData { age: Age::Exact(21), minage: Age::Exact(20), maxage: Age::Exact(21), date: Date::from_u32(20011231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };


    //     let mut interp_arr = [a,b];
    //     let old_arr = [c,d];

    //     interpolate(&mut interp_arr);

    //     assert!(interp_arr.iter().eq(old_arr.iter()));
    // }

    // // #[test]
    // // fn test_basic_interp() {
    // //     let a = AgeData { age: Age::Exact(20), minage: Age::Exact(20), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };
    // //     let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20011231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };

    // //     let c = AgeData { age: Age::Exact(20), minage: Age::Exact(20), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };
    // //     let d = AgeData { age: Age::Exact(21), minage: Age::Exact(21), maxage: Age::Exact(21), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };


    // //     let mut interp_arr = [a,b];
    // //     let old_arr = [c,d];

    // //     interpolate(&mut interp_arr);

    // //     assert!(interp_arr.iter().eq(old_arr.iter()));
    // // }

    // #[test]
    // fn test_nodata() {
    //     let a = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20001231),birthyear:None, birthdate: None, linenum: 1000 };
    //     let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20011231),birthyear:None, birthdate: None, linenum: 2000 };

    //     let c = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20001231),birthyear:None, birthdate: None, linenum: 1000 };
    //     let d = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20011231),birthyear:None, birthdate: None, linenum: 2000 };


    //     let mut interp_arr = [a,b];
    //     let old_arr = [c,d];

    //     interpolate(&mut interp_arr);

    //     assert!(interp_arr.iter().eq(old_arr.iter()));
    // }
}