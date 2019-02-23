fn main() {
    new_vector();
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
    
}
