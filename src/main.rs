use advent2016::days::day14::day14;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day14();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
