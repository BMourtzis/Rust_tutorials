#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    //Associated Functions
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }

    //Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle{ width:30, height: 50 };

    let sq = Rectangle::square(50);

    println!( "the area of the rectangle is {}.", rect.area());
}


//User
fn build_user(email: String, username: String) -> User {
    User {
    email,
    username,
    sign_in_count: 0,
    active: true
}
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
