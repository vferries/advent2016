use advent2016::days::day15::day15;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day15();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
