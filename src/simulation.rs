use crate::chip_info;
use rand::distributions::{Normal, Distribution};

pub fn one_simulation(chip : &chip_info::ChipInfo, sigma : f64, frequency_config : &Vec<f64>) -> (i64, i64, [i64;7]) {
    let delta : f64 = 0.34;
    let mut freq_list : Vec<f64> = vec![];
    // populate the freq_list
    for i in 0..chip.qubit_num as usize {
        // generate gaussian noise
        let normal = Normal::new(frequency_config[i], sigma);
        let v = normal.sample(&mut rand::thread_rng());
        freq_list.push(v);
    }
    let mut yield_success = 1;
    let mut collision_number = 0;
    let mut collision_counter = [0;7];

    for e in &chip.coupling_list {
        let qj : usize = e[0] as usize;
        let qk : usize = e[1] as usize;
        let edge_delta = (freq_list[qj] - freq_list[qk]).abs();
        if edge_delta < 0.017 {
            yield_success = 0;
            collision_number += 2;
            collision_counter[0] += 1;
        }
        if (edge_delta - (delta/2.0)).abs() < 0.004 {
            yield_success = 0;
            collision_number += 1;
            collision_counter[1] += 1
        }
        if (edge_delta - delta).abs() < 0.025 {
            yield_success = 0;
            collision_number += 1;
            collision_counter[2] += 1;
        }
    }

    for e in &chip.grid_edge_list {
        let qj : usize = e[0] as usize;
        let qk : usize = e[1] as usize;
        if (freq_list[qj]-freq_list[qk]).abs() > delta {
            yield_success = 0;
            collision_number += 1;
            collision_counter[3] += 1;
        }
    }

    for e in &chip.via_edge_list {
        let qi : usize = e[0] as usize;
        let qk : usize = e[2] as usize;
        let edge_delta = (freq_list[qi]-freq_list[qk]).abs();
        if edge_delta < 0.017 {
            yield_success = 0;
            collision_number += 2;
            collision_counter[4] += 1;
        }
        if (edge_delta-delta).abs() < 0.025 {
            yield_success = 0;
            collision_number += 1;
            collision_counter[5] += 1;
        }
    }
    for qj in 0..chip.qubit_num as usize {
        for qiid in 0..(chip.edge_list[qj]).len() {
            let qi : usize = chip.edge_list[qj][qiid] as usize;
            for qkid in qiid+1..(chip.edge_list[qj]).len() {
                let qk : usize = chip.edge_list[qj][qkid] as usize;
                if ((2.0*freq_list[qj]-delta)-(freq_list[qk]+freq_list[qi])).abs() < 0.017 {
                    yield_success = 0;
                    collision_number += 1;
                    collision_counter[6] += 1;
                }
            }
        }
    }
    (collision_number,yield_success,collision_counter)
}

pub fn complete_yield_simulation(chip : &chip_info::ChipInfo, sigma : f64, frequency_config : &Vec<f64>) -> (f64,f64) {
    const number_of_trials : usize = 10000;
    let mut collision_per_trial : [i64;number_of_trials] = [0;number_of_trials];
    //let mut yield_array = [0;number_of_trials];
    let mut yield_sum = 0;
    let mut collision_counter = [0;7];
    for trial in 0..number_of_trials {
        let (a,b,c) = one_simulation(&chip, sigma, &frequency_config);
        collision_per_trial[trial] = a;
        //yield_array[trial] = b;
        yield_sum += b;
        for i in 0..7 {
            collision_counter[i] += c[i];
        }
    }
    let collision_number : f64 = (collision_per_trial.iter().sum::<i64>() as f64)/(number_of_trials as f64);
    //let yield_rate : f64 = (yield_array.iter().sum::<i64>() as f64)/(number_of_trials as f64);
    let yield_rate : f64 = (yield_sum as f64)*100.0/(number_of_trials as f64);
    (collision_number,yield_rate)
}