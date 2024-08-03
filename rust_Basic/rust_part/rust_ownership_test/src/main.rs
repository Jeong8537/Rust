// ownership problem [1]
// fn take_ownership(s: String) -> String {
//     println!("{}", s);
//     s
// }
// fn main() {
//     let s1 = String::from("hello, world!");
//     let s2 = take_ownership(s1);

//     println!("{}", s2);
// }

// ownership problem [2]
// fn main() {
//     let x = String::from("hello, world!");
//     let y = &x;

//     println!("{} {}", x, y);
// }

// ownership problem [3]
// fn main() {
//     let mut s = String::from("hello, ");

//     borrow_object(&mut s);

//     println!("Success!");
// }
// fn borrow_object(s: &mut String) {
//     println!("{}", s);
// }

// ownership problem [4]
// fn main() {
//     let mut name = "A".to_string();
//     let new_name = name.clone();

//     let mut inc1 = move || {
//         name.push_str("B");
//         println!("{}", name);
//     };

//     inc1();

//     let mut name = new_name;
//     let mut inc2 = move || {
//         name.push_str("C");
//         println!("{}", name);
//     };

//     inc2();
// }

// ownership problem [5]
// fn factory() -> impl Fn(i32) -> i32 {
//     let num = 5;

//     move |x: i32| x + num
// }

// fn main() {
//     println!("{}", factory()(1));
// }
