use crate::simple_schedule::simple_schedule;

mod optimized_schedule;
mod simple_schedule;

fn main() {
    let (opt_size, opt_schedule) =
        optimized_schedule::optimal_folding_strategy(1 << 25, 8, 1, vec![0]);

    println!(
        "The size and folding factor are: {} {:?}",
        opt_size, opt_schedule
    );

    for i in 1..=4 {
        let (size, schedule) = simple_schedule(1 << 25, 8, 1, 64, i);

        println!("The size and folding factor are: {} {:?}", size, schedule);
    }
}
