use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Score of Blue is {:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 25);
    println!("{:?}", scores);
    // the original value of 10 has been overwritten

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let s: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point,

    // only inserting a value if the key has no velue
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // returns a mutable reference to the value for this key
        *count += 1;
    }
}
