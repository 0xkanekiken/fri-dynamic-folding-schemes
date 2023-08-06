use super::optimized_schedule::estimate_proof_size;

/// Computes a schedule for a simple FRI proof. The schedule is a vector of folding factors. The
/// folding factors are represented as in the form of bits. For example, a folding factor of 4 is
/// represented as 2. The folding would stop if the degree of the polynomial to be proved is less
/// than the remainder poly degree.
///
/// # Arguments
/// * `degree` - The degree of the polynomial to be proved
/// * `blowup_factor` - The blowup factor used in the FRI protocol
/// * `num_queries` - The number of queries used in the FRI protocol
/// * `remainder_poly_degree` - The degree of the remainder polynomial
/// * `folding_factor` - The folding factor used in the FRI protocol
///
/// # Returns
/// * `proof_size` - The estimated proof size in terms of field elements
/// * `folding_schedule` - The folding schedule
///
/// # Panics
/// * If the degree is not a power of 2
/// * If the blowup factor is not a power of 2
/// * If the remainder poly degree is not a power of 2
/// * If the folding factor is not a power of 2
/// * If the remainder poly degree is greater than the degree of the polynomial to be proved
pub(crate) fn simple_schedule(
    degree: usize,
    blowup_factor: usize,
    num_queries: usize,
    remainder_poly_degree: usize,
    folding_factor: usize,
) -> (usize, Vec<usize>) {
    // The degree, blowup factor and remainder poly degree must be powers of 2.
    debug_assert!(degree.is_power_of_two());
    debug_assert!(blowup_factor.is_power_of_two());
    debug_assert!(remainder_poly_degree.is_power_of_two());

    // The remainder poly degree must be less than the degree of the polynomial to be proved.
    debug_assert!(remainder_poly_degree <= degree / blowup_factor);

    // The degree of the polynomial to be proved.
    let poly_degree = degree / blowup_factor;

    // The number of rounds and the last round fold.
    let (num_rounds, last_round_fold) =
        num_rounds(poly_degree, folding_factor, remainder_poly_degree);

    // The folding schedule. Initially, the folding schedule contains only the first round fold.
    let mut folding_schedule = vec![0];

    // the folding schedule for subsequent rounds.
    folding_schedule.resize(num_rounds, folding_factor);

    // The last fold round must be a power of 2.
    assert!(last_round_fold.is_power_of_two() || last_round_fold == 0);
    if last_round_fold > 1 {
        folding_schedule.push(last_round_fold);
    }

    let proof_size = estimate_proof_size(degree, blowup_factor, num_queries, &folding_schedule);

    (proof_size, folding_schedule)
}

/// Computes the number of rounds and the last round fold.
fn num_rounds(
    degree: usize,
    folding_factor: usize,
    remainder_poly_degree: usize,
) -> (usize, usize) {
    let mut num_rounds = 1;
    let mut current_degree = degree;

    // The folding factor in absolute terms.
    let folding_factor = 1 << folding_factor;

    while current_degree / folding_factor >= remainder_poly_degree {
        current_degree /= folding_factor;
        num_rounds += 1;
    }

    // The last round fold.
    let mut last_round_fold = 0;
    if current_degree > remainder_poly_degree {
        last_round_fold = (current_degree / remainder_poly_degree).ilog2() as usize;
    }

    (num_rounds, last_round_fold)
}
