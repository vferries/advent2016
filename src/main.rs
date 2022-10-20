use advent2016::days::day8::day8;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        day8();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
