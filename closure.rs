fn main() {
    fn function (j: i32) -> i32 { j + 1 }

    let closure_annotated = |k: i32| -> i32 { k + 1 };
    let closure_inferred = |l| l + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());
}
