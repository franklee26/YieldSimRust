/*
Define the various transition probability distributions here.
Naming format: [TEMPERATURE DECAY TYPE]_[DISTRIBUTION TYPE]_[PARAM LENIENCY]
*/

enum distribution {
    boltzmann,
    fermi_dirac,
    random,
}

// boltzmann distribution (defines temperature decay type) using custom parameters
pub fn boltzmann_custom(
    iteration_number: i64,
    delta: f64,
    delta_param: f64,
    scaling_param: f64,
) -> f64 {
    scaling_param * (-delta_param * delta / (iteration_number as f64 + 0.001)).exp()
}

// exponential temperature decay scaled with fermi-dirac using custom parameters
pub fn exp_fermi_dirac_custom(
    iteration_number: i64,
    delta: f64,
    temp_param: f64,
    delta_param: f64,
    scaling_param: f64,
) -> f64 {
    let temperature: f64 = (-temp_param * iteration_number as f64).exp();
    scaling_param * temperature / (1.0 + (delta_param * delta).exp())
}

// exponential temperature decay scaled with fermi-dirac using standard parameters
pub fn exp_fermi_dirac_standard(iteration_number: i64, delta: f64) -> f64 {
    let temperature: f64 = (-0.025 * iteration_number as f64).exp();
    1.4 * temperature / (1.0 + (1.2 * delta).exp())
}

// exponential temperature decay scaled with fermi-dirac using lenient parameters
pub fn exp_fermi_dirac_lenient(iteration_number: i64, delta: f64) -> f64 {
    let temperature: f64 = (-0.015 * iteration_number as f64).exp();
    1.5 * temperature / (1.0 + (0.9 * delta).exp())
}

// exponential temperature decay scaled with fermi-dirac using harsh parameters
pub fn exp_fermi_dirac_harsh(iteration_number: i64, delta: f64) -> f64 {
    let temperature: f64 = (-0.03 * iteration_number as f64).exp();
    1.1 * temperature / (1.0 + (3.0 * delta).exp())
}
