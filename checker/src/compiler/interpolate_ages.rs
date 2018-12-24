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

    #[test]
    fn test_interp() {
        let a = AgeData { age: 20, minage: 20, maxage: 20, date: Date::from_u32(2000_12_31);,birthyear:1980, birthdate: Date::from_u32(1980_01_01), 2000 };
        let b = AgeData { age: 20, minage: 20, maxage: 20, date: Date::from_u32(2000_12_31);,birthyear:1980, birthdate: Date::from_u32(1980_01_01), 2000 };

        let c = AgeData { age: 20, minage: 20, maxage: 20, date: Date::from_u32(2000_12_31);,birthyear:1980, birthdate: Date::from_u32(1980_01_01), 2000 };
        let d = AgeData { age: 20, minage: 20, maxage: 20, date: Date::from_u32(2000_12_31);,birthyear:1980, birthdate: Date::from_u32(1980_01_01), 2000 };


        let mut interp_arr = [a,b];
        let old_arr = [c,d];

        interpolate(&mut interp_arr);

        assert!(interp_arr.iter().eq(old_arr.iter()));
    }
}