enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
    }
}

let m = Message::Write(String::from("Hello"));
m.call();

let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;

fn main() {
    println!("Hello, world!");
}
