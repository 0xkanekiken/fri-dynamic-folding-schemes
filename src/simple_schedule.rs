use super::optimized_schedule::estimate_proof_size;

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

    let poly_degree = degree / blowup_factor;
    let (num_rounds, last_round_fold) =
        num_rounds(poly_degree, folding_factor, remainder_poly_degree);
    let mut folding_schedule = vec![0];
    folding_schedule.resize(num_rounds, folding_factor);

    // The last fold round must be a power of 2.
    assert!(last_round_fold.is_power_of_two() || last_round_fold == 0);
    if last_round_fold > 1 {
        folding_schedule.push(last_round_fold);
    }

    let proof_size = estimate_proof_size(degree, blowup_factor, num_queries, &folding_schedule);

    (proof_size, folding_schedule)
}

fn num_rounds(
    degree: usize,
    folding_factor: usize,
    remainder_poly_degree: usize,
) -> (usize, usize) {
    let mut num_rounds = 1;
    let mut current_degree = degree;

    let folding_factor = 1 << folding_factor;

    while current_degree / folding_factor >= remainder_poly_degree {
        current_degree /= folding_factor;
        num_rounds += 1;
    }
    let mut last_round_fold = 0;
    if current_degree > remainder_poly_degree {
        last_round_fold = (current_degree / remainder_poly_degree).ilog2() as usize;
    }

    println!(
        "num_rounds: {}, last_round_fold: {}",
        num_rounds, last_round_fold
    );
    (num_rounds, last_round_fold)
}
