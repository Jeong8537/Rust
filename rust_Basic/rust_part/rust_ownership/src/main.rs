// ownership 돌려주기

// fn dummy(x: String) -> String {
//     println!("{}", x);
//     x
// }
// fn main() {
//     let x = String::from("Hello");
//     let x = dummy(x);
//     println!("{}", x);
// }

// ownership_borrow, reference [1], [2]

// fn main() {
//     let x = String::from("Hello");
//     let y = &x;

//     println!("{} {}", x, y);
// }

// fn dummy(y: &String) {
//     println!("{}", y);
// }

// fn main() {
//     let x = String::from("Hello");
//     dummy(&x);
//     println!("{}", x);
// }

// variable reference
// fn dummy(y: &mut String) {
//     y.push_str(", world!");
//     println!("{}", y);
// }

// fn main() {
//     let mut x = String::from("Hello");
//     dummy(&mut x);
//     println!("{}", x);
// }

// variable reference error
// fn main() {
//     let mut x = String::from("Hello");
//     // let y = &mut x;
//     // let z = &mut y;
//     let y = &x;
//     let z = &x;

//     println!("{} {}", y, z);
// }

// closure environment capture
// fn main() {
//     let multiplier = 5;

//     let func = |x: i32| -> i32 { x * multiplier };

//     for i in 1..=5 {
//         println!("{}", func(i));
//     }

//     println!("{}", multiplier);
// }

// fn main() {
//     let mut multiplier = 5;

//     let mut func = |x: i32| -> i32 {
//         multiplier += 1;
//         x * multiplier
//     };

//     for i in 1..=5 {
//         println!("{}", func(i));
//     }
//     println!("{}", multiplier);
// }

// closure move
fn factory(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

fn main() {
    let multiplier = 5;
    let mult = factory(multiplier);
    for i in 1..=3 {
        println!("{}", mult(i));
    }
}
