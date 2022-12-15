use advent2016::days::day13::day13;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day13();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
