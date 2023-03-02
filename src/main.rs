use advent2016::days::day21::day21;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day21();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
