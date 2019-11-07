mod populate;
mod simulation;
mod chip_info;
mod Annealer;
use rand::distributions::{Distribution, Uniform};

/* Helpers ======================================== */

// initial global hunter that will try to return a first non-zero yield rate
fn global_hunter(chip : &chip_info::ChipInfo, sigma : f64) -> Vec<f64> {
    // for uniform guessing
    let mut range = rand::thread_rng();
    let uniform_range = Uniform::from(5.0..5.34);
    // the answer
    let mut f: Vec<f64> = (0..chip.qubit_num).map(|_| {
        uniform_range.sample(&mut range)
    }).collect();
    // make the first call
    let (_,mut curr_yield_rate) = simulation::complete_yield_simulation(chip, sigma, &f);
    // now search for just 5 iterations
    for _ in 0..5 {
        println!("Global hunter starter {:.3}%", curr_yield_rate);
        let new_f : Vec<f64> = (0..chip.qubit_num).map(|_| {
            uniform_range.sample(&mut range)
        }).collect();
        let (_,new_yield_rate) = simulation::complete_yield_simulation(chip, sigma, &new_f);
        if new_yield_rate > curr_yield_rate {
            curr_yield_rate = new_yield_rate;
            f = new_f;
        }
    }
    // done, hopefully found a non-zero yield rate
    f
}

/* ================================================ */

fn main() {
    /*
    Build a chip object. Initialised parameters do not really matter,
    as long as they are empty
    */
    let mut IBM17Q2B : chip_info::ChipInfo = chip_info
        ::ChipInfo::new(0, vec![],vec![],vec![],vec![],vec![],vec![],vec![]);

    /*
    Populate this chip object through the chip file. Note, do not add
    the chip/ directory
    */
    IBM17Q2B.populate_from_file("17q_bus2.chip");
    // checkup
    //IBM17Q2B.print_details();

    //let mut f : Vec<f64> = vec![5.07, 5.27, 5.2, 5.13, 5.17, 5.23, 5.34, 5.08, 5.13, 5.05, 5.34, 5.27, 5.0, 5.1, 5.21, 5.0, 5.25];
    let mut f : Vec<f64> = global_hunter(&IBM17Q2B, 0.01);

    /*
    Now that we have an f that (hopefully) has a yield_rate > 0, we can
    begin annealing. We can choose a standard annealing process or we can
    choose the segmented approach:

    Annealer::standard(_chip, _frequency, _number_iterations, _threshold, _sigma);
    Annealer::segmented(_chip, _frequency, _number_iterations, _threshold, _sigma);
    */
    Annealer::segmented(&IBM17Q2B, &mut f, 280, 12.0, 0.01);
}