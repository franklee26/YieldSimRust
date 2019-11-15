//! Brute force algorithms
use crate::chip_info;
use crate::simulation;
use std::process;

pub fn brute_force_2(chip: &chip_info::ChipInfo) -> f64 {
    if chip.qubit_num != 2 {
        println!("brute_force_2 expected 2q chip");
        process::exit(1);
    }
    let mut counter: i64 = 0;
    let mut running_max: f64 = 0.0;
    let mut running_f: Vec<f64> = vec![];
    let freq_space: Vec<f64> = (500..535).map(|f| f as f64 / 100.0).collect();
    for i1 in &freq_space {
        for i2 in &freq_space {
            let mut f = vec![*i1, *i2];
            let (_, temp_yield_rate) = simulation::complete_yield_simulation(&chip, chip.sigma, &f);
            if temp_yield_rate > running_max {
                //println!("Found new max {:.3}%", temp_yield_rate);
                running_max = temp_yield_rate;
                running_f = f;
            }
            counter += 1;
        }
    }
    running_max
}

pub fn brute_force_3(chip: &chip_info::ChipInfo) -> f64 {
    if chip.qubit_num != 3 {
        println!("brute_force_3 expected 3q chip");
        process::exit(1);
    }
    let mut counter: i64 = 0;
    let mut running_max: f64 = 0.0;
    let mut running_f: Vec<f64> = vec![];
    let freq_space: Vec<f64> = (500..535).map(|f| f as f64 / 100.0).collect();
    for i1 in &freq_space {
        for i2 in &freq_space {
            for i3 in &freq_space {
                let mut f = vec![*i1, *i2, *i3];
                let (_, temp_yield_rate) =
                    simulation::complete_yield_simulation(&chip, chip.sigma, &f);
                if temp_yield_rate > running_max {
                    //println!("Found new max {:.3}%", temp_yield_rate);
                    running_max = temp_yield_rate;
                    running_f = f;
                }
                counter += 1;
            }
        }
    }
    running_max
}

pub fn brute_force_4(chip: &chip_info::ChipInfo) -> f64 {
    if chip.qubit_num != 4 {
        println!("brute_force_4 expected 4q chip");
        process::exit(1);
    }
    let mut counter: i64 = 0;
    let mut running_max: f64 = 0.0;
    let mut running_f: Vec<f64> = vec![];
    let freq_space: Vec<f64> = (500..535).map(|f| f as f64 / 100.0).collect();
    for i1 in &freq_space {
        for i2 in &freq_space {
            for i3 in &freq_space {
                for i4 in &freq_space {
                    let mut f = vec![*i1, *i2, *i3, *i4];
                    let (_, temp_yield_rate) =
                        simulation::complete_yield_simulation(&chip, chip.sigma, &f);
                    if temp_yield_rate > running_max {
                        // /println!("Found new max {:.3}%", temp_yield_rate);
                        running_max = temp_yield_rate;
                        running_f = f;
                    }
                    counter += 1;
                }
            }
        }
    }
    running_max
}

pub fn brute_force_6(chip: &chip_info::ChipInfo) -> f64 {
    if chip.qubit_num != 6 {
        println!("brute_force_6 expected 6q chip");
        process::exit(1);
    }
    let mut counter: i64 = 0;
    let mut running_max: f64 = 0.0;
    let mut running_f: Vec<f64> = vec![];
    let freq_space: Vec<f64> = (500..535).map(|f| f as f64 / 100.0).collect();
    for i1 in &freq_space {
        for i2 in &freq_space {
            for i3 in &freq_space {
                for i4 in &freq_space {
                    for i5 in &freq_space {
                        for i6 in &freq_space {
                            let mut f = vec![*i1, *i2, *i3, *i4, *i5, *i6];
                            if counter % 500 == 0 {
                                println!("Step {}", counter);
                            }
                            let (_, temp_yield_rate) =
                                simulation::complete_yield_simulation(&chip, chip.sigma, &f);
                            if temp_yield_rate > running_max {
                                println!("Found new max {:.3}%", temp_yield_rate);
                                running_max = temp_yield_rate;
                                running_f = f;
                            }
                            counter += 1;
                        }
                    }
                }
            }
        }
    }
    running_max
}
