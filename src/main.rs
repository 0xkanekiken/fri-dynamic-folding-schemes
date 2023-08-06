mod optimized_schedule;

fn main() {
    let (size, vec) =
        optimized_schedule::optimal_folding_strategy(2_u32.pow(25) as usize, 8, 1, vec![0]);

    println!("The size and folding factor are: {} {:?}", size, vec);
}
