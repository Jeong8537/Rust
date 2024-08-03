//fn multiply problem [1]
// fn multiply(a: i32, b: i32) -> i32 {
//     a * b
// }
// fn main() {
//     let result = multiply(3, 4);
//     println!("The result is: {}", result);
// }

//fn multiply problem [2]
fn main() {
    let multiply_numbers = |a: i32, b: i32| -> i32 { a * b };
    let result = multiply_numbers(3, 4);
    println!("The result is: {}", result);
}
