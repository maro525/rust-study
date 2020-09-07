fn main() {
    use std::mem;

    let color = String::from("green");

    let print = || println!("`color`: {}", color);
    
    print();
    print();

    // let _reborrow = color;
    // print();

    // let _color_moved = color;
    // print();

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();

    let _reborrow = &count;
    // inc();

    let _count_reborrowed = &mut count;
    let _count_reborrowed = count;
    
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // consume();

    // move
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    let i = 3;
    println!("{}", contains(&i));
    println!("{}", contains(4));
}
