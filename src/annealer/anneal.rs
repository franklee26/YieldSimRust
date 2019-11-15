//! Novel annealing algorithms
use crate::chip_info;
use crate::distribution;
use crate::simulation;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

/* Helpers ===================================================================== */

// helper to make and squish the move
fn return_moved_freq(f: &mut Vec<f64>) -> Vec<f64> {
    let mut range = rand::thread_rng();
    let mut moved_f = f.clone();
    for i in 0..moved_f.len() {
        let to_move: f64 = Uniform::from(-0.2..0.2).sample(&mut range);
        moved_f[i] += to_move;
        if moved_f[i] > 5.34 {
            moved_f[i] = 5.34;
        } else if moved_f[i] < 5.0 {
            moved_f[i] = 5.0;
        }
    }
    moved_f
}

// same as return_moved_freq except it targets segments
// 17q temporarily segmented as [0-5], [6,11], [12,16]
fn return_moved_freq_segment(
    segment_number: i64,
    f: &mut Vec<f64>,
    segments: &Vec<Vec<usize>>,
) -> Vec<f64> {
    let mut range = rand::thread_rng();
    let mut moved_f = f.clone();
    for i in &(segments[segment_number as usize]) {
        let to_move: f64 = Uniform::from(-0.25..0.25).sample(&mut range);
        moved_f[*i] += to_move;
        if moved_f[*i] > 5.34 {
            moved_f[*i] = 5.34;
        } else if moved_f[*i] < 5.0 {
            moved_f[*i] = 5.0;
        }
    }
    moved_f
}

/* ============================================================================== */

/// Standard simulated annealing algorithm
///
/// # Arguments
///
/// * `chip` - A `chip_info::ChipInfo` reference (ideally populated)
/// * `f` - A mutuable reference to an initial frequency configuration vector
/// * `number_of_passes` - The max number of iterations that will be run
/// * `threshold` - The threshold yield_rate that can cause an early break in the iteration
///
///  # Returns
///
/// `(i64, f64)` - A tuple where the first element is the number of iterations taken and the secnod element is the final yield-rate
///
/// # Notes
///
/// * Method uses methods from the `distribution` module. Change distribution and the parameters within the src code
pub fn standard(
    chip: &chip_info::ChipInfo,
    f: &mut Vec<f64>,
    number_of_passes: i64,
    threshold: f64,
) -> (i64, f64) {
    let mut iteration_number: i64 = 0;
    let (_, mut current_yield_rate) = simulation::complete_yield_simulation(&chip, chip.sigma, &f);
    let mut range = rand::thread_rng();
    for i in 0..number_of_passes {
        //println!();
        iteration_number = i;
        if current_yield_rate > threshold {
            // met threshold requirement
            //println!("Found suitable yield rate {:.3}%.",current_yield_rate);
            break;
        }
        // set the temperature
        // let mut temperature: f64 = -0.01 * i as f64;
        // temperature = temperature.exp();
        let f_with_move = return_moved_freq(f);
        // recompute yield rate
        let (_, moved_yield_rate) =
            simulation::complete_yield_simulation(&chip, chip.sigma, &f_with_move);

        //println!("{}: Considering new {:.3}% against current {:.3}% at temperature {:.3}.", i, moved_yield_rate, current_yield_rate, temperature);

        if moved_yield_rate > current_yield_rate {
            // accept the move
            //println!("Higher yield rate forced transition.");
            *f = f_with_move;
            current_yield_rate = moved_yield_rate;
        } else {
            // this is a bad move
            let mut prob: f64 = 1.0;
            if current_yield_rate - moved_yield_rate != 0.0 {
                prob =
                    distribution::exp_fermi_dirac_harsh(i, current_yield_rate - moved_yield_rate);
                if moved_yield_rate == 0.0 {
                    // further punishment
                    prob *= 0.005;
                }
            }
            //println!("Lower yield rate with diff {:.3} will move with {:.3} probability",current_yield_rate-moved_yield_rate,prob);
            // deciding to make the move
            if prob > range.gen_range(0.0, 1.0) {
                // make the move
                //println!("Move made despite lower yield rate.");
                *f = f_with_move;
                current_yield_rate = moved_yield_rate;
            }
        }
    }
    (iteration_number, current_yield_rate)
}

/// Segmented simulated annealing (SSA) algorithm
///
/// # Arguments
///
/// * `chip` - A `chip_info::ChipInfo` reference (ideally populated)
/// * `f` - A mutuable reference to an initial frequency configuration vector
/// * `number_of_passes` - The max number of iterations that will be run
/// * `threshold` - The threshold yield_rate that can cause an early break in the iteration
/// * `segments` - A vector of `usize` vectors, where each vector defines the indices for each segment
///
///  # Returns
///
/// `(i64, f64)` - A tuple where the first element is the number of iterations taken and the secnod element is the final yield-rate
///
/// # Notes
///
/// * Method uses methods from the `distribution` module. Change distribution and the parameters within the src code
pub fn segmented(
    chip: &chip_info::ChipInfo,
    f: &mut Vec<f64>,
    number_of_passes: i64,
    threshold: f64,
    segments: &Vec<Vec<usize>>,
) -> (i64, f64) {
    let mut iteration_number: i64 = 0;
    let (_, mut current_yield_rate) = simulation::complete_yield_simulation(&chip, chip.sigma, &f);
    // generate the segment number
    let mut range = rand::thread_rng();
    let mut segment_number: i64 = Uniform::from(0..(segments.len() as i64)).sample(&mut range);
    // begin annealing
    for i in 1..number_of_passes + 1 {
        //println!();
        iteration_number = i;
        if current_yield_rate > threshold {
            // met threshold requirement
            //println!("Found suitable yield rate {:.3}%.",current_yield_rate);
            break;
        }
        // set the temperature
        // let mut temperature: f64 = -0.025 * i as f64;
        // temperature = temperature.exp();
        let f_with_move = return_moved_freq_segment(segment_number, f, segments);
        // recompute yield rate
        let (_, moved_yield_rate) =
            simulation::complete_yield_simulation(&chip, chip.sigma, &f_with_move);

        // println!(
        //     "{}/{}: Considering new {:.3}% against current {:.3}% at temperature {:.3}.",
        //     segment_number, i, moved_yield_rate, current_yield_rate, temperature
        // );

        if moved_yield_rate > current_yield_rate {
            // accept the move
            //println!("Higher yield rate forced transition.");
            *f = f_with_move;
            current_yield_rate = moved_yield_rate;
        } else {
            // this is a bad move
            let mut prob: f64 = 1.0;
            if current_yield_rate - moved_yield_rate != 0.0 {
                // prob = (1.0 * temperature)
                //     / (1.0 + (4.0 * (current_yield_rate - moved_yield_rate)).exp());
                //prob = temperature * (4.0 * (current_yield_rate - moved_yield_rate)).exp();
                //prob = transition::exp_fermi_dirac_harsh(i, current_yield_rate - moved_yield_rate);
                prob = distribution::exp_fermi_dirac_custom(
                    i,
                    current_yield_rate - moved_yield_rate,
                    0.029,
                    3.9,
                    1.0,
                );
                if moved_yield_rate == 0.0 {
                    // further punishment
                    prob *= 0.005;
                }
            }
            // println!(
            //     "Lower yield rate with diff {:.3} will move with {:.3} probability",
            //     current_yield_rate - moved_yield_rate,
            //     prob
            // );
            // deciding to make the move
            if prob > Uniform::from(0.0..1.0).sample(&mut range) {
                // make the move
                //println!("Move made despite lower yield rate.");
                *f = f_with_move;
                current_yield_rate = moved_yield_rate;
            }
        }
        if i % 5 == 0 {
            // change the segment
            segment_number = Uniform::from(0..(segments.len() as i64)).sample(&mut range);
        }
    }
    (iteration_number, current_yield_rate)
}

/// Random frequency-selection algorithm
///
/// # Arguments
///
/// * `chip` - A `chip_info::ChipInfo` reference (ideally populated)
/// * `f` - A mutuable reference to an initial frequency configuration vector
/// * `number_of_passes` - The max number of iterations that will be run
/// * `threshold` - The threshold yield_rate that can cause an early break in the iteration
///
///  # Returns
///
/// `(i64, f64)` - A tuple where the first element is the number of iterations taken and the secnod element is the final yield-rate
pub fn random(
    chip: &chip_info::ChipInfo,
    f: &mut Vec<f64>,
    number_of_passes: i64,
    threshold: f64,
) -> (i64, f64) {
    let (_, mut current_yield_rate) = simulation::complete_yield_simulation(&chip, chip.sigma, &f);
    // for uniform guessing
    let mut range = rand::thread_rng();
    let mut iteration_number: i64 = 0;
    let uniform_range = Uniform::from(5.0..5.34);
    for i in 0..number_of_passes {
        // generate new guess
        //println!("{:.3}", current_yield_rate);
        iteration_number = i;
        if current_yield_rate > threshold {
            break;
        }
        let mut new_f: Vec<f64> = (0..chip.qubit_num)
            .map(|_| uniform_range.sample(&mut range))
            .collect();
        let (_, mut temp_yield_rate) =
            simulation::complete_yield_simulation(&chip, chip.sigma, &new_f);
        if temp_yield_rate > current_yield_rate {
            current_yield_rate = temp_yield_rate;
        }
    }
    (iteration_number, current_yield_rate)
}
