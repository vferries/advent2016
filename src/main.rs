use advent2016::days::day11::day11;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day11();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
