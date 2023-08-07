mod optimized_schedule;
mod simple_schedule;

fn main() {
    let (degree, blowup_factor, num_queries, remainder_max_degree) = (1 << 25, 8, 1, 256);

    let (opt_size, opt_schedule) =
        optimized_schedule::optimal_folding_strategy(degree, blowup_factor, num_queries, vec![0]);

    println!(
        "The optimal size {} and optimal folding schedule {:?}",
        opt_size, opt_schedule
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
            "Folding factor {} size {} and folding sequence {:?}",
            1 << i,
            size,
            schedule
        );
    }
}
