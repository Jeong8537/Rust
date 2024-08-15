// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let sum_of_squares = v
//         .into_iter()
//         .map(|x| x * x)
//         .filter(|&x| x % 2 == 0)
//         .fold(0. |sum, x| sum + x);

//     println!("Sum of squares: {}", sum_of_squares);
// }

//Struct
// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: i32,
// }

// fn main() {
//     let mut jane = Person {
//         name: String::from("Jane"),
//         age: 30
//     };
//     jane.age += 1;
//     println!("{} {}", jane.name, jane.age);
//     println!("{:?}", jane);
// }

//associate function
// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: i32,
//     alive: bool,
// }

// impl Person {
//     fn new(name: String, age: i32) -> Self {
//         Person {
//             name,
//             age,
//             alive: true,
//         }
//     }
// }

// fn main() {
//     let jane = Person::new(String::from("Jane"), 30);
//     println!("{:?}", jane);
// }

//Method
// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: i32,
//     alive: bool,
// }

// impl Person {
//     fn new(name: &str, age: i32) -> Person {
//         Person {
//             name: String::from(name),
//             age,
//             alive: true,
//         }
//     }

//     fn info(&self) {
//         println!("{} {}", self.name, self.age);
//     }

//     fn get_older(&mut self, year: i32){
//         self.age += year;
//     }
// }

// fn main() {
//     let mut john = Person::new("John", 20);
//     john.info();
//     john.get_older(5);
//     john.info();
// }

//tuple struct
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main(){
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);

//     println!("{} {}", black.0, origin.0);
// }

// Trait [1]
// trait Greet {
//     fn say_hello(&self){}
// }

// struct Person {
//     name: String,
//     age: i32,
//     alive: bool,
// }

// impl Person {
//     fn new(name: &str, age: i32) -> Person {
//         Person {
//             name: String::from(name),
//             age,
//             alive: true,
//         }
//     }
//     fn get_older(&mut self, year: i32){
//         self.age += year;
//     }
// }

// impl Greet for Person{}

// struct Student {
//     name: String,
//     age: i32,
//     alive: bool,
//     major: String,
// }

// impl Student {
//     fn new(name: &str, age: i32, major: &str) -> Student {
//         Student {
//             name: String::from(name),
//             age,
//             alive: true,
//             major: String::from(major),
//         }
//     }
// }

// impl Greet for Student{
//     fn say_hello(&self){
//         println!("Hello, my name is {} and I'm studying {}", self.name, self.major);
//     }
// }

// fn main() {
//     let mut person = Person::new("John", 20);
//     person.say_hello();
//     person.get_older(1);
//     println!("{} is now {} years old", person.name, person.age);

//     let student = Student::new("Alice", 22, "Computer Science");
//     student.say_hello();
// }

// Trait [2]
// trait Greet {
//     fn say_hello(){
//         println!("Hello, Rustacean!");
//     }
// }

// struct Person {
//     name: String,
//     age: i32,
//     alive: bool,
// }

// impl Greet for Person{}

// struct Student {
//     name: String,
//     age: i32,
//     alive: bool,
//     major: String,
// }

// impl Greet for Student{}

// fn main() {
//     let person = Person {
//         name: String::from("John"),
//         age: 20,
//         alive: true,
//     };
//     Person::say_hello();

//     let student = Student {
//         name: String::from("Alice"),
//         age: 22,
//         alive: true,
//         major: String::from("Computer Science"),
//     };
//     Student::say_hello();
// }

// Derived trait - copy trait
// fn i32_copy(val: i32) {
//     println!("i32: {}", val);
// }

// fn main() {
//     let my_i32 = 3;
//     i32_copy(my_i32);
//     println!("{}", my_i32);
// }

// Derived trait - clone trait
// #[derive(Clone, Copy)]
// struct Point {
//     val: i32,
// }

// fn main() {
//     let mut p1 = Point { val: 3 };
//     let p2 = p1;
//     p1.val = 3;

//     println!("p1.val: {}, p2.val: {}", p1.val, p2.val);
// }

// Derived trait - Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { 
        width: 30, 
        height: 50, 
    };
    println!("rect1 is {:?}", rect1);
}