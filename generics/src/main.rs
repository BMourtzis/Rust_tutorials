fn main() {
    get_largest(&vec![1,2,3,4]);
}

fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

//for a specific concrete class
impl Point<i32> {
    fn distanc_from_origin(&self) -> f32 {
        3.0
        // (self.x.pow(2) + self.y.pow(2)).sqrt()
    }
}

enum Option<T>{
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}