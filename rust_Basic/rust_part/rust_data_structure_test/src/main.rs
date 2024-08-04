// Problem [1]
// use std::collections::HashMap;
// fn main() {
//     let fruits = vec!["apple", "banana", "apple", "banana","orange", "pear", "orange"];
//     let mut counts: HashMap<&str, i32> = HashMap::new();

//     for fruit in &fruits {
//         *counts.entry(fruit).or_insert(0) += 1;
//     }
//     println!("{:?}", counts);
// }

// Problem [2]
// fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
//     if denominator == 0.0 {
//         Err("Cannot divide by zero")
//     } else {
//         Ok(numerator / denominator)
//     }
// }

// fn main() {
//     let result =divide(2 as f64, 3 as f64);

//     match result {
//         Ok(x) => println!("Result: {}", x),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// // Problem [3]
// fn get_square_root(number: f64) -> Option<f64> {
//     if number >= 0.0 {
//         Some(number.sqrt())
//     } else {
//         None
//     }
// }
// fn main() {
//     let result = get_square_root(9.0);

//     match result {
//         Some(x) => println!("9^0.5: {}", x),
//         None => println!("음수는 입력할 수 없습니다."),
//     }
// }

// // Problem [4]
// fn main() {
//     let nums: Vec<i32> = vec![1, 2, 3];
    
//     let maps: Vec<i32> = nums.iter().map(|x| x * 2).collect();
//     println!("{:?}", maps);
// }

// // Problem [5]
fn main() {
    let nums: Vec<i32> = vec![1, 2, 3];

    let filters: Vec<i32> = nums.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("{:?}", filters);
}