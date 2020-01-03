fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let vv = vec![1, 2, 3];
    let second: &i32 = &v[2];
    let third: Option<&i32> = vv.get(3);

    let v3 = vec![100, 32, 57];
    for i in &v3 {
        println!("{}", i);
    }

    let mut v4 = vec![99, 33, 49];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v4);

    enum SpreadsheetCell {
        Int(i32);
        Float(f64);
        Text(String);
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ]
}
