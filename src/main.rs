use advent2016::days::day6::day6;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day6();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
