mod populate;
mod simulation;
mod chip_info;
use std::time::{Instant};

/* Helpers ======================================== */

/* ================================================ */

fn main() {
    let mut IBM17Q2B : chip_info::ChipInfo = chip_info
        ::ChipInfo::new(0, vec![],vec![],vec![],vec![],vec![],vec![],vec![]);

    // populate the chip from file
    IBM17Q2B.populate_from_file("17q_bus2.chip");
    // checkup
    //IBM17Q2B.print_details();

    for _ in 0..80 {
        let (x,y) = simulation::complete_yield_simulation(17, &IBM17Q2B, 0.01, &vec![5.07, 5.27, 5.2, 5.13, 5.17, 5.23, 5.34, 5.08, 5.13, 5.05, 5.34, 5.27, 5.0, 5.1, 5.21, 5.0, 5.25]);
        println!("{:.3}%",y*100.0);
    }
}