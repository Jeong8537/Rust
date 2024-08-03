// Vec 선언
// fn main() {
//     let vec1 = Vec::from([1, 2, 3]);
//     let vec2 = vec![1, 2, 3];
//     let vec3 = Vec<i32> = Vec::new();
//     let vec4 = Vec<i32> = vec![1, 2, 3];
// }

// Vec 원소 접근
// fn main() {
//     let vec1 = vec![1, 2, 3];

//     let num = vec1[1];

//     println!("{}", num);
// }

// Vec 원소 추가
// fn main() {
//     let mut vec1 = vec![1, 2, 3];

//     vec1.push(4);
//     vec1.push(5);
//     vec1.push(6);

//     println!("{:?}", vec1);
// }

// Vec 원소 값 삭제
// fn main() {
//     let mut vec1 = vec![1, 2, 3];

//     let num1 = vec1.pop().unwrap();
//     let num2 = vec1.remove(0);

//     println!("{} {} {:?}", num1, num2, vec1);
// }

// Deque
// fn fib(n: u32) -> u32 {
//     fn _fib(n: u32, cache: &mut Vec<u32>) -> u32 {
//         if n < cache.len() as u32 {
//             cache[n as usize]
//         } else {
//             let result = _fib(n - 1, cache) + _fib(n - 2, cache);
//             cache.push(result);
//             result
//         }
//     }

//     let mut cache = vec![0, 1];
//     _fib(n, &mut cache)
// }

// fn main() {
//     println!("{}", fib(10));
// }

// Vecdeque
// use std::collections::VecDeque;

// fn main() {
//     let mut deq = VecDeque::from([1, 2, 3]);
//     println!("{}", deq.pop_front().unwrap());
// }

// Array
// fn main() {
//     let months = [
//         "January",
//         "February",
//         "March",
//         "April",
//         "May",
//         "June",
//         "July",
//         "August",
//         "September",
//         "October",
//         "November",
//         "December",
//     ];

//     println!("{:?}", months);

//     let months = [3; 5]; //배열 초기화
//     println!("{:?}", months);
// }

// Array elements reference
// fn main() {
//     let mut nums = [3; 5];
//     nums[1] = 1;
//     println!("{:?}", nums);
// }

// Tuple
// fn main() {
//     let tup1 = (0, 0.1, "hello");
//     let tup2: (i32, f64, &str) = (1, 1.01, "bye");

//     let (_, y, _) = tup2;

//     println!("tup1 has {:?} and the value of y is : {}", tup1, y)
// }

// Tuple elements reference
// fn main() {
//     let tup1 = (0, 0.1, ("hello", "world"));

//     println!("{} {}", tup1.2 .0, tup1.2 .1);
// }

// Tuple invariability
// fn main() {
//     let mut tup1 = (0, 0.1, "hello");

//     let mut x = tup1.0;
//     let (_, mut y, _) = tup1;

//     x = 1;
//     y = 1.1;

//     println!("{:?} {} {}", tup1, x, y);

//     tup1.0 = 4;
// }

// use std::collections::HashMap;

// fn main() {
//     let mut songs = HashMap::from([
//         ("Toto", "Africa"),
//         ("Post Malone", "Rockstar"),
//         ("twenty one pilots", "Stressed Out"),
//     ]);

//     println!("-----playlist-----");
//     if songs.contains_key("Toto") && songs.values().any(|&val| val == "Africa") {
//         println!("Toto's Africa is the best song!");
//     }

//     songs.insert("a-ha", "Take on Me");
//     songs.entry("Post Malone").and_modify(|v| *v = "Happier");

//     for (artist, title) in songs.iter() {
//         println!("{} - {}", artist, title);
//     }

//     println!("------------------");
//     songs.remove("Post Malone");
//     println!(
//         "{:?}",
//         songs
//             .get("Post Malonr")
//             .unwrap_or(&"Post Malone is not in the playlist.")
//     );
// }

// fn main() {
//     let mut s = String::new();

//     let data = "initial contents";
//     let s = data.to_string();
//     let s = "initial contents".to_string();

//     let s = String::from("initial contents");
// }

// String slice
// fn main() {
//     let greet = String::from("Hi, buzzi!");

//     let name = &greet[4..];
//     println!("{}", name);
// }

fn main() {
    let greet = String::from("Hi😊 buzzi!");
    let greet_chars: Vec<char> = greet.chars().collect();
    let name = &greet_chars[4..].iter().collect::<String>();
    println!("{:?}", name);
}
