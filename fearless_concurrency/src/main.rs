use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let v = vec![
            String::from("hello"),
            String::from("hi"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in v {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}