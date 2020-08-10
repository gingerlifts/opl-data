//use checker::compiler::*;
use opltypes::*;


mod group_age;
use group_age::*;

fn main() {
     
    let birthdate1 = Date::from_parts(1965, 02, 03);
    let birthdate2 = Date::from_parts(1966, 02, 03);
    let birthdate3 = Date::from_parts(1967, 02, 03);
    let birthdate4 = Date::from_parts(1966, 02, 03);
    let birthdate5 = Date::from_parts(1936, 02, 03);
    
    let date1 = Date::from_parts(2020, 07, 26);
    let date2 = Date::from_parts(2017, 03, 22);
    let date3 = Date::from_parts(2005, 03, 15);

    let mut bdr1 = BirthDateRange::default();
    bdr1.narrow_by_birthdate(birthdate1);

    let mut bdr2 = BirthDateRange::default();
    bdr2.narrow_by_birthdate(birthdate2);

    let mut bdr3 = BirthDateRange::default();
    bdr3.narrow_by_birthdate(birthdate3);

    let mut bdr4 = BirthDateRange::default();
    bdr4.narrow_by_birthdate(birthdate4);

    let mut bdr5 = BirthDateRange::default();
    bdr5.narrow_by_birthdate(birthdate5);

    let mut bdr6 = BirthDateRange::default();
    bdr6.narrow_by_age(Age::Exact(84), date1);

    let mut bdr7 = BirthDateRange::default();
    bdr7.narrow_by_age(Age::Approximate(50), date2);

    let mut bdr8 = BirthDateRange::default();
    bdr8.narrow_by_range(Age::Approximate(40), Age::Approximate(99), date3);

    let age_data = [bdr1,bdr2,bdr3,bdr4,bdr5,bdr6,bdr7,bdr8];

    let grouped_data = group_by_age(& age_data,0);

    print_groups(&age_data,grouped_data);

}
