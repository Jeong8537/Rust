//add 함수 // fn add(num1: i32, num2: i32) -> i32 {
// //     num1 + num2 //return, ; 생략가능
// // }

//swap 함수 //fn swap(num1: i32, num2: i32) -> (i32, i32) {
//     (num2, num1)
// }
// fn main() {
//     let (num1, num2) = swap(1, 2);
//     println!("{num1}, {num2}");
//     // println!("{}", add(1, 2));
// }

//Scope 개념
// fn hello(name: String) {
//     let num = 3;
//     println!("Hello {}", name);
// }

// fn main() {
//     let my_name = "buzzi".to_string();
//     {
//         println!("My name is {}", my_name);
//         let my_name = "mellon";
//     }
//     hello(my_name);
// }

//closure [1] - 1 line
// fn main() {
//     let my_func = |x: i32| -> i32 { x + 1 };
//     println!("{}", my_func(3));
// }

//closure [2] - n lines
// fn main() {
//     let my_func = |mut x: i32| {
//         x = x + 1;
//         println!("{}", x);
//     };
//     my_func(3);
// }

//fibonacci
// fn fib(n: u32) -> u32 {
//     let cache = vec![0, 1];
//     let _fib = |n| {
//         if n < cache.len() {
//             cache[n]
//         } else {
//             let result = _fib(n - 1) + _fib(n - 2);
//             cache.push(result);
//             result
//         }
//     };
//     _fib(n)
// }

// fn main() {
//     println!("{}", fib(10));
// }

//meta_Programming
// macro_rules! get_sum {
//     ($($x:expr), *) => {{
//         let args = vec![$($x),*];

//         args.iter().sum::<i32>()
//     }};
// }

// fn main() {
//     println!("{}", get_sum!(1, 2));
//     println!("{}", get_sum!(1, 2, 3));
// }

