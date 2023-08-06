const ELEMENTS_IN_HASH_OUTPUT: usize = 4;
const FE_IN_EACH_ELEMENTS: usize = 2;

/// Computes the optimal folding strategy for a FRI proof. The function uses a heuristic to estimate
/// the proof size in terms of field elements. It then iteratively explores different folding strategies
/// to identify the one yielding the smallest estimated proof size.
///
/// # Arguments
/// * `degree` - The degree of the polynomial to be proved
/// * `blowup_factor` - The blowup factor used in the FRI protocol
/// * `num_queries` - The number of queries used in the FRI protocol
/// * `current_folding_seq` - The current folding sequence. This is used to recursively explore different
/// folding strategies. The default value is an vector with a single element, 0(`bits`)), which corresponds to
/// no folding. We include first FRI layer into the FRI proof without any folding. The folding factors
/// are represented as in the form of bits. For example, a folding factor of 4 is represented as 2.
///
/// # Returns
/// * `optimal_proof` - The estimated proof size in terms of field elements
/// * `optimal_sequences` - The optimal folding sequence
///
/// # Panics
/// * If the degree is not a power of 2
/// * If the blowup factor is not a power of 2
pub(crate) fn optimal_folding_strategy(
    degree: usize,
    blowup_factor: usize,
    num_queries: usize,
    current_folding_seq: Vec<usize>,
) -> (usize, Vec<usize>) {
    // The degree and blowup factor must be powers of 2.
    debug_assert!(degree.is_power_of_two());
    debug_assert!(blowup_factor.is_power_of_two());

    let mut optimal_sequences = current_folding_seq.clone();
    let mut optimal_proof =
        estimate_proof_size(degree, blowup_factor, num_queries, &optimal_sequences);

    // The current layer degree is the degree of the polynomial at the current layer. This is
    // initialized to the degree of the polynomial to be proved. At each layer, the degree is
    // divided by the folding factor.
    let folding_sum = (1 << current_folding_seq.iter().sum::<usize>()) as usize;
    let current_layer_degree: usize = degree / folding_sum;

    // The maximum folding factor is the largest power of 2 that divides the current layer degree.
    // This is capped at 4.
    let max_folding_factor = ((current_layer_degree / blowup_factor).ilog2() as usize).min(4);

    for factor in 1..=max_folding_factor {
        let mut sequences_this_layer = current_folding_seq.clone();
        sequences_this_layer.push(factor);

        // The size of the proof is the sum of the size of the proof at the current layer and the
        // size of the proof at the next layer.
        let (size, sequences_layer) =
            optimal_folding_strategy(degree, blowup_factor, num_queries, sequences_this_layer);

        // If the size of the proof is smaller than the current optimal proof size, update the
        // optimal proof size and the optimal folding sequence.
        if size < optimal_proof {
            optimal_proof = size;
            optimal_sequences = sequences_layer;
        }
    }
    (optimal_proof, optimal_sequences)
}

/// Estimates the size of the FRI proof for a given folding strategy in terms of
/// field elements. This function provides an estimation based on a heuristic and
/// may not yield exact values, especially due to potential Merkle path compressions.
///
/// # Arguments
/// * `degree` - The degree of the polynomial to be proved
/// * `blowup_factor` - The blowup factor used in the FRI protocol
/// * `num_queries` - The number of queries used in the FRI protocol
/// * `folding_seq` - The folding sequence
///
/// # Returns
/// * `num_elements` - The estimated proof size in terms of field elements
pub(crate) fn estimate_proof_size(
    degree: usize,
    blowup_factor: usize,
    num_queries: usize,
    folding_seq: &Vec<usize>,
) -> usize {
    // The current layer degree is the degree of the polynomial at the current layer. This is
    // initialized to the degree of the polynomial to be proved. At each layer, the degree is
    // divided by the folding factor.
    let mut current_layer_degree = degree;

    // The number of elements in the proof is the sum of the number of elements in the Merkle
    // path, the number of elements in the remainder polynomial, and the number of elements in
    // nodes.
    let mut num_elements = 0;
    for folding_factors_bits in folding_seq {
        // computing the factor as 2^folding_factors_bits.
        let factor = (1 << folding_factors_bits) as usize;

        // number of elements in the Merkle path. No compression is assumed.
        num_elements +=
            num_queries * current_layer_degree.ilog2() as usize * ELEMENTS_IN_HASH_OUTPUT;

        // count neighboring elements. Neighboring field elements are hashed together
        // to form a node.
        num_elements += num_queries * factor * FE_IN_EACH_ELEMENTS;

        // update the current layer degree.
        current_layer_degree /= factor;
    }

    // remainder polynomial in coefficient form (orginal form has degree *
    // blowup_factor)
    let remainder_poly_degree: usize = current_layer_degree / blowup_factor;

    // number of elements in the remainder polynomial.
    num_elements += remainder_poly_degree * FE_IN_EACH_ELEMENTS;
    num_elements
}
