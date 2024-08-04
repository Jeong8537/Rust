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

// fn main() {
//     let greet = String::from("Hi😊 buzzi!");
//     let greet_chars: Vec<char> = greet.chars().collect();
//     let name = &greet_chars[4..].iter().collect::<String>();
//     println!("{:?}", name);
// }

// enumeration [1]
// #[derive(Debug)]

// enum Languages {
//     Python,
//     Rust,
//     Javascript,
//     Go,
// }

// impl Languages {
//     fn echo(&self) {
//         println!("{:?}", &self);
//     }
// }

// fn main() {
//     let language = Languages::Rust;
//     language.echo();

//     match language {
//         Languages::Python => println!("I Love Python!"),
//         Languages::Go => println!("I Love Go!"),
//         Languages::Javascript => println!("I Love Javascript!"),
//         _ => println!("I Love Rust!🍕")
//     }
// }

// enumeration [2]
// #[derive(Debug)]

// enum Grade {
//     A,
//     B,
//     C,
// }

// enum Job {
//     Student(Grade, String),
//     Developer(String),
// }

// fn main() {
//     let indo = Job::Student(Grade::A, "Indo".to_string());

//     match indo{
//         Job::Student(grade, name) => {
//             println!("{} is a student with grade {:?}", name, grade);
//         }
//         Job::Developer(name) => {
//             println!("{} is a developer", name);
//         }
//     }
// }

// Option<T> [1]
// fn main() {
//     let some_number = Some(5);
//     let some_string = Some("Hello, world!");

//     let absent_number: Option<i32> = None;

//     println!("{:?} {:?} {:?}", some_number, some_string, absent_number);
// }


// Option<T> with match
// fn check_len(vec: Vec<i32>) -> Option<usize> {
//     match vec.len() {
//         0 => None,
//         _ => Some(vec.len()),
//     }
// }

// fn main() {
//     let nums = vec![1, 2, 3];

//     match check_len(nums) {
//         Some(len) => println!("Length: {}", len),
//         None => println!("No elements!"),
//     }
// }

// if let
// fn main() {
//     let val = Some(5);
//     match val{
//         Some(5) => println!("Five!"),
//         _ => (),
//     }

//     if let Some(5) = val {
//         println!("Five!");
//     }
// }


// Result<T, E> - match
// use std::fs::File;
// use std::io::Read;

// fn main() {
//     let mut s = String::new();
//     match File::open("hello.txt") {
//         Ok(mut File) => {
//             file.read_to_string(&mut s).unwrap();
//             println!("{}", s);
//         }
//         Err(error) => panic!("Error opening file: {:?}", error),
//     };
// }

// Result<T, E> - if let
// use std::fs::File;
// use std::io::Read;

// fn main() {
//     let mut s = String::new();
//     if let Ok(mut file) = File::open("hello.txt") {
//         file.read_to_string(&mut s).unwrap();
//         println!("{}", s);
//     } else {
//         panic!("Error opening file");
//     }
// }

// Result<T, E> - ? 연산자

// use std::fs::File;
// use std::io::{self, Write};

// fn write_info(name: &str) -> io::Result<()> {
//     match File::create("my_best_friends.txt") {
//         Err(e) => return Err(e),
//         OK(mut f) => {
//             if let Err(e) = f.write_all(format!("name: {}\n", name).as_bytes()) {
//                 return Err(e);
//             }
//         }
//     };
//     Ok(())
// }

// fn main() {
//     if let Ok(_) = write_info("John") {
//         println!("Writing to file successful!");
//     }       
// }

// use std::fs::File;
// use std::io::{self, Write};

// fn write_info(name: &str) -> io::Result<()> {
//     let mut file = File::create("my_best_friends.txt")?;
//     file.write_all(format!("name: {}\n", name).as_bytes())?;
//     Ok(())
// }

// fn main() {
//     if let Ok(_) = write_info("John") {
//         println!("Writing to file successful!");
//     }
// }

// iterator [1]
// fn main() {
//     let names = vec!["John", "Alice", "Bob"];
//     for name in names.iter() {
//         println!("{}", name);
//     }
//     println!("{:?}", names);
// }

// iterator [2]
// fn main() {
//     let names = vec!["John", "Alice", "Bob"];
//     let names_iter = names.iter();
//     for name in names_iter {
//         println!("{}", name);
//     }
//     println!("{:?}", names);
// }

// iterator - useful(sum, max, min)
// fn main() {
//     let num = vec![1, 2, 3];

//     let sum: i32 = num.iter().sum();
//     let max = num.iter().max().unwrap();
//     let min = num.iter().min().unwrap();
//     println!("Sum: {}, Max: {}, Min: {}", sum, max, min);
// }

//method to create a new iterator
// fn main() {
//     let nums1 = vec![1, 2, 3];
//     let nums2 = vec![4, 5, 6];

//     let enumer: Vec<(usize, &i32)> = nums1.iter().enumerate().collect();
//     println!("{:?}", enumer);

//     let zip: Vec<(&i32, &i32)> = nums1.iter().zip(nums2.iter()).collect();
//     println!("{:?}", zip);
// }

// map, filter
// fn main() {
//     let nums: Vec<i32> = vec![1, 2, 3];

//     let f = |x: &i32| x + 1;

//     let maps: Vec<i32> = nums.iter().map(f).collect();
//     println!("{:?}", maps);

//     let filters: Vec<i32> = nums.into_iter().filter(|&x| x % 2 == 1).collect();
//     println!("{:?}", filters);
// }

// map, filter - vec clone
// fn main() {
//     let nums: Vec<i32> = vec![1, 2, 3];
//     let f = |x: &i32| x + 1;

//     let maps: Vec<i32> = nums.iter().map(f).collect();
//     println!("{:?}", maps);

//     let filters: Vec<i32> = nums.clone().into_iter().filter(|&x| x % 2 == 1).collect();
//     println!("{:?}", filters);

//     println!("{:?}", nums);
// }

// map, filter cloned - iterator clone
fn main() {
    let nums: Vec<i32> = vec![1, 2, 3];
    let f = |x: &i32| x + 1;

    let maps: Vec<i32> = nums.iter().map(f).collect();
    println!("{:?}", maps);

    let filters: Vec<i32> = nums.iter().filter(|&x| x % 2 == 1).cloned().collect();
    println!("{:?}", filters);

    println!("{:?}", nums);
}