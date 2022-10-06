use advent2016::days::day5::day5;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day5();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
