use std::time::Instant;

fn main() {
    let start = Instant::now();
    let time = start.elapsed();
    println!("time:{:?}", time);
}
