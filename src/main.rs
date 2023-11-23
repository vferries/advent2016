use advent2016::days::day24::day24;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day24();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
