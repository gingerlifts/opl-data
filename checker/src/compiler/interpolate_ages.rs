use crate::check_entries::Entry;
use opltypes::*;

pub struct AgeData {
    pub age: Age,
    pub minage: Age,
    pub maxage: Age,
    pub date: Option<Date>,
    pub birthyear: Option<u32>,
    pub birthdate: Option<Date>,
    pub linenum: u32,

}

fn is_birthyear_consistent(entries: &[AgeData]) -> bool {
	true
}

fn is_birthdate_consistent(entries: &[AgeData]) -> bool {
	true
}

fn is_agedata_consistent(entries: &[AgeData]) -> bool {
	true
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

}