use advent2016::days::day10::day10;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day10();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
