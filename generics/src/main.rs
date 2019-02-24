fn main() {
    let mut vec = vec![1,2,3,4];
    let something = get_largest(&vec);

    vec.push(5);
}

fn get_largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn get_largest<T: PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > &largest {
            largest = item;
        }
    }

    &largest
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