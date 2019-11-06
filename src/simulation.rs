use crate::populate;
use crate::chip_info;
use std::convert::TryInto;
use rand::distributions::{Normal, Distribution};

pub fn one_simulation(qubit_num : i64, chip : &chip_info::ChipInfo, sigma : f64, frequency_config : Vec<f64>) -> (i64, i64, [i64;7]) {
    let delta = 0.34;
    let mut freq_list : Vec<f64> = vec![];
    // populate the freq_list
    for (i,_) in (0..qubit_num).enumerate() {
        // generate gaussian noise
        let normal = Normal::new(frequency_config[i], sigma);
        let v = normal.sample(&mut rand::thread_rng());
        freq_list.push(v);
    }

    let mut yield_success = 1;
    let mut collision_number = 0;
    let mut collision_counter = [0;7];

    for e in &chip.coupling_list {
        let qj : usize = e[0].try_into().unwrap();
        let qk : usize = e[1].try_into().unwrap();
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
        let qj : usize = e[0].try_into().unwrap();
        let qk : usize = e[1].try_into().unwrap();
        if (freq_list[qj]-freq_list[qk]).abs() > delta {
            yield_success = 0;
            collision_number += 1;
            collision_counter[3] += 1;
        }
    }

    for e in &chip.via_edge_list {
        let qi : usize = e[0].try_into().unwrap();
        let qk : usize = e[2].try_into().unwrap();
        let edge_delta = (freq_list[qi]-freq_list[qk]).abs();
        if edge_delta < 0.017 {
            yield_success = 0;
            collision_number += 2;
            collision_counter[4] += 1;
        }
        if (edge_delta-delta).abs() < 0.025 {
            yield_success = 0;
            collision_number += 1;
            collision_counter[5] == 1;
        }
    }

    for (qj,_) in (0..qubit_num).enumerate(){
        for (qiid,_) in (0..(chip.edge_list[qj]).len()).enumerate() {
            let qi : usize = chip.edge_list[qj][qiid].try_into().unwrap();
            for (qkid,_) in (qiid+1..(chip.edge_list[qj]).len()).enumerate(){
                let qk : usize = chip.edge_list[qj][qkid].try_into().unwrap();
                if (2.0*freq_list[qj]-delta)-(freq_list[qk]+freq_list[qi]).abs() < 0.017 {
                    yield_success = 0;
                    collision_number += 1;
                    collision_counter[6] += 1;
                }
            }
        }
    }

    (collision_number,yield_success,collision_counter)
}