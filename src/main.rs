mod Annealer;
mod chip_info;
mod populate;
mod simulation;
use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::io::Write;

/* Helpers ======================================== */

// initial global hunter that will try to return a first non-zero yield rate
fn global_hunter(chip: &chip_info::ChipInfo, number_of_calls: usize) -> Vec<f64> {
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
fn write_to_file(data: &Vec<f64>, file_name: &str) {
    // convert to string
    let yields_string: Vec<String> = data.iter().map(|y| y.to_string()).collect();

    let mut file = File::create("data/".to_owned() + file_name).expect("Unable to create file");
    writeln!(file, "{}", yields_string.join("\n"));
    println!("\nDone writing to file data/{}", file_name);
}

fn write_to_file_data(iter: &Vec<i64>, yields: &Vec<f64>, file_name: &str) {
    let mut final_string: Vec<String> = vec![];
    for i in 0..iter.len() {
        final_string.push(iter[i].to_string() + " " + &yields[i].to_string());
    }

    let mut file = File::create("data/".to_owned() + file_name).expect("Unable to create file");
    writeln!(file, "{}", final_string.join("\n"));
    println!("\nDone writing to file data/{}", file_name);
}

/* ================================================ */

fn main() {
    /*
    Build a chip object. Initialised parameters do not really matter,
    as long as they are empty
    */
    let mut IBM17Q2B: chip_info::ChipInfo = chip_info::ChipInfo::new(
        0,
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        0.0,
    );

    /*
    Populate this chip object through the chip file. Note, do not add
    the chip/ directory
    */
    IBM17Q2B.populate_from_file("17q_bus2.chip");
    // checkup
    //IBM17Q2B.print_details();

    //let mut f : Vec<f64> = vec![5.07, 5.27, 5.2, 5.13, 5.17, 5.23, 5.34, 5.08, 5.13, 5.05, 5.34, 5.27, 5.0, 5.1, 5.21, 5.0, 5.25];
    // let mut f : Vec<f64> = global_hunter(&IBM17Q2B, 0.01);

    /*
    Now that we have an f that (hopefully) has a yield_rate > 0, we can
    begin annealing. We can choose a standard annealing process or we can
    choose the segmented approach:

    Annealer::brute_forcce(_chip, _frequency, _number_iterations, _threshold) -> (i64,f64);
    Annealer::standard(_chip, _frequency, _number_iterations, _threshold) -> (i64,f64);
    Annealer::segmented(_chip, _frequency, _number_iterations, _threshold, _segments) -> (i64,f64);

    NOTE: need to pass the segments for the segmented annealer (default is THREE segments)
    RETURNS: tuple (final_iter_number, final_yield_number)
    */

    // here are my segments
    //let segments : Vec<Vec<usize>> = vec![vec![0,1,2,3,4,5],vec![6,7,8,9,10,11],vec![12,13,14,15,16]];
    let segments2: Vec<Vec<usize>> = vec![
        vec![0, 1, 4, 5, 8, 9],
        vec![2, 3, 6, 7, 11, 15],
        vec![10, 12, 13, 14, 16],
    ];
    //let segments3 : Vec<Vec<usize>> = vec![vec![0,1,5,10,14],vec![4,8,7,9,12,13],vec![2,3,6,11,15,16]];
    //let segments4 : Vec<Vec<usize>> = vec![vec![0,2,8,10,13,16],vec![1,4,7,9,11,14],vec![3,5,6,12,15]];
    //let seg35_1: Vec<Vec<usize>> = vec![vec![0, 1, 3], vec![2, 4, 5]];
    // here are the yield rates (last one is my final yield)

    // I'm going to run 100 trials
    //let mut trial_results : Vec<f64> = vec![];
    let mut iterations: Vec<i64> = vec![];
    let mut the_yields: Vec<f64> = vec![];
    for i in 0..100 {
        let mut f: Vec<f64> = global_hunter(&IBM17Q2B, 5);
        let (iter_number, yields) = Annealer::segmented(&IBM17Q2B, &mut f, 280, 13.0, &segments2);
        //let (iter_number, yields) = Annealer::brute_force(&IBM17Q2B, &mut f, 280, 95.0);
        //let (iter_number, yields) = Annealer::standard(&IBM17Q2B, &mut f, 280, 0.5);
        iterations.push(iter_number);
        the_yields.push(yields);
        println!("{}: {} {}", i, iter_number, yields);
    }
    // write this data to file for analysis
    //write_to_file_data(&iterations, &the_yields, "100_trials_25_normal.txt");
}
