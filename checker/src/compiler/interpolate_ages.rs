

use opltypes::*;
use std::cmp::Ordering;
use std::fmt;
use std::cmp;

use crate::check_entries::Entry;
extern crate permutohedron;


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
#[derive(Copy,Clone,Debug,PartialEq, Eq)]
enum BirthdateConstraint{
    // Region that a lifters birthdate lies in
    Bound {min_date: Date, max_date: Date},

    //Region in which we know that the lifter was 0
    KnownRegion {min_date: Date, max_date: Date},

    // No known age information
    None,
}

impl fmt::Display for BirthdateConstraint {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                        BirthdateConstraint::Bound{min_date,max_date} => write!(f, "{}-{}", min_date, max_date),
                        BirthdateConstraint::KnownRegion{min_date,max_date} => write!(f, "{}-{}", min_date, max_date),
                        BirthdateConstraint::None => Ok(()),
                }
        }
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
    //bd_range = [Min Date,Max Date,Age at min,Age at max]
    let mut bd_range : Option<(Date,Date,u8,u8)> = None;
    let mut known_range : Option<(Date,Date,u8)> = None;

    //So we can compare ages as if they were from the same year
    let mut reference_year : Option<u32> = None;

    for entry in entries{
            // If the lifter has a recorded birthdate use that
        if entry.birthdate.is_some(){
            return BirthdateConstraint::Bound{min_date:entry.birthdate.unwrap(), max_date: entry.birthdate.unwrap()};
        }

        if entry.birthyear.is_some(){
            bd_range = Some((Date::from_u32(entry.birthyear.unwrap()*100_00+0101),
                              Date::from_u32(entry.birthyear.unwrap()*100_00+1231),
                              (entry.date.year()-entry.birthyear.unwrap()) as u8 -1,
                              (entry.date.year()-entry.birthyear.unwrap()) as u8));
            reference_year = Some(entry.date.year());
        }

            // Use age to tighten our birthdate bound
        match entry.age{
            Age::Exact(age) => {
                let shifted_age :u8;
                let shifted_date: Date;

                //Shift the age and date to be from reference_year
                if reference_year.is_some(){
                    shifted_age = (age as i32 + (reference_year.unwrap() as i32- entry.date.year() as i32)) as u8;
                    shifted_date = Date::from_u32(reference_year.unwrap()*100_00+entry.date.monthday());
                }
                else{
                    reference_year = Some(entry.date.year());
                    shifted_age = age;
                    shifted_date = entry.date;
                }



                // Then we can update the range in which we know their age
                if known_range.is_none(){
                        known_range = Some((shifted_date,shifted_date,shifted_age));
                }
                else{

                    // Shift date again to try and line it up with the known_range
                    let known_shifted_date = match known_range.unwrap().2 {
                        known_age if known_age < shifted_age => Date::from_u32((shifted_date.year()-1)*100_00+shifted_date.monthday()),
                        known_age if known_age > shifted_age => Date::from_u32((shifted_date.year()+1)*100_00+shifted_date.monthday()),
                        _=> shifted_date, 
                    };

            
                    if known_shifted_date < known_range.unwrap().0{
                            known_range = Some((known_shifted_date,known_range.unwrap().1,known_range.unwrap().2));
                    }
                    else if known_shifted_date > known_range.unwrap().1{
                                known_range = Some((known_range.unwrap().0,known_shifted_date,known_range.unwrap().2));
                    }


                    if bd_range.is_none() {
                        //We've detected an age change within a year, now we can bound the birthdate
                        
                         if shifted_age == known_range.unwrap().2 -1{ // We've found an age younger than the prior ages
                            bd_range = Some((shifted_date,known_range.unwrap().0,shifted_age,shifted_age+1));
                        }
                        else if shifted_age == known_range.unwrap().2 +1 {
                            bd_range = Some((known_range.unwrap().1,shifted_date,shifted_age-1,shifted_age));

                        }
                    }
                }
                            
                // If we have an existing bd_range created from approx_age/maxage/minage we want to update both known_range & bd_range
                if bd_range.is_some(){ 

                    // Ages & dates shifted relative to the existing bd_range
                    let mut range_shifted_min :Date = shifted_date; 
                    let mut range_shifted_max :Date = shifted_date;

                    let mut range_shifted_age_min =shifted_age;
                    let mut range_shifted_age_max =shifted_age;

                    if shifted_age > bd_range.unwrap().2 { //if we're greater than the lower bound, try subtracting a year
                        range_shifted_min = Date::from_u32((shifted_date.year()-1)*100_00+shifted_date.monthday());
                        range_shifted_age_min = range_shifted_age_min - 1;
                    }
                    else if shifted_age < bd_range.unwrap().3{//if we're smaller than the upper bound, try adding a year
                        range_shifted_max = Date::from_u32((shifted_date.year()+1)*100_00+shifted_date.monthday());
                        range_shifted_age_max = range_shifted_age_max + 1;
                    }



                    if range_shifted_min > bd_range.unwrap().0 && range_shifted_age_min >= bd_range.unwrap().2{
                      bd_range = Some((range_shifted_min,bd_range.unwrap().1,range_shifted_age_min,bd_range.unwrap().3));
                    }
                    else if range_shifted_max < bd_range.unwrap().1 && range_shifted_age_max >= bd_range.unwrap().3{
                      bd_range = Some((bd_range.unwrap().0,range_shifted_max,bd_range.unwrap().2,range_shifted_age_max));
                    }

                }


                         
        },
        Age::Approximate(age) => { 


            let shifted_age :u8;
            let shifted_date: Date;

            //Shift the age and date to be from reference_year
            if reference_year.is_some(){
                shifted_age = (age as i32 + (reference_year.unwrap() as i32- entry.date.year() as i32)) as u8;
                shifted_date = Date::from_u32(reference_year.unwrap()*100_00+entry.date.monthday());
            }
            else{
                reference_year = Some(entry.date.year());
                shifted_age = age;
                shifted_date = entry.date;
            }

            let bd_min = Date::from_u32(reference_year.unwrap()*100_00+0101);
            let bd_max = Date::from_u32(reference_year.unwrap()*100_00+1231);

            if bd_range.is_none(){

                if known_range.is_none(){                
                        bd_range = Some((bd_min,bd_max,shifted_age,shifted_age+1)); 
                }
                else{

                    // Shift date again to try and line it up with the known_range
                    let known_shifted_date = match known_range.unwrap().2 {
                        known_age if known_age < shifted_age => Date::from_u32((shifted_date.year()-1)*100_00+shifted_date.monthday()),
                        known_age if known_age > shifted_age => Date::from_u32((shifted_date.year()+1)*100_00+shifted_date.monthday()),
                        _=> shifted_date, 
                    };
                    
                    
                    //then they haven't had their birthday yet in the known region
                    if shifted_age == known_range.unwrap().2{

                        bd_range = Some((known_range.unwrap().1,bd_max,shifted_age,shifted_age+1));
                    }
                    else{ //They've had their birthday
                        bd_range = Some((bd_min,known_range.unwrap().0,shifted_age,shifted_age+1));
                    }
                }
            }
            else{ //If the exisiting range is obtained from maxage or minage we can tighten the bound

                //need to update the age here also
                if bd_range.unwrap().0 < bd_min{
                        bd_range = Some((bd_min,bd_range.unwrap().1,bd_range.unwrap().2,bd_range.unwrap().3));
                }
                
                if bd_range.unwrap().1 > bd_max{
                        bd_range = Some((bd_range.unwrap().0,bd_max,bd_range.unwrap().2,bd_range.unwrap().3));
                }

            }
            
        },
        Age::None =>(),
         }

            // Use minage to tighten our birthdate bound slightly
        match entry.minage{ 
            Age::Exact(minage) => {
                if bd_range.is_none(){
                    bd_range = Some((Date::from_u32(00000101),
                        Date::from_u32((entry.date.year()-minage as u32)*100_00+entry.date.monthday()),
                        255,
                        minage));
                }
                else{
        //                 if (entry.date.year()    - minage    as u32) < bd1_range.unwrap().1.year(){
        //                     bd1_range.unwrap().1 = Date::from_u32((entry.date.year()     - minage as u32)*100_00+1231);
        //                 }
     //                        if minage > init_age.unwrap() { //Then we can bound their birthdate from the division
     //                            if bd2_range.is_none(){
     //                                                    bd2_range = Some((Date::from_u32((entry.date.year() -minage as u32)*100_00+entry.date.monthday()),Date::from_u32((entry.date.year() -minage as u32)*100_00+entry.date.monthday())));
     //                            }
     //                                            else{
     //                                    if entry.date.monthday() < bd2_range.unwrap().0.monthday() {
     //                                                            bd2_range = Some((Date::from_u32((entry.date.year()-minage as u32)*100_00+entry.date.monthday()),bd2_range.unwrap().1));
     //                                    }
     //                                    else{
     //                                                            bd2_range = Some((bd2_range.unwrap().0,Date::from_u32((entry.date.year()-minage as u32)*100_00+entry.date.monthday())));
     //                                    }
     //                                            }
     //                        }
     //                        else if minage == init_age.unwrap(){
     //                        if entry.date.monthday() < bd1_range.unwrap().0.monthday() {
     //                                                    bd1_range = Some((Date::from_u32((entry.date.year()-minage as u32)*100_00+entry.date.monthday()),bd1_range.unwrap().1));
     //                            }
     //                            else{
     //                                                    bd2_range = Some((bd2_range.unwrap().0,Date::from_u32((entry.date.year()-minage as u32)*100_00+entry.date.monthday())));
     //                            }
     //                        }
                                }
                },
                Age::Approximate(minage) => {
     //                if bd1_range.is_none(){
     //                    bd1_range = Some((Date::from_u32(00000101),Date::from_u32((entry.date.year()-minage as u32 -1)*100_00+1231)));
     //                                    init_age = Some(minage+1);
     //                }
     //                if (entry.date.year()     - minage    as u32 -1) < bd1_range.unwrap().1.year() && bd2_range.is_some(){ // For when a lower bound on the age has already been obtained
     //                                    bd1_range = Some((bd2_range.unwrap().0,Date::from_u32((entry.date.year() - minage as u32 -1)*100_00+1231)));
     //                            }
     //                            // Then they must have had their birthday after the init_age entry, but we don't know when
     //                if init_age.is_some() && minage    == init_age.unwrap() && bd2_range.is_none(){ 
     //                                    bd1_range =    Some((Date::from_u32((entry.date.year() -minage    as u32-1)*100_00+0101),Date::from_u32((entry.date.year() -minage as u32-1)*100_00+bd1_range.unwrap().1.monthday())));
        //                             bd2_range = Some((bd1_range.unwrap().1,Date::from_u32((entry.date.year() -minage    as u32-1)*100_00+1231)));
     //                }        
                },
                Age::None =>(),
            }

     //            // Use maxage to tighten our birthdate bound slightly
     //        match entry.maxage{ 
     //            Age::Exact(maxage) => {
     //                if bd1_range.is_none(){
     //                    bd1_range = Some((Date::from_u32((entry.date.year() -maxage as u32)*100_00+entry.date.monthday()),entry.date));
     //                                    init_age = Some(maxage);
     //                }
     //                else{
         //                if entry.date.year()    - maxage    as u32> bd1_range.unwrap().0.year(){
        //                     bd1_range = Some((Date::from_u32((entry.date.year()    - maxage    as u32)*100_00+0101),bd1_range.unwrap().1));
        //                 } 

     //                        if maxage < init_age.unwrap() { //Then we can bound their birthdate from the division
     //                            if bd2_range.is_none(){
     //                                                    bd2_range = Some((Date::from_u32((entry.date.year() -init_age.unwrap() as u32)*100_00+entry.date.monthday()),Date::from_u32((entry.date.year() -init_age.unwrap() as u32)*100_00+entry.date.monthday())));
     //                            }
     //                            else{
     //                                    if entry.date.monthday() < bd2_range.unwrap().0.monthday() {
     //                                                            bd2_range = Some((Date::from_u32((entry.date.year()-maxage as u32)*100_00+entry.date.monthday()),bd2_range.unwrap().1));
     //                                    }
     //                                    else{
     //                                                            bd2_range = Some((bd2_range.unwrap().0,Date::from_u32((entry.date.year() -maxage as u32)*100_00+entry.date.monthday())));
     //                                    }
        //                         }
     //                        }
     //                        else if maxage == init_age.unwrap(){
     //                        if entry.date.monthday() < bd1_range.unwrap().0.monthday() {
     //                                                    bd1_range = Some((Date::from_u32((entry.date.year() -maxage as u32)*100_00+entry.date.monthday()),bd1_range.unwrap().1));
     //                            }
     //                            else{
     //                                                    bd1_range = Some((bd1_range.unwrap().0,Date::from_u32((entry.date.year() -maxage as u32)*100_00+entry.date.monthday())));
     //                            }
     //                        }
     //                            }
     //            },
            // Age::Approximate(maxage) => {
            //     if bd1_range.is_none(){
     //                    bd1_range = Some((Date::from_u32((entry.date.year() -maxage as u32 -1)*100_00+entry.date.monthday()),entry.date));
     //                                    init_age = Some(maxage);
     //                }
     //                if entry.date.year()     - maxage as u32 -1 > bd1_range.unwrap().0.year(){ // For when a lower bound on the age has been obtained from maxage
     //                                    bd1_range = Some((Date::from_u32((entry.date.year()    - maxage as u32)*100_00+0101),bd1_range.unwrap().1));
     //                }
     //                // Then they must have had their birthday before the init_age entry, but we don't know when
     //                if init_age.is_some() && maxage +1 == init_age.unwrap().into() && bd2_range.is_none() { 
     //                                    bd1_range =    Some((Date::from_u32((entry.date.year() -maxage as u32)*100_00+bd1_range.unwrap().0.monthday()),Date::from_u32((entry.date.year() -maxage as u32)*100_00+1231)));
        //                             bd2_range = Some((Date::from_u32((entry.date.year()-maxage as u32)*100_00+0101),bd1_range.unwrap().0));
     //                }                
     //            },
     //            Age::None =>(),
     //        }
        }
                // return BirthdateConstraint::Bound{min_date: Date::from_u32(bd_range.unwrap().2 as u32),
                //                                                                             max_date: Date::from_u32(bd_range.unwrap().3 as u32)};   
                // return BirthdateConstraint::Bound{min_date: bd_range.unwrap().0,
                //                                                                             max_date: bd_range.unwrap().1};        
        // Bounded, first age range is before second
        if bd_range.is_some(){
                return BirthdateConstraint::Bound{min_date: Date::from_u32((bd_range.unwrap().0.year()-bd_range.unwrap().3 as u32)*100_00+bd_range.unwrap().0.monthday()),
                                                                                            max_date: Date::from_u32((bd_range.unwrap().1.year()-bd_range.unwrap().3 as u32)*100_00+bd_range.unwrap().1.monthday())};        }
        else if known_range.is_some() {// Not bounded, return exclusion zone
                return BirthdateConstraint::KnownRegion{min_date: Date::from_u32((known_range.unwrap().0.year()-known_range.unwrap().2 as u32)*100_00+known_range.unwrap().0.monthday()),
                                                                                            max_date: Date::from_u32((known_range.unwrap().1.year()-known_range.unwrap().2 as u32)*100_00+known_range.unwrap().1.monthday())};
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
                    Age::Exact(age2) => if (age1 as    i8-age2 as i8).abs() as u8 != yd {return false;},
                    Age::Approximate(age2) => if (age1 as i8-age2 as i8).abs() as u8 !=yd {return false;},
                    Age::None =>(),
                }
                match entry2.minage { 
                    Age::Exact(minage2)             => if ((age1 as i8-minage2 as i8).abs() as u8) > yd {return false;},
                    Age::Approximate(minage2) => if ((age1 as i8-minage2 as i8).abs() as u8) > yd {return false;},
                    Age::None =>(),
                }
                match entry2.maxage {
                    Age::Exact(maxage2)             => if ((age1 as i8-maxage2 as i8).abs() as u8) < yd {return false;},
                    Age::Approximate(maxage2) => if ((age1 as i8-maxage2 as i8).abs() as u8) < yd {return false;},
                    Age::None =>(),
                }
             if entry2.birthdate.is_some()    && entry1.age != entry2.birthdate.unwrap().age_on(entry1.date).unwrap() {return false}
             if entry2.birthyear.is_some()    && (entry1.date.year() - entry2.birthyear.unwrap()) as u8 != age1 && (entry1.date.year() - entry2.birthyear.unwrap()) as u8 != age1 +1 {return false;}
                },
                Age::Approximate(age1) => {
            match entry2.age {
                    Age::Exact(age2) => if (age1 as i8-age2 as i8).abs() as u8 != yd {return false;},
                    Age::Approximate(age2) => if ((age1 as i8-age2 as i8).abs() as u8) != yd {return false;},
                    Age::None =>(),
                }
                match entry2.minage {
                    Age::Exact(minage2)             => if ((age1 as i8-minage2 as i8).abs() as u8) > yd {return false;},
                    Age::Approximate(minage2) => if ((age1 as i8-minage2 as i8).abs() as u8) > yd {return false;},
                    Age::None =>(),
                }
                match entry2.maxage {
                    Age::Exact(maxage2)             => if ((age1 as i8-maxage2 as i8).abs() as u8) < yd + 1 {return false;},
                    Age::Approximate(maxage2) => if ((age1 as i8-maxage2 as i8).abs() as u8) < yd {return false;},
                    Age::None =>(),
                }
             if entry2.birthdate.is_some(){
                 let age_on = entry2.birthdate.unwrap().age_on(entry1.date).unwrap().to_u8_option().unwrap();
                 if age_on != age1 && age_on != age1+1{
                     return false;
                 }
             } 
             if entry2.birthyear.is_some()    && (entry1.date.year() - entry2.birthyear.unwrap()) as u8 != age1 +1 {return false;}
                },
                Age::None =>(),
        }

        // Check that entry1.minage is consistent with the data in entry2
        match entry1.minage{
                        Age::Exact(minage1) => {
                                match entry2.age{
                                    Age::Exact(age2)             => if ((age2 as    i8-minage1 as i8).abs() as u8) > yd {return false;},
                                    Age::Approximate(age2) => if ((age2 as i8-minage1 as i8).abs() as    u8) > yd {return false;},
                                    Age::None =>(),

                                }
                                match entry2.maxage{
                                    Age::Exact(maxage2) => if entry1.date < entry2.date && minage1+yd > maxage2 {return false;},
                                    Age::Approximate(maxage2)=> if entry1.date < entry2.date && minage1 + yd > maxage2 {return false;},
                                    Age::None => (),

                                }
                         if entry2.birthyear.is_some() && ((entry1.date.year() - entry2.birthyear.unwrap()) as u8) < minage1 {return false;}
                         if entry2.birthdate.is_some()    && entry1.minage > entry2.birthdate.unwrap().age_on(entry1.date).unwrap() {return false;}
                        },
                        Age::Approximate(minage1) => {
                                match entry2.age{
                                    Age::Exact(age2)             => if ((age2 as i8-minage1 as i8).abs() as    u8) > yd{return false;},
                                    Age::Approximate(age2) => if ((age2 as i8-minage1 as i8).abs() as u8) > yd {return false;},
                                    Age::None =>(),

                                }
 
                                match entry2.maxage{
                                    Age::Exact(maxage2) => if entry1.date < entry2.date && minage1+yd -1 > maxage2 {return false;},
                                    Age::Approximate(maxage2) => if entry1.date < entry2.date && minage1+yd > maxage2 {return false;},
                                    Age::None => (),

                                }
                         if entry2.birthyear.is_some() && ((entry1.date.year() - entry2.birthyear.unwrap()) as u8) < minage1 {return false;}
                         if entry2.birthdate.is_some()    && entry1.minage > entry2.birthdate.unwrap().age_on(entry1.date).unwrap() {return false;}
                        },
                        Age::None =>(),
        }

        // Check that entry1.maxage is consistent with the data in entry2
        match entry1.maxage{
                        Age::Exact(maxage1) => {
                                match entry2.age{
                                    Age::Exact(age2)             => if ((age2 as i8-maxage1 as i8).abs() as u8) < yd {return false;},
                                    Age::Approximate(age2) => if ((age2 as i8-maxage1 as i8).abs() as u8) < yd + 1 {return false;},
                                    Age::None =>(),

                                }
                                match entry2.minage{
                                    Age::Exact(minage2) => if entry2.date < entry1.date && minage2+yd > maxage1 {return false;},
                                    Age::Approximate(minage2)=> if entry2.date < entry1.date && minage2 + yd -1 > maxage1 {return false;},
                                    Age::None =>(),

                                }

                         if entry2.birthyear.is_some() && (entry1.date.year() - entry2.birthyear.unwrap()-1) as u8 > maxage1 {return false;}
                         if entry2.birthdate.is_some()    && entry1.maxage < entry2.birthdate.unwrap().age_on(entry1.date).unwrap() {return false;}
                        },
                        Age::Approximate(maxage1) => {
                match entry2.age{
                                    Age::Exact(age2)             => if ((age2 as i8-maxage1 as i8).abs() as u8) < yd {return false;},
                                    Age::Approximate(age2) => if ((age2 as i8 -maxage1 as i8).abs() as u8) < yd {return false;},
                                    Age::None =>(),

                                }
                                match entry2.minage{
                                    Age::Exact(minage2)=> if entry2.date < entry1.date && minage2 + yd > maxage1 {return false;},
                                    Age::Approximate(minage2) => if entry2.date < entry1.date && minage2+yd > maxage1 {return false;},
                                    Age::None =>(), 

                                }

                         if entry2.birthyear.is_some() && (entry1.date.year() - entry2.birthyear.unwrap()-1) as u8 > maxage1 {return false;}
                         if entry2.birthdate.is_some()    && entry1.maxage < entry2.birthdate.unwrap().age_on(entry1.date).unwrap() {return false;}
                        },
                        Age::None =>(),
        }
            
        // Check that entry1.birthyear is consistent with the data in entry2
        if entry1.birthyear.is_some() {
            match entry2.age {
                    Age::Exact(age2)             => if (entry2.date.year() - entry1.birthyear.unwrap()) as u8 != age2 && (entry2.date.year() - entry1.birthyear.unwrap()) as u8 != age2 +1 {return false;}
                    Age::Approximate(age2) => if (entry2.date.year() - entry1.birthyear.unwrap()) as u8 != age2+1 {return false;},
                    Age::None => (),
            }
            match entry2.minage {
                    Age::Exact(minage2)             => if ((entry2.date.year() - entry1.birthyear.unwrap()) as u8) < minage2 {return false;},
                    Age::Approximate(minage2) => if ((entry2.date.year() - entry1.birthyear.unwrap()) as u8) < minage2 {return false;},
                    Age::None => (),
            }
            match entry2.maxage {
                    Age::Exact(maxage2)             => if ((entry2.date.year() - entry1.birthyear.unwrap()) as u8) > maxage2 {return false;},
                    Age::Approximate(maxage2) => if ((entry2.date.year() - entry1.birthyear.unwrap()-1) as u8) > maxage2 {return false;},
                    Age::None => (),
            }
            if entry2.birthyear.is_some() && entry1.birthyear.unwrap() != entry2.birthyear.unwrap() {return false}
            if entry2.birthdate.is_some() && entry1.birthyear.unwrap() != entry2.birthdate.unwrap().year() {return false}
        }

        // Check that entry1.birthdate is consistent with the data in entry2
        if entry1.birthdate.is_some() {
            match entry2.age {
                    Age::Exact(_age2)             => if entry1.birthdate.unwrap().age_on(entry2.date).unwrap() != entry2.age {return false;}
                    Age::Approximate(age2) =>{
                     let age_on = entry1.birthdate.unwrap().age_on(entry2.date).unwrap().to_u8_option().unwrap();
                     if age_on != age2 && age_on != age2+1{
                         return false;
                     }
                 },
                 Age::None =>(),
        }

            if entry2.minage != Age::None    && entry1.birthdate.unwrap().age_on(entry2.date).unwrap() < entry2.minage {return false}
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

        //asserts that all permutation of an array give the same birthdate constraint
        //This is a super crappy way of doing this, write something better :P
        fn all_permutation_bd_equal(entries:    &[AgeData],bound: BirthdateConstraint) ->bool {
            let mut entries_copy = entries.to_vec().clone();
            let mut hasfailed = false;
         
            permute_bd_equal(&mut entries_copy,bound,entries.len(),&mut hasfailed);
            if hasfailed{
                return false;
            }
            true
        }


        // Generating permutation using Heap Algorithm 
fn permute_bd_equal(entries:    &mut [AgeData], bound: BirthdateConstraint, n:usize, hasfailed:    &mut bool ){
        // if we are at the bottom of the permutation, check if this gives the correct bound
        if n == 1 { 
            if estimate_birthdate(entries) != bound{
                *hasfailed = true;
            }        
        } 
        else{
            
                for ii in 0..n { 
                        permute_bd_equal(entries,bound,n-1,hasfailed); 
            
                        // if n is odd, swap first and last 
                        // element 
                        if n%2==1 {
                            let temp: AgeData = entries[n-1];

                        entries[n-1] = entries[0];
                        entries[0] = temp;         
                    }
                        else{ // If n is even, swap ith and last element 
                                let temp: AgeData = entries[n-1];

                        entries[n-1] = entries[ii];
                        entries[ii] = temp;            
                        }
                }
        } 
}




        #[test]
        fn test_invalid_exact_age() {
            // Age <-> Age
                let a = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let b = AgeData { age: Age::Exact(41), minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 100 };
                let interp_arr = [a,b];
                let interp_arr2 = [b,a];

            // Age <-> Approx Age
                let c = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let d = AgeData { age: Age::Approximate(41), minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 100 };
                let interp_arr3 = [c,d];
                let interp_arr4 = [d,c];

            // Age <-> Approx Minage
                let e = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let f = AgeData { age: Age::None, minage: Age::Approximate(41), maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 100 };
                let interp_arr5 = [e,f];
                let interp_arr6 = [f,e];

            // Age <-> Exact Minage
                let g = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let h = AgeData { age: Age::None, minage: Age::Exact(41), maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 100 };
                let interp_arr7 = [g,h];
                let interp_arr8 = [h,g];

            // Age <-> Approx Maxage
                let i = AgeData { age: Age::Exact(18), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let j = AgeData { age: Age::None, minage: Age::None, maxage: Age::Approximate(40), date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 100 };
                let interp_arr9 = [i,j];
                let interp_arr10 = [j,i];

            // Age <-> Exact Maxage
                let k = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let l = AgeData { age: Age::None, minage: Age::None, maxage: Age::Exact(40), date: Date::from_u32(20040705),birthyear:None, birthdate: None, linenum: 100 };
                let interp_arr11 = [k,l];
                let interp_arr12 = [l,k];

            // Age <-> Birthyear
                let m = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let n = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:Some(1964), birthdate: None, linenum: 100 };
                let interp_arr13 = [m,n];
                let interp_arr14 = [n,m];

            // Age <-> Birthdate
                let o = AgeData { age: Age::Exact(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let p = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: Some(Date::from_u32(19630705)), linenum: 100 };
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
                let a = AgeData { age: Age::Approximate(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let b = AgeData { age: Age::Approximate(41), minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 100 };
                let interp_arr1 = [a,b];
                let interp_arr2 = [b,a];

            // Age <-> Approx Minage
                let c = AgeData { age: Age::Approximate(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let d = AgeData { age: Age::None, minage: Age::Approximate(41), maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 100 };
                let interp_arr3 = [c,d];
                let interp_arr4 = [d,c];

            // Age <-> Exact Minage
                let e = AgeData { age: Age::Approximate(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let f = AgeData { age: Age::None, minage: Age::Exact(42), maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 100 };
                let interp_arr5 = [e,f];
                let interp_arr6 = [f,e];

            // Age <-> Approx Maxage
                let g = AgeData { age: Age::Approximate(18), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let h = AgeData { age: Age::None, minage: Age::None, maxage: Age::Approximate(40), date: Date::from_u32(20040605),birthyear:None, birthdate: None, linenum: 100 };
                let interp_arr7 = [g,h];
                let interp_arr8 = [h,g];

            // Age <-> Exact Maxage
                let i = AgeData { age: Age::Approximate(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let j = AgeData { age: Age::None, minage: Age::None, maxage: Age::Exact(40), date: Date::from_u32(20040705),birthyear:None, birthdate: None, linenum: 100 };
                let interp_arr9 = [i,j];
                let interp_arr10 = [j,i];

            // Age <-> Birthyear
                let k = AgeData { age: Age::Approximate(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let l = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:Some(1963), birthdate: None, linenum: 100 };
                let interp_arr11 = [k,l];
                let interp_arr12 = [l,k];

            // Age <-> Birthdate
                let m = AgeData { age: Age::Approximate(17), minage: Age::None, maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let n = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040605),birthyear:None, birthdate: Some(Date::from_u32(19630705)), linenum: 100 };
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
                let a = AgeData { age: Age::None, minage: Age::Exact(40), maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::Exact(53), date: Date::from_u32(20040705),birthyear:None, birthdate: None, linenum: 100 };

                let interp_arr1 = [a,b];
                let interp_arr2 = [b,a];

            // Exact Minage <-> Approx Maxage
                let c = AgeData { age: Age::None, minage: Age::Exact(40), maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let d = AgeData { age: Age::None, minage: Age::None, maxage: Age::Approximate(52), date: Date::from_u32(20040705),birthyear:None, birthdate: None, linenum: 100 };

                let interp_arr3 = [c,d];
                let interp_arr4 = [d,c];

            // Exact Minage <-> Birthyear
                let e = AgeData { age: Age::None, minage: Age::Exact(40), maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let f = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040705),birthyear:Some(1941), birthdate: None, linenum: 100 };

                let interp_arr5 = [e,f];
                let interp_arr6 = [f,e];

            // Exact Minage <-> Birthdate
                let g = AgeData { age: Age::None, minage: Age::Exact(40), maxage: Age::None, date: Date::from_u32(19800703),birthyear:None, birthdate: None, linenum: 100 };
                let h = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20040705),birthyear:None, birthdate: Some(Date::from_u32(19400705)), linenum: 100 };

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
                let a = AgeData { age: Age::None, minage: Age::Approximate(40), maxage: Age::None, date: Date::from_u32(1980_07_03),birthyear:None, birthdate: None, linenum: 100 };
                let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::Exact(53), date: Date::from_u32(2004_07_05),birthyear:None, birthdate: None, linenum: 100 };

                let interp_arr1 = [a,b];
                let interp_arr2 = [b,a];

            // Exact Minage <-> Approx Maxage
                let c = AgeData { age: Age::None, minage: Age::Approximate(40), maxage: Age::None, date: Date::from_u32(1980_07_03),birthyear:None, birthdate: None, linenum: 100 };
                let d = AgeData { age: Age::None, minage: Age::None, maxage: Age::Approximate(53), date: Date::from_u32(2004_07_05),birthyear:None, birthdate: None, linenum: 100 };

                let interp_arr3 = [c,d];
                let interp_arr4 = [d,c];

            // Exact Minage <-> Birthyear
                let e = AgeData { age: Age::None, minage: Age::Approximate(40), maxage: Age::None, date: Date::from_u32(1980_07_03),birthyear:None, birthdate: None, linenum: 100 };
                let f = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2004_07_05),birthyear:Some(1941), birthdate: None, linenum: 100 };

                let interp_arr5 = [e,f];
                let interp_arr6 = [f,e];

            // Exact Minage <-> Birthdate
                let g = AgeData { age: Age::None, minage: Age::Approximate(40), maxage: Age::None, date: Date::from_u32(1980_07_03),birthyear:None, birthdate: None, linenum: 100 };
                let h = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2004_07_05),birthyear:None, birthdate: Some(Date::from_u32(1940_07_05)), linenum: 100 };

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
                let a = AgeData { age: Age::None, minage: Age::None, maxage: Age::Exact(18), date: Date::from_u32(1980_07_03),birthyear:None, birthdate: None, linenum: 100 };
                let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2004_07_05),birthyear:Some(1960), birthdate: None, linenum: 100 };

                let interp_arr1 = [a,b];
                let interp_arr2 = [b,a];

            // Exact Maxage <-> Birthdate
                let c = AgeData { age: Age::None, minage: Age::None, maxage: Age::Exact(18), date: Date::from_u32(1980_07_05),birthyear:None, birthdate: None, linenum: 100 };
                let d = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2004_07_05),birthyear:None, birthdate: Some(Date::from_u32(1961_07_03)), linenum: 100 };

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
                let a = AgeData { age: Age::None, minage: Age::None, maxage: Age::Approximate(18), date: Date::from_u32(1980_07_03),birthyear:None, birthdate: None, linenum: 100 };
                let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2004_07_05),birthyear:Some(1960), birthdate: None, linenum: 100 };

                let interp_arr1 = [a,b];
                let interp_arr2 = [b,a];

            // Approx Maxage <-> Birthdate
                let c = AgeData { age: Age::None, minage: Age::None, maxage: Age::Approximate(18), date: Date::from_u32(1980_07_05),birthyear:None, birthdate: None, linenum: 100 };
                let d = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2004_07_05),birthyear:None, birthdate: Some(Date::from_u32(1960_07_03)), linenum: 100 };

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
                let a = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(1980_07_03),birthyear:Some(1961), birthdate: None, linenum: 100 };
                let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2004_07_05),birthyear:Some(1960), birthdate: None, linenum: 100 };

                let interp_arr1 = [a,b];
                let interp_arr2 = [b,a];

            // Birthyear <-> Birthdate
                let c = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(1980_07_05),birthyear:Some(1961), birthdate: None, linenum: 100 };
                let d = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2004_07_05),birthyear:None, birthdate: Some(Date::from_u32(19600703)), linenum: 100 };

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
                let a = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(1980_07_05),birthyear:None, birthdate: Some(Date::from_u32(19600704)), linenum: 100 };
                let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2004_07_05),birthyear:None, birthdate: Some(Date::from_u32(19600703)), linenum: 100 };

                let interp_arr1 = [a,b];
                let interp_arr2 = [b,a];

                assert!(!is_agedata_consistent(&interp_arr1));
                assert!(!is_agedata_consistent(&interp_arr2));

        }

        #[test]
        fn test_bound_no_data() {
            // Make sure no age data works
                let a1 = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };
                let a2 = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_10_12),birthyear:None, birthdate: None, linenum: 100 };
                let a3 = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2001_07_04),birthyear:None, birthdate: None, linenum: 100 };
                let a4 = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2007_03_05),birthyear:None, birthdate: None, linenum: 100 };

                let interp_arr1 = [a1,a2,a3,a4];

                assert_eq!(estimate_birthdate(&interp_arr1),BirthdateConstraint::None);

        }

        #[test]
        fn test_bound_age_range() {
            // See one instance of two different ages in one year
                let a1 = AgeData { age: Age::Exact(20), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };
                let a2 = AgeData { age: Age::Exact(21), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_10_12),birthyear:None, birthdate: None, linenum: 100 };
                let a3 = AgeData { age: Age::Exact(21), minage: Age::None, maxage: Age::None, date: Date::from_u32(2001_07_04),birthyear:None, birthdate: None, linenum: 100 };
                let a4 = AgeData { age: Age::Exact(27), minage: Age::None, maxage: Age::None, date: Date::from_u32(2007_03_05),birthyear:None, birthdate: None, linenum: 100 };

                let mut interp_arr1 = [a1,a2,a3,a4];
                let bound1 = BirthdateConstraint::Bound{min_date: Date::from_u32(1979_08_05),max_date:Date::from_u32(1979_10_12)};

                // See two instances of different ages in a year, bound should be tighter
                let b1 = AgeData { age: Age::Exact(20), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };
                let b2 = AgeData { age: Age::Exact(21), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_10_12),birthyear:None, birthdate: None, linenum: 100 };
                let b3 = AgeData { age: Age::Exact(21), minage: Age::None, maxage: Age::None, date: Date::from_u32(2001_07_04),birthyear:None, birthdate: None, linenum: 100 };
                let b4 = AgeData { age: Age::Exact(27), minage: Age::None, maxage: Age::None, date: Date::from_u32(2007_03_05),birthyear:None, birthdate: None, linenum: 100 };
                let b5 = AgeData { age: Age::Exact(28), minage: Age::None, maxage: Age::None, date: Date::from_u32(2007_09_15),birthyear:None, birthdate: None, linenum: 100 };

                let mut interp_arr2 = [b1,b2,b3,b4,b5];
                let bound2 = BirthdateConstraint::Bound{min_date: Date::from_u32(1979_08_05),max_date:Date::from_u32(1979_09_15)};

            // See an age change, but split between two years
                let c1 = AgeData { age: Age::Exact(20), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };
                let c2 = AgeData { age: Age::Exact(21), minage: Age::None, maxage: Age::None, date: Date::from_u32(2001_06_12),birthyear:None, birthdate: None, linenum: 100 };
                let c3 = AgeData { age: Age::Exact(25), minage: Age::None, maxage: Age::None, date: Date::from_u32(2004_10_12),birthyear:None, birthdate: None, linenum: 100 };
                let c4 = AgeData { age: Age::Exact(26), minage: Age::None, maxage: Age::None, date: Date::from_u32(2006_03_05),birthyear:None, birthdate: None, linenum: 100 };

                let mut interp_arr3 = [c1,c2,c3,c4];
                let bound3 = BirthdateConstraint::Bound{min_date: Date::from_u32(1979_08_05),max_date:Date::from_u32(1979_10_12)};

            // See two age changes, split between years
                let d1 = AgeData { age: Age::Exact(20), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };
                let d2 = AgeData { age: Age::Exact(21), minage: Age::None, maxage: Age::None, date: Date::from_u32(2001_06_12),birthyear:None, birthdate: None, linenum: 100 };
                let d3 = AgeData { age: Age::Exact(25), minage: Age::None, maxage: Age::None, date: Date::from_u32(2004_10_12),birthyear:None, birthdate: None, linenum: 100 };
                let d4 = AgeData { age: Age::Exact(26), minage: Age::None, maxage: Age::None, date: Date::from_u32(2006_03_05),birthyear:None, birthdate: None, linenum: 100 };
                let d5 = AgeData { age: Age::Exact(29), minage: Age::None, maxage: Age::None, date: Date::from_u32(2008_09_15),birthyear:None, birthdate: None, linenum: 100 };

                let mut interp_arr4 = [d1,d2,d3,d4,d5];
                let bound4 = BirthdateConstraint::Bound{min_date: Date::from_u32(1979_08_05),max_date:Date::from_u32(1979_09_15)};

                assert!(all_permutation_bd_equal(&mut interp_arr1,bound1));
                assert!(all_permutation_bd_equal(&mut interp_arr2,bound2));
                assert!(all_permutation_bd_equal(&mut interp_arr3,bound3));
                assert!(all_permutation_bd_equal(&mut interp_arr4,bound4));

        }

        #[test]
        fn test_known_age_range() {
                //All ages from one year, no age change
                let a1 = AgeData { age: Age::Exact(20), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };
                let a2 = AgeData { age: Age::Exact(20), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_10_12),birthyear:None, birthdate: None, linenum: 100 };
                let a3 = AgeData { age: Age::Exact(20), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_07_04),birthyear:None, birthdate: None, linenum: 100 };
                let a4 = AgeData { age: Age::Exact(20), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_03_05),birthyear:None, birthdate: None, linenum: 100 };

                let mut interp_arr1 = [a1,a2,a3,a4];
                let known1 = BirthdateConstraint::KnownRegion{min_date: Date::from_u32(1980_03_05),max_date:Date::from_u32(1980_10_12)};
                
                //Ages from different years, no age change
                let b1 = AgeData { age: Age::Exact(20), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };
                let b2 = AgeData { age: Age::Exact(21), minage: Age::None, maxage: Age::None, date: Date::from_u32(2001_10_12),birthyear:None, birthdate: None, linenum: 100 };
                let b3 = AgeData { age: Age::Exact(24), minage: Age::None, maxage: Age::None, date: Date::from_u32(2004_07_04),birthyear:None, birthdate: None, linenum: 100 };
                let b4 = AgeData { age: Age::Exact(26), minage: Age::None, maxage: Age::None, date: Date::from_u32(2006_03_05),birthyear:None, birthdate: None, linenum: 100 };
                
                let mut interp_arr2 = [b1,b2,b3,b4];

                assert!(all_permutation_bd_equal(&mut interp_arr1,known1));
                assert!(all_permutation_bd_equal(&mut interp_arr2,known1));
        }

        #[test]
        fn test_approx_age(){
        // Only an approximate age
                let a1 = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };
                let a2 = AgeData { age: Age::Approximate(20), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_10_12),birthyear:None, birthdate: None, linenum: 100 };
                let a3 = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2001_07_04),birthyear:None, birthdate: None, linenum: 100 };
                let mut interp_arr1 = [a1,a2,a3];
                let bound1 = BirthdateConstraint::Bound{min_date: Date::from_u32(1979_01_01),max_date:Date::from_u32(1979_12_31)};


                // Update a known age range to a birthdate range using an approximate age
                let b1 = AgeData { age: Age::Exact(20), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };
                let b2 = AgeData { age: Age::Approximate(20), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_10_12),birthyear:None, birthdate: None, linenum: 100 };
                let b3 = AgeData { age: Age::Exact(21), minage: Age::None, maxage: Age::None, date: Date::from_u32(2001_07_04),birthyear:None, birthdate: None, linenum: 100 };
                let b4 = AgeData { age: Age::Exact(27), minage: Age::None, maxage: Age::None, date: Date::from_u32(2007_03_05),birthyear:None, birthdate: None, linenum: 100 };
                let mut interp_arr2 = [b1,b2,b3,b4];
                let bound2 = BirthdateConstraint::Bound{min_date: Date::from_u32(1979_08_05),max_date:Date::from_u32(1979_12_31)};

                assert!(all_permutation_bd_equal(&mut interp_arr1,bound1));
                assert!(all_permutation_bd_equal(&mut interp_arr2,bound2));
        }

        #[test]
        fn test_age_minage(){
         // Age <-> Minage
                let a1 = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_10_12),birthyear:None, birthdate: None, linenum: 100 };
                let a2 = AgeData { age: Age::None, minage: Age::Exact(40), maxage: Age::None, date: Date::from_u32(2000_11_13),birthyear:None, birthdate: None, linenum: 100 };
                let a3 = AgeData { age: Age::Exact(39), minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };

                let interp_arr1 = [a1,a2,a3];
                let interp_arr1_1 = [a1,a3,a2];
                let known1 = BirthdateConstraint::Bound{min_date: Date::from_u32(1960_08_05),max_date:Date::from_u32(1960_11_13)};
             
        //         assert_eq!(estimate_birthdate(&interp_arr1),known1);
        //         assert_eq!(estimate_birthdate(&interp_arr1_1),known1);        
        // }

        // #[test]
        // fn test_minage() {
        //     // Just exact minage, upper bound on birthdate
        //         let a1 = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_10_12),birthyear:None, birthdate: None, linenum: 100 };
        //         let a2 = AgeData { age: Age::None, minage: Age::Exact(40), maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };
        //         let a3 = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_10_12),birthyear:None, birthdate: None, linenum: 100 };

        //         let interp_arr1 = [a1,a2,a3];
        //         let known1 = BirthdateConstraint::KnownRegion{min_date: Date::from_u32(0000_01_01),max_date:Date::from_u32(1960_08_05)};
             
        //         // Just approx minage, upper bound on birthdate
        //         let b1 = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_10_12),birthyear:None, birthdate: None, linenum: 100 };
        //         let b2 = AgeData { age: Age::None, minage: Age::Approximate(39), maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };
        //         let b3 = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_10_12),birthyear:None, birthdate: None, linenum: 100 };

        //         let interp_arr2 = [b1,b2,b3];
        //         let known2 = BirthdateConstraint::KnownRegion{min_date: Date::from_u32(0000_01_01),max_date:Date::from_u32(1960_12_31)};

        //         // Two exact minages, tighten bound
        //         let c1 = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_10_12),birthyear:None, birthdate: None, linenum: 100 };
        //         let c2 = AgeData { age: Age::None, minage: Age::Exact(40), maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };
        //         let c3 = AgeData { age: Age::None, minage: Age::Exact(45), maxage: Age::None, date: Date::from_u32(2002_10_12),birthyear:None, birthdate: None, linenum: 100 };

        //         let interp_arr3 = [c1,c2,c3];
        //         let known3 = BirthdateConstraint::KnownRegion{min_date: Date::from_u32(0000_01_01),max_date:Date::from_u32(1957_10_12)};

        //         // An exact and an approx minage, tighten bound
        //         let d1 = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(2000_10_12),birthyear:None, birthdate: None, linenum: 100 };
        //         let d2 = AgeData { age: Age::None, minage: Age::Exact(40), maxage: Age::None, date: Date::from_u32(2000_08_05),birthyear:None, birthdate: None, linenum: 100 };
        //         let d3 = AgeData { age: Age::None, minage: Age::Approximate(45), maxage: Age::None, date: Date::from_u32(2002_10_12),birthyear:None, birthdate: None, linenum: 100 };

        //         let interp_arr4 = [d1,d2,d3];
        //         let interp_arr4_1 = [d1,d3,d2];
        //         let known4 = BirthdateConstraint::KnownRegion{min_date: Date::from_u32(0000_01_01),max_date:Date::from_u32(1957_12_31)};             
             
        //         assert_eq!(estimate_birthdate(&interp_arr1),known1);
        //         assert_eq!(estimate_birthdate(&interp_arr2),known2);
        //         //assert_eq!(estimate_birthdate(&interp_arr3),known3);
        //         // assert_eq!(estimate_birthdate(&interp_arr4),known4);
        //         // assert_eq!(estimate_birthdate(&interp_arr4_1),known4);

        }

        // #[test]
        // fn test_alldata() {
        //         let a = AgeData { age: Age::Exact(20), minage: Age::Exact(19), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 1000 };
        //         let b = AgeData { age: Age::Exact(21), minage: Age::Exact(20), maxage: Age::Exact(21), date: Date::from_u32(20011231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };

        //         let c = AgeData { age: Age::Exact(20), minage: Age::Exact(19), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 1000 };
        //         let d = AgeData { age: Age::Exact(21), minage: Age::Exact(20), maxage: Age::Exact(21), date: Date::from_u32(20011231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };


        //         let mut interp_arr = [a,b];
        //         let old_arr = [c,d];

        //         interpolate(&mut interp_arr);

        //         assert!(interp_arr.iter().eq(old_arr.iter()));
        // }

        // // #[test]
        // // fn test_basic_interp() {
        // //         let a = AgeData { age: Age::Exact(20), minage: Age::Exact(20), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };
        // //         let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20011231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };

        // //         let c = AgeData { age: Age::Exact(20), minage: Age::Exact(20), maxage: Age::Exact(20), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };
        // //         let d = AgeData { age: Age::Exact(21), minage: Age::Exact(21), maxage: Age::Exact(21), date: Date::from_u32(20001231),birthyear:Some(1980), birthdate: Some(Date::from_u32(19800101)), linenum: 2000 };


        // //         let mut interp_arr = [a,b];
        // //         let old_arr = [c,d];

        // //         interpolate(&mut interp_arr);

        // //         assert!(interp_arr.iter().eq(old_arr.iter()));
        // // }

        // #[test]
        // fn test_nodata() {
        //         let a = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20001231),birthyear:None, birthdate: None, linenum: 1000 };
        //         let b = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20011231),birthyear:None, birthdate: None, linenum: 2000 };

        //         let c = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20001231),birthyear:None, birthdate: None, linenum: 1000 };
        //         let d = AgeData { age: Age::None, minage: Age::None, maxage: Age::None, date: Date::from_u32(20011231),birthyear:None, birthdate: None, linenum: 2000 };


        //         let mut interp_arr = [a,b];
        //         let old_arr = [c,d];

        //         interpolate(&mut interp_arr);

        //         assert!(interp_arr.iter().eq(old_arr.iter()));
        // }
}