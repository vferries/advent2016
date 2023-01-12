use advent2016::days::day16::day16;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day16();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
