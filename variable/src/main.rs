// fn main() {
//     let spaces = "    ";
//     let spaces = spaces.len();
// }


// fn main() {
//     let guess: u32 = "42".parse().expect("Not a number");
//     println!("{}", guess);
// }


fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}