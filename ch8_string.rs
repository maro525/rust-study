fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);

    let a1 = String::from("tic");
    let a2 = String::from("tac");
    let a3 = String::from("toe");
    let a = a1 + "-" + &a2 + "-" + &a3;
    println!("a is {}", a);

    let b1 = String::from("tic");
    let b2 = String::from("tac");
    let b3 = String::from("toe");
    let b = format!("{}-{}-{}", b1, b2, b3);
    println!("b is {}", b);

    // let h = String::from("Здравствуйте");
    // let h_len = h.len();
    // let h0 = &h[0];

    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s is {}", s);

    for c in hello.chars() {
        println!("{}", c);
    }

    for b in hello.bytes() {
        println!("{}", b);
    }
}
