fn main() {
    let x = five();

    let y = {
        let x = 3;
        x + 1
    };

    let z = plus_one(44);

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    iterate_num();
}

fn five() -> i32 {
    25
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn iterate_num() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
