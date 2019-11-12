use crate::chip_info;
use crate::simulation;
use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::io::Write;

// initial global hunter that will try to return a first non-zero yield rate
pub fn global_hunter(chip: &chip_info::ChipInfo, number_of_calls: usize) -> Vec<f64> {
    // for uniform guessing
    let mut range = rand::thread_rng();
    let uniform_range = Uniform::from(5.0..5.34);
    // the answer
    let mut f: Vec<f64> = (0..chip.qubit_num)
        .map(|_| uniform_range.sample(&mut range))
        .collect();
    // make the first call
    let (_, mut curr_yield_rate) = simulation::complete_yield_simulation(chip, chip.sigma, &f);
    // now search for just 5 iterations
    for _ in 0..number_of_calls {
        //println!("Global hunter starter {:.3}%", curr_yield_rate);
        let new_f: Vec<f64> = (0..chip.qubit_num)
            .map(|_| uniform_range.sample(&mut range))
            .collect();
        let (_, new_yield_rate) = simulation::complete_yield_simulation(chip, chip.sigma, &new_f);
        if new_yield_rate > curr_yield_rate {
            curr_yield_rate = new_yield_rate;
            f = new_f;
        }
    }
    // done, hopefully found a non-zero yield rate
    f
}

// writes the data to the file
pub fn write_to_file(data: &Vec<f64>, file_name: &str) {
    // convert to string
    let yields_string: Vec<String> = data.iter().map(|y| y.to_string()).collect();

    let mut file = File::create("data/".to_owned() + file_name).expect("Unable to create file");
    writeln!(file, "{}", yields_string.join("\n"));
    println!("\nDone writing to file data/{}", file_name);
}

pub fn write_to_file_data(iter: &Vec<i64>, yields: &Vec<f64>, file_name: &str) {
    let mut final_string: Vec<String> = vec![];
    for i in 0..iter.len() {
        final_string.push(iter[i].to_string() + " " + &yields[i].to_string());
    }

    let mut file = File::create("data/".to_owned() + file_name).expect("Unable to create file");
    writeln!(file, "{}", final_string.join("\n"));
    println!("\nDone writing to file data/{}", file_name);
}
