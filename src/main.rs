use advent2016::days::day19::day19;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day19();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
