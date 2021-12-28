// fn main() {
//     let mut stack = Vec::new();

//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     while let Some(top) = stack.pop() {
//         println!("{}", top);
//     }
// }

// fn main() {
//     let v = vec!['a', 'b', 'c'];

//     for item in v.iter() {
//         println!("item is {}", item);
//     }
// }



// fn main() {
//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(y) => println!("Matched y = {:?}", y),
//         _ => println!("Default case, x = {:?}", x),
//     }

//     println!("at the end: x= {:?}, y = {:?}", x, y);
// }


// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let p = Point {x: 0, y: 7};

//     let Point{ x: a, y: b } = p;
//     assert_eq!(0, a);
//     assert_eq!(7, b);
// }


// fn main() {
//     let p = Point { x: 1, y: 7 };

//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         Point { x: 0, y } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neithger axis: ({}, {})", x, y),
//     }
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg = Message::ChangeColor(0, 160, 255);

//     let msg = Message::ChangeColor(0, 160, 255);

//     match msg {
//         Message::ChangeColor(r, g, b) => {
//             println!("Change the color to red {}, green {} and blue {}", r, g, b);
//         },
//         _ => {
//             println!("Welcome");
//         }
//     }
// }

// enum Color {
//     Rgb(i32, i32, i32),
//     Hsv(i32, i32, i32),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(Color),
// }

// fn main() {
//     let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

//     match msg {
//         Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
//             "Change the color to red {}, green {}, blue {}",
//             r, g, b
//         ),
//         Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
//             "Change the color to hue {}, saturation {}, and value {}",
//             h, s, v
//         ),
//         _ => (),
//     }
// }


// fn foo(_: i32, y: i32) {
//     println!("This code only uses the y parameter: {}", y);
// }

// fn main() {
//     foo(3, 5);
// }


// fn main() {
//     let mut setting_value = Some(5);
//     let new_setting_value = Some(10);

//     match(setting_value, new_setting_value) {
//         (Some(_), Some(_)) => {
//             println!("Can't overwrite an existing customized value");
//         }
//         _ => {
//             setting_value = new_setting_value;
//         }
//     }

//     println!("Setting is {:?}", setting_value);`
// }


fn main() {
    let numbers = (2, 4, 5, 6, 1);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}