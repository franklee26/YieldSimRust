//! IBM's provided frequency-selection algorithms
use crate::chip_info;
use crate::simulation;
use std::process;

/// IBM's frequency selection for a 6 qubit chip
pub fn one_6q_trial(chip: &chip_info::ChipInfo) -> f64 {
    if chip.qubit_num != 6 {
        println!("one_6q_trial expected 6q chip");
        process::exit(1);
    }
    // pre defined frequency allocation
    let mut f: Vec<f64> = vec![5.00, 5.07, 5.13, 5.20, 5.27, 5.00];
    let (_, yield_rate) = simulation::complete_yield_simulation(chip, chip.sigma, &f);
    yield_rate
}

/// IBM's frequency selection for a 10 qubit chip
pub fn one_10q_trial(chip: &chip_info::ChipInfo) -> f64 {
    if chip.qubit_num != 10 {
        println!("one_10q_trial expected 10q chip");
        process::exit(1);
    }
    // pre defined frequency allocation
    let mut f: Vec<f64> = vec![5.00, 5.07, 5.13, 5.20, 5.27, 5.00, 5.07, 5.13, 5.20, 5.27];
    let (_, yield_rate) = simulation::complete_yield_simulation(chip, chip.sigma, &f);
    yield_rate
}

/// IBM's frequency selection for a 12 qubit chip
pub fn one_12q_trial(chip: &chip_info::ChipInfo) -> f64 {
    if chip.qubit_num != 12 {
        println!("one_12q_trial expected 12q chip");
        process::exit(1);
    }
    // pre defined frequency allocation
    let mut f: Vec<f64> = vec![
        5.00, 5.07, 5.13, 5.20, 5.27, 5.00, 5.07, 5.13, 5.20, 5.27, 5.00, 5.07,
    ];
    let (_, yield_rate) = simulation::complete_yield_simulation(chip, chip.sigma, &f);
    yield_rate
}

/// IBM's frequency selection for a 17 qubit chip
pub fn one_17q_trial(chip: &chip_info::ChipInfo) -> f64 {
    if chip.qubit_num != 17 {
        println!("one_17q_trial expected 17q chip");
        process::exit(1);
    }
    // pre defined frequency allocation
    let mut f: Vec<f64> = vec![
        5.00, 5.07, 5.13, 5.20, 5.27, 5.00, 5.07, 5.13, 5.20, 5.27, 5.00, 5.07, 5.13, 5.20, 5.27,
        5.00,
    ];
    let (_, yield_rate) = simulation::complete_yield_simulation(chip, chip.sigma, &f);
    yield_rate
}

/// IBM's frequency selection for a 25 qubit chip
pub fn one_25q_trial(chip: &chip_info::ChipInfo) -> f64 {
    if chip.qubit_num != 25 {
        println!("one_25q_trial expected 25q chip");
        process::exit(1);
    }
    // pre defined frequency allocation
    let mut f: Vec<f64> = vec![
        5.00, 5.07, 5.13, 5.20, 5.27, 5.00, 5.07, 5.13, 5.20, 5.27, 5.00, 5.07, 5.13, 5.20, 5.27,
        5.00, 5.07, 5.13, 5.20, 5.27, 5.00, 5.07, 5.13, 5.20, 5.00,
    ];
    let (_, yield_rate) = simulation::complete_yield_simulation(chip, chip.sigma, &f);
    yield_rate
}

/// IBM's frequency selection for a 30 qubit chip
pub fn one_30q_trial(chip: &chip_info::ChipInfo) -> f64 {
    if chip.qubit_num != 30 {
        println!("one_30q_trial expected 30q chip");
        process::exit(1);
    }
    // pre defined frequency allocation
    let mut f: Vec<f64> = vec![
        5.00, 5.07, 5.13, 5.20, 5.27, 5.00, 5.07, 5.13, 5.20, 5.27, 5.00, 5.07, 5.13, 5.20, 5.27,
        5.00, 5.07, 5.13, 5.20, 5.27, 5.00, 5.07, 5.13, 5.20, 5.00, 5.07, 5.13, 5.20, 5.27, 5.00,
    ];
    let (_, yield_rate) = simulation::complete_yield_simulation(chip, chip.sigma, &f);
    yield_rate
}
