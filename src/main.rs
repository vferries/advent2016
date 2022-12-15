use advent2016::days::day12::day12;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day12();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
