mod optimized_schedule;
mod simple_schedule;

fn main() {
    let (degree, blowup_factor, num_queries, remainder_max_degree) = (1 << 25, 8, 27, 64);

    let (opt_size, opt_schedule) =
        optimized_schedule::optimal_folding_strategy(degree, blowup_factor, num_queries, vec![0]);

    println!(
        "The optimal size {} kBs and optimal folding schedule {:?}",
        optimized_schedule::size_in_bytes(opt_size) / 1024,
        opt_schedule
    );

    for i in 1..=4 {
        let (size, schedule) = simple_schedule::simple_schedule(
            degree,
            blowup_factor,
            num_queries,
            remainder_max_degree,
            i,
        );

        println!(
            "Folding factor {} size {} kBs and folding sequence {:?}",
            1 << i,
            optimized_schedule::size_in_bytes(size) / 1024,
            schedule
        );
    }
}
