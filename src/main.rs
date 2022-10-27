use advent2016::days::day9::day9;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day9();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
