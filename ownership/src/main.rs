fn main() {
    struct Color(i32, i32, i32);

    let black = Color(256, 256, 256);
    println!("{}", black.0);
}