use advent2016::days::day23::day23;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day23();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
