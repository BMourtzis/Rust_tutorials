fn main() {
    println!("Hello, world!");
}

enum IpAddrKind {
    IPv4(u8, u8, u8, u8),
    IPv6(String)
}

enum Message {
    Quit,
    Move(x: i32, y: i32),
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {

    }
}

enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        None => None,
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);