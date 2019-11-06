mod populate;
mod chip_info;

/* Helpers ======================================== */

/* ================================================ */

fn main() {

    let mut IBM17Q2B : chip_info::ChipInfo = chip_info
        ::ChipInfo::new(0, vec![],vec![],vec![],vec![],vec![],vec![],vec![]);

    // populate the chip from file
    IBM17Q2B.populate_from_file("17q_bus2.chip");
    // checkup
    IBM17Q2B.print_details();
}