use advent2016::days::day17::day17;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day17();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
