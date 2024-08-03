fn main() {
    // for 문 [1]
    // for i in 6..10 {
    //     print!("{},", i);
    // }

    // [2]
    // let num_range = 6..=10;
    // for i in num_range {
    //     print!("{},", i);
    // }

    // while 문
    // let mut x = 0;
    // while x < 5 {
    //     print!("{},", x);
    //     x += 1;
    // }

    // loop 문 [1]
    // let mut x = 0;
    // loop {
    //     x += 1;
    //     if x == 5 {
    //         break;
    //     }
    //     print!("{},", x);
    // }

    // [2]
    // let mut x = 0;
    // let y = loop {
    //     x += 1;
    //     if x == 5 {
    //         break x;
    //     }
    //     print!("{},", x);
    // };
    // println!("{}", y);

    // continue, break
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        } else if i == 7 {
            break;
        }

        print!("{},", i);
    }
}
