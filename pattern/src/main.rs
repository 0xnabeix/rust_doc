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




struct Point {
    x: i32,
    y: i32,
}

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

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {} and blue {}", r, g, b);
        },
        _ => {
            println!("Welcome");
        }
    }
}