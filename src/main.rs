use advent2016::days::day20::day20;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day20();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
