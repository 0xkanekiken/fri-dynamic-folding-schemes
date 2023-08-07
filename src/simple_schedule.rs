use super::optimized_schedule::estimate_proof_size;

/// Computes a schedule for a simple FRI proof. The schedule is a vector of folding factors. The
/// folding factors are represented as in the form of bits. For example, a folding factor of 4 is
/// represented as 2. The folding would stop if the degree of the polynomial to be proved is less
/// than the `remainder_max_degree`. It follows `winterfell` implementation FRI proof generation.
///
/// # Arguments
/// * `degree` - The degree of the polynomial to be proved
/// * `blowup_factor` - The blowup factor used in the FRI protocol
/// * `num_queries` - The number of queries used in the FRI protocol
/// * `remainder_max_degree` - The maximum degree of the remainder polynomial
/// * `folding_factor` - The folding factor used in the FRI protocol
///
/// # Returns
/// * `proof_size` - The estimated proof size in terms of field elements
/// * `folding_schedule` - The folding schedule
///
/// # Panics
/// * If the degree is not a power of 2
/// * If the blowup factor is not a power of 2
/// * If the folding factor is not a power of 2
/// * If the remainder poly degree is greater than the degree of the polynomial to be proved
pub(crate) fn simple_schedule(
    degree: usize,
    blowup_factor: usize,
    num_queries: usize,
    remainder_max_degree: usize,
    folding_factor: usize,
) -> (usize, Vec<usize>) {
    // The degree, blowup factor must be powers of 2.
    debug_assert!(degree.is_power_of_two());
    debug_assert!(blowup_factor.is_power_of_two());

    // The `remainder_max_degree` must be less than the degree of the polynomial to be proved.
    debug_assert!(remainder_max_degree <= degree / blowup_factor);

    // The degree of the polynomial to be proved.
    let poly_degree = degree / blowup_factor;

    // The number of rounds.
    let num_rounds = num_rounds(poly_degree, folding_factor, remainder_max_degree);

    // The folding schedule. Initially, the folding schedule contains only the first round fold.
    let mut folding_schedule = vec![0];

    // the folding schedule for subsequent rounds.
    folding_schedule.resize(num_rounds, folding_factor);

    let proof_size = estimate_proof_size(degree, blowup_factor, num_queries, &folding_schedule);

    (proof_size, folding_schedule)
}

/// Computes the number of rounds during FRI proof generation. The folding stops when the degree of
/// the polynomial to be proved is less than the `remainder_max_degree`.
fn num_rounds(degree: usize, folding_factor: usize, remainder_max_degree: usize) -> usize {
    let mut num_rounds = 1;
    let mut current_degree = degree;

    // The folding factor in absolute terms.
    let folding_factor = 1 << folding_factor;

    while current_degree > remainder_max_degree {
        current_degree /= folding_factor;
        num_rounds += 1;
    }

    num_rounds
}
