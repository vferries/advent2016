use advent2016::days::day18::day18;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day18();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
