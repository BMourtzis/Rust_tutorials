use std::collections::HashMap;

fn main() {
    new_vector();
    new_string();
    new_hasmaps();
}

fn new_vector() {
    let mut v = vec![1, 2, 3];

    v.push(4);
    v.push(5);

    let third = &v[2];

    v.push(4);

    match v.get(2) {
        Some(third) => print!("The thrid element is {}", third),
        None => print!("There is no third element"),
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }
}

fn new_string() {
    let mut s = String::new();
    s = String::from("initial contents");
    s = String::from("Ελληνικά στρίνγκ");

    s.push_str("bar");

    let s2 = String::from("hello");

    let s3 = s + &s2;

    let s4 = format!("{}-{}", s3, s2);

    // NOTE: string are stored are u8 vectors
    // user .chars();

    for c in s4.chars() {
        println!("{}", c);
    }
}

fn new_hasmaps() {
     let mut scores = HashMap::new();

     scores.insert(String::from("Blue"), 150);
     scores.insert(String::from("Yellow"), 50);

     let teams = vec![String::from("Blue"), String::from("Yellow")];
     let initial_scores = vec![10, 50];

     let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

     let score = scores.get(&String::from("Yellow"));

     let new_score = scores.entry(String::from("Yellow")).or_insert(0);
     *new_score += 1;
}
