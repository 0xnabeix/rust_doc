use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox draw function");
    }
}

use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            },
            Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            },
        ],
    };

    screen.run();
}