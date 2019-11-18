/// Annealing algorithms
pub mod annealer;
/// Chip implementations
pub mod chip_info;
/// Transition probability distributions
pub mod distribution;
/// Helping methods
pub mod helper;
/// Parsing .chip files
pub mod populate;
/// Yield rate simulation algorithm
pub mod simulation;

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
    IBM17Q2B.populate_from_file("30q_bus2.chip");
    // checkup
    //IBM17Q2B.print_details();

    /*
    Now that we have an f that (hopefully) has a yield_rate > 0, we can
    begin annealing. We can choose a standard annealing process or we can
    choose the segmented approach:

    Annealer::anneal::random(_chip, _frequency, _number_iterations, _threshold) -> (i64,f64);
    Annealer::anneal::standard(_chip, _frequency, _number_iterations, _threshold) -> (i64,f64);
    Annealer::anneal::segmented(_chip, _frequency, _number_iterations, _threshold, _segments) -> (i64,f64);

    NOTE: need to pass the segments for the segmented annealer (default is THREE segments)
    RETURNS: tuple (final_iter_number, final_yield_number)
    */

    /* here are my segments
    let segments : Vec<Vec<usize>> = vec![vec![0,1,2,3,4,5],vec![6,7,8,9,10,11],vec![12,13,14,15,16]];
    let segments2: Vec<Vec<usize>> = vec![
        vec![0, 1, 4, 5, 8, 9],
        vec![2, 3, 6, 7, 11, 15],
        vec![10, 12, 13, 14, 16],
    ];
    let segments3 : Vec<Vec<usize>> = vec![vec![0,1,5,10,14],vec![4,8,7,9,12,13],vec![2,3,6,11,15,16]];
    let segments4 : Vec<Vec<usize>> = vec![vec![0,2,8,10,13,16],vec![1,4,7,9,11,14],vec![3,5,6,12,15]];
    let temp_seg: Vec<Vec<usize>> = vec![
        vec![0, 1, 2, 5, 6, 7],
        vec![3, 4, 8, 9, 12, 13],
        vec![10, 11, 14, 15, 16, 17],
        vec![21, 22, 23, 27, 28, 29],
        vec![18, 19, 20, 24, 25, 26],
    ];

    let bigger_seg: Vec<Vec<usize>> = vec![
        vec![0, 1, 2, 5, 6, 7],
        vec![3, 4, 9, 12, 13, 17],
        vec![8, 10, 11, 14, 15, 16],
        vec![18, 19, 20, 24, 25, 26],
        vec![21, 22, 23, 27, 28, 29],
    ];

    let chain_seg: Vec<Vec<usize>> = vec![
        vec![0, 1, 2, 3, 4, 28],
        vec![5, 24, 7, 8, 9, 10],
        vec![23, 12, 13, 21, 15, 16],
        vec![17, 19, 20, 14, 22, 1],
        vec![18, 6, 25, 26, 27, 29],
    ];
    */

    // let seg_25: Vec<Vec<usize>> = vec![
    //     vec![2, 3, 8, 9, 23, 24, 22],
    //     vec![0, 1, 4, 5, 6, 10, 15],
    //     vec![11, 14, 16, 18, 21, 10],
    //     vec![7, 12, 13, 17, 19, 20, 8, 9],
    // ]
    // I'm going to run 100 trials
    // let mut iterations: Vec<i64> = vec![];
    // let mut the_yields: Vec<f64> = vec![];
    let mut yield_sum: f64 = 0.0;
    for i in 0..100 {
        //let mut f: Vec<f64> = helper::global_hunter(&IBM17Q2B, 10);
        //let (iter_number, yields) = Annealer::segmented(&IBM17Q2B, &mut f, 280, 0.1, &seg_25);
        //let (iter_number, yields) = Annealer::random(&IBM17Q2B, &mut f, 280, 0.1);
        //let (iter_number, yields) = annealer::anneal::standard(&IBM17Q2B, &mut f, 280, 0.1);
        // iterations.push(iter_number);
        // the_yields.push(yields);
        //println!("{}: {} {}", i, iter_number, yields);
        let yield_temp = annealer::base_line::one_30q_trial(&IBM17Q2B);
        yield_sum += yield_temp;
        //println!("{}", yield_temp);
    }
    println!("{:.3} average.", yield_sum / 100.0)
    // write this data to file for analysis
    //helper::write_to_file_data(&iterations, &the_yields, "100_trials_seg25_3.txt");
}
