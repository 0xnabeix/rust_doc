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

fn main() {
    let p = Point {x: 0, y: 7};

    let Point{ x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}