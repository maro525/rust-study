enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn printu8value(x: u8) {
    match x {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("nothing"),
    }
}

fn main() {
    let p = Coin::Dime;
    value_in_cents(p);

    let five = Some(5);
    let six =  plus_one(five);
    let none = plus_one(None);

    // let some_u8_value = 0u8;
    // printu8value(some_u8_value);

    if let Some(3) = some_u8_value {
        println!("three");
    }
}
