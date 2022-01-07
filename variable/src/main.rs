// fn main() {
//     let spaces = "    ";
//     let spaces = spaces.len();
// }


fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);
}