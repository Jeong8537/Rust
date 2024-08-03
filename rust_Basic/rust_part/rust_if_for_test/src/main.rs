// fn check_sign(n: i32) {
//     if n % 2 == 0 {
//         println!("positive")
//     } else {
//         println!("negative")
//     }
// }

fn main() {
    for i in 1..=10 {
        for _ in 10..=-i {
            print!("*");
        }
        println!();
    }
    // check_sign(3);
}
