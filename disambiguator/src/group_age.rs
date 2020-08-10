//! Checks CSV data files for validity.

// The algorithm described in https://iopscience.iop.org/article/10.0088/0026-1394/44/3/005 
// is used for grouping.

//use checker::compiler::*;

use opltypes::*;

extern crate itertools;
extern crate itertools_num;

use itertools_num::linspace;

fn calc_distance(bd_range1: & BirthDateRange, x: f32) -> f32{
    if bd_range1.min.count_days() as f32 > x{
        return bd_range1.min.count_days() as f32 - x;
    }
    else if (bd_range1.max.count_days() as f32) < x{
        return bd_range1.max.count_days() as f32 - x;
    }
    0.0
}

//Need to account for uncertainty here as well
fn calc_error(bd_range1: & BirthDateRange, x: f32) -> f32{
    calc_distance(bd_range1,x).powf(2.0)
}


fn bd_range_vec_is_consistent(bd_range_vec: & Vec<& BirthDateRange>) -> bool{
    let mut bdr = BirthDateRange::default();
    for curr_range in bd_range_vec {
        if curr_range.max < bdr.min{
            return false;
        }
        else if curr_range.max < bdr.max{
            bdr.max = curr_range.max
        }

        if curr_range.min > bdr.max{
            return false;
        }
        else if curr_range.min > bdr.min{
            bdr.min = curr_range.min
        }
    }
    true

}

// Finds the distance of a point from a vector of birthdate ranges and returns them in sorted order
fn get_sorted_errors(x: f32, bd_range_vec: & Vec<& BirthDateRange>) -> Vec<(usize,f32)>{

    let mut ii = 0;
    let mut errors :Vec<(usize,f32)> = bd_range_vec.into_iter()
                             .map(|bd_range| {ii+=1; (ii-1,calc_error(bd_range,x))}).collect();
    errors.sort_by(|a,b| (*a).1.partial_cmp(&(*b).1).unwrap());
    errors
}

//Check a vector of sample points to find the largest LCS possible (if any) at one of these points
fn find_lcs_numeric<'a>(bd_range_vec: &Vec<&'a BirthDateRange>, test_vals: &Vec<f32>) -> Option<Vec<&'a BirthDateRange>>{
    let mut best_errors = Vec::<(usize,f32)>::new();
    let mut _wm = 0.0;

    let mut error_vec = Vec::new();
    for val in test_vals{
        error_vec.push(get_sorted_errors(*val,&bd_range_vec));
    }
    for r in (2..bd_range_vec.len()).rev() {
        let mut F_min = error_vec[0][0..r].iter().map(|(_a,b)| b).sum();
        best_errors = error_vec[0][0..r].to_vec();
        _wm = test_vals[0];
        
        for ii in 1..test_vals.len(){

            let F:f32 = error_vec[ii][0..r].iter().map(|(_a,b)| b).sum();
            if F < F_min{
                F_min = F;
                best_errors = error_vec[ii][0..r].to_vec();
                _wm = test_vals[ii];
            }
        }
        if F_min == 0.0{
            return  Some(best_errors.iter().map(|(a,_b)| bd_range_vec[*a]).collect());
        }
    }
    None
}

//Find the points that could possibly yield a minima
fn get_test_points(bd_range_vec: &Vec<& BirthDateRange>) ->  Vec<f32> {

    //Calculate the points where the ordering of the error curves changes.
    let mut test_points = Vec::new();

    //breakpoints occur when we are greater than the end of one range and smaller than the start of a second
    //Start by obtaining a list of sorted range mins and maxes

    let mut range_mins: Vec<f32>= bd_range_vec.iter().map(|a| a.min.count_days() as f32).collect();
    let mut range_maxs: Vec<f32>= bd_range_vec.iter().map(|a| a.max.count_days() as f32).collect();

    let mut sorted_ranges = Vec::new();
    for bd_range in bd_range_vec{
        sorted_ranges.push((bd_range.min.count_days() as f32 ,(bd_range.max.count_days() as f32)));
    }
    //sort by 
    sorted_ranges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    //There will be one intersection point per range maximum (I think...)
    //Midpoint of left edge and first breakpoint
    test_points.push((sorted_ranges[0].1+((sorted_ranges[0].1+sorted_ranges[1].0)/2.0)/2.0));
    for ii in 0..range_maxs.len()-2{
        let breakpoint1 = (sorted_ranges[ii].1+sorted_ranges[ii+1].0)/2.0;
        let breakpoint2 = (sorted_ranges[ii+1].1+sorted_ranges[ii+2].0)/2.0;
        test_points.push((breakpoint1+breakpoint2)/2.0)
    }

    //Midpoint of right edge and last breakpoint
    test_points.push((sorted_ranges[range_mins.len()-1].0
                      +((sorted_ranges[range_mins.len()-2].1+sorted_ranges[range_mins.len()-1].0)/2.0)/2.0));

    //Surely theres a better way of doing this
    for bd_range in bd_range_vec
    {
        test_points.push(bd_range.min.count_days() as f32);
        test_points.push(bd_range.max.count_days() as f32);
    }
    test_points.sort_by(|a, b| a.partial_cmp(b).unwrap());

    test_points.dedup();

    test_points
}

// Version of the LCS algorithm that uses where the error curves intersect to potentially reduce
// the numer of calls
fn find_lcs_algebraic<'a>(bd_range_vec: &Vec<&'a BirthDateRange>) -> Option<Vec<&'a BirthDateRange>>{
    let mut best_errors = Vec::<(usize,f32)>::new();
    let mut _wm = 0.0;

    let test_points = get_test_points(bd_range_vec);

    let mut error_vec = Vec::new();
    for val in &test_points{
        error_vec.push(get_sorted_errors(*val,&bd_range_vec));
    }


    for r in (2..bd_range_vec.len()).rev() {
        let mut F_min = error_vec[0][0..r].iter().map(|(_a,b)| b).sum();
        best_errors = error_vec[0][0..r].to_vec();
        _wm = test_points[0];
        for ii in 1..test_points.len(){

            let F:f32 = error_vec[ii][0..r].iter().map(|(_a,b)| b).sum();
            if F < F_min{
                F_min = F;
                best_errors = error_vec[ii][0..r].to_vec();
                _wm = test_points[ii];
            }
        }
        if F_min == 0.0{
            return  Some(best_errors.iter().map(|(a,_b)| bd_range_vec[*a]).collect());
        }
    }

    None
}

//Find the least consistent subset (if any) of a BirthDateRange
//I don't know what I'm doing with this lifetime parameter, compiler told me to.
fn find_lcs<'a>(bd_range_vec: &Vec<&'a BirthDateRange>) -> Option<Vec<&'a BirthDateRange>>{

    let mut lcs = Vec::new();
    if bd_range_vec.len() == 0{
        return None;
    }
    // LCS of a length 1 vector is the vector
    else if bd_range_vec.len() == 1{
        lcs.push(bd_range_vec[0]);
        return Some(lcs);
    }
    //LCS of a consistent vector is the vector
    else if bd_range_vec_is_consistent(bd_range_vec){
        return Some(bd_range_vec.to_vec());
    }

    let mut x_min = bd_range_vec[0].min; 
    let mut x_max = bd_range_vec[0].max;

    let mut min_gap = x_max-x_min;
    for ii in 1..bd_range_vec.len(){
        if bd_range_vec[ii].min < x_min{
            x_min = bd_range_vec[ii].min;
        }
        else if bd_range_vec[ii].max > x_max{
            x_max = bd_range_vec[ii].max;
        }
        if bd_range_vec[ii].max - bd_range_vec[ii].min < min_gap{
            min_gap = bd_range_vec[ii].max - bd_range_vec[ii].min;
        }
    }

    // Number of sample dates, need logic to decide when to use the approximate vs algebraic methods
    let M = ((x_max-x_min)/(min_gap+1)+1) as usize;

    let numeric_ops = (M as f32)*(bd_range_vec.len() as f32)*(bd_range_vec.len() as f32).log(2.0);
    //Paper has an O(N^3) complexity, but I think we're O(N^2), need to confirm
    let algebraic_ops = (bd_range_vec.len() as f32).powf(2.0);


    if numeric_ops < algebraic_ops{
        let test_vals: Vec<f32>  = linspace::<f32>(x_min.count_days() as f32,x_max.count_days()  as f32,M).map(|x| x).collect();
        find_lcs_numeric(bd_range_vec,&test_vals)
    }
    else{
        find_lcs_algebraic(bd_range_vec)
    }

}

// Group data by consistent subsets of age data
pub fn group_by_age(bd_range_vec: & [BirthDateRange], _acceptable_delta: u32) ->  Vec<Vec<&BirthDateRange> >{
    let mut all_groups_vec = Vec::new();
    let mut ungrouped_vec: Vec<& BirthDateRange> = Vec::new();

    for bd_range in bd_range_vec{
        ungrouped_vec.push(bd_range);
    }

    let mut lcs = find_lcs(&ungrouped_vec);

    // Add this group to our list of groups and find the LCS of the remaining elements
    while lcs.is_some()
    {
        ungrouped_vec.retain(|&x| !lcs.as_ref().unwrap().contains(&x));

        all_groups_vec.push(lcs.unwrap());
        lcs = find_lcs(&ungrouped_vec);
       // println!("{:?}",ungrouped_vec.len());
    }

    // Then the remaining elements are all singletons
    if !ungrouped_vec.is_empty(){
        for bd_range in ungrouped_vec{
            all_groups_vec.push(vec![bd_range]);
        }
    } 

    all_groups_vec
}

//Function for printing grouped lifter data
pub fn print_groups(initial_group: &[BirthDateRange], bd_groups: Vec<Vec<&BirthDateRange> >){
    println!("Input is:");
    for curr_range in initial_group {
        print!("({},{}) ",curr_range.min,curr_range.max)
    }
    println!("");


    println!("Groupings are:");
    let mut ii = 0;
    for group in bd_groups {
        print!("{}: ",ii);
        for bd_range in group{
            print!("({},{}) ",bd_range.min,bd_range.max)
        }
        println!("");
        ii = ii+1;
    }
}
