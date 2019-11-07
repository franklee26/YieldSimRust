use crate::chip_info;
use crate::simulation;
use rand::Rng;
use rand::distributions::{Uniform, Distribution};


/* Helpers ===================================================================== */

// helper to make and squish the move
fn return_moved_freq(f: &mut Vec<f64>) -> Vec<f64> {
    let mut range = rand::thread_rng();
    let mut moved_f = f.clone();
    for i in 0..moved_f.len() {
        let to_move : f64 = Uniform::from(-0.2..0.2).sample(&mut range);
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
fn return_moved_freq_segment(segment_number: i64, f: &mut Vec<f64>) -> Vec<f64> {
    let mut range = rand::thread_rng();
    let mut moved_f = f.clone();
    if segment_number == 0 {
        for i in 0..6 {
            let to_move : f64 = Uniform::from(-0.2..0.2).sample(&mut range);
            moved_f[i] += to_move;
            if moved_f[i] > 5.34 {
                moved_f[i] = 5.34;
            } else if moved_f[i] < 5.0 {
                moved_f[i] = 5.0;
            }
        }
    } else if segment_number == 1 {
        for i in 6..12 {
            let to_move : f64 = Uniform::from(-0.2..0.2).sample(&mut range);
            moved_f[i] += to_move;
            if moved_f[i] > 5.34 {
                moved_f[i] = 5.34;
            } else if moved_f[i] < 5.0 {
                moved_f[i] = 5.0;
            }
        }
    } else if segment_number == 2 {
        for i in 12..17 {
            let to_move : f64 = Uniform::from(-0.2..0.2).sample(&mut range);
            moved_f[i] += to_move;
            if moved_f[i] > 5.34 {
                moved_f[i] = 5.34;
            } else if moved_f[i] < 5.0 {
                moved_f[i] = 5.0;
            }
        }
    }
    moved_f
}

/* ============================================================================== */

// standard annealer
pub fn standard(chip : &chip_info::ChipInfo, f : &mut Vec<f64>, number_of_passes : i64, threshold : f64, sigma : f64) {
    let (_,mut current_yield_rate) = simulation::complete_yield_simulation(&chip, sigma, &f);
    let mut range = rand::thread_rng();
    for i in 0..number_of_passes {
        println!();
        if current_yield_rate > threshold {
            // met threshold requirement
            println!("Found suitable yield rate {:.3}%.",current_yield_rate);
            break;
        }
        // set the temperature
        let mut temperature : f64 = -0.01 * i as f64;
        temperature = temperature.exp();
        let f_with_move = return_moved_freq(f);
        // recompute yield rate
        let (_,moved_yield_rate) = simulation::complete_yield_simulation(&chip, sigma, &f_with_move);

        println!("{}: Considering new {:.3}% against current {:.3}% at temperature {:.3}.", i, moved_yield_rate, current_yield_rate, temperature);

        if moved_yield_rate > current_yield_rate {
            // accept the move
            println!("Higher yield rate forced transition.");
            *f = f_with_move;
            current_yield_rate = moved_yield_rate;
        } else {
            // this is a bad move
            let mut prob : f64 = 1.0;
            if current_yield_rate - moved_yield_rate != 0.0 {
                prob = (1.4*temperature)/(1.0+(1.2*(current_yield_rate-moved_yield_rate)).exp());
                if moved_yield_rate == 0.0 {
                    // further punishment
                    prob *= 0.5;
                }
            }
            println!("Lower yield rate with diff {:.3} will move with {:.3} probability",current_yield_rate-moved_yield_rate,prob);
            // deciding to make the move
            if prob > range.gen_range(0.0, 1.0) {
                // make the move
                println!("Move made despite lower yield rate.");
                *f = f_with_move;
                current_yield_rate = moved_yield_rate;
            }
        }
    }
}

// segmented annealer
pub fn segmented(chip : &chip_info::ChipInfo, f : &mut Vec<f64>, number_of_passes : i64, threshold : f64, sigma : f64) {
    let (_,mut current_yield_rate) = simulation::complete_yield_simulation(&chip, sigma, &f);
    // generate the segment number
    let mut range = rand::thread_rng();
    let mut number_of_segment_passes = 0;
    let mut segment_number : i64 = Uniform::from(0..3).sample(&mut range);
    // begin annealing
    for i in 0..number_of_passes {
        println!();
        if current_yield_rate > threshold {
            // met threshold requirement
            println!("Found suitable yield rate {:.3}%.",current_yield_rate);
            break;
        }
        // set the temperature
        let mut temperature : f64 = -0.01 * i as f64;
        temperature = temperature.exp();
        let f_with_move = return_moved_freq_segment(segment_number, f);
        // recompute yield rate
        let (_,moved_yield_rate) = simulation::complete_yield_simulation(&chip, sigma, &f_with_move);

        println!("{}/{}: Considering new {:.3}% against current {:.3}% at temperature {:.3}.", segment_number, i, moved_yield_rate, current_yield_rate, temperature);

        if moved_yield_rate > current_yield_rate {
            // accept the move
            println!("Higher yield rate forced transition.");
            *f = f_with_move;
            current_yield_rate = moved_yield_rate;
        } else {
            // this is a bad move
            let mut prob : f64 = 1.0;
            if current_yield_rate - moved_yield_rate != 0.0 {
                prob = (1.4*temperature)/(1.0+(1.2*(current_yield_rate-moved_yield_rate)).exp());
                if moved_yield_rate == 0.0 {
                    // further punishment
                    prob *= 0.5;
                }
            }
            println!("Lower yield rate with diff {:.3} will move with {:.3} probability",current_yield_rate-moved_yield_rate,prob);
            // deciding to make the move
            if prob > Uniform::from(0.0..1.0).sample(&mut range) {
                // make the move
                println!("Move made despite lower yield rate.");
                *f = f_with_move;
                current_yield_rate = moved_yield_rate;
            }
        }
        number_of_segment_passes += 1;
        if number_of_segment_passes > 20 {
            // change the segment
            segment_number = Uniform::from(0..3).sample(&mut range);
            // reset
            number_of_segment_passes = 0;
        }
    }
}