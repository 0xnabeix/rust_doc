fn main() {
    // let favorite_color: Option<&str> = None;
    // let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    println!("age is {:?}", age);
}