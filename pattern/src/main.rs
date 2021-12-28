// fn main() {
//     let mut stack = Vec::new();

//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     while let Some(top) = stack.pop() {
//         println!("{}", top);
//     }
// }

fn main() {
    let v = vec!['a', 'b', 'c'];

    for item in v.iter() {
        println!("item is {}", item);
    }
}