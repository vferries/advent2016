use advent2016::days::day7::day7;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day7();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
