// let x: f64 = 1.2;
// let y = x as i32;
// println!("{} -> {}", x, y);

const THRESHOLD: i32 = 10;
fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    println!("{}", THRESHOLD);
    println!("{}", is_big(5));

    THRESHOLD = 5;
}
