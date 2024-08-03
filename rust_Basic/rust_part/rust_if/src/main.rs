// fn check_password(password: i32) -> bool {
//     if password == 1234 {
//         true
//     } else {
//         false
//     }
// }

fn main() {
    // let x = 1.0;
    // let y = 10;

    // let password = 1234;
    // let result = check_password(password);
    // println!("Password check: {}", result);

    // if 문
    // if x < (y as f64) {
    //     println!("x is less than y");
    // } else if x == (y as f64) {
    //     println!("x is equal to y");
    // } else {
    //     println!("x is not less than y");
    // }

    // let if문
    // let result = if x < (y as f64) {
    //     "x is less than y"
    // } else if x == (y as f64) {
    //     "x is equal to y"
    // } else {
    //     " x is not less than y"
    // };

    // println!("{}", result);

    // match 문 [1]
    // let name = "John";
    // match name {
    //     "John" => println!("Hello, John!"),
    //     "Alice" => println!("Hello, Alice!"),
    //     _ => println!("Hello, stranger!"),
    // }

    // match 문 [2]
    let name = "John";
    let greet = match name {
        "John" => "Hello, John!",
        "Alice" => "Hello, Alice!",
        _ => "Hello, stranger!",
    };
    println!("{}", greet);
}
