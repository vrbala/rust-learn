#[derive(Debug)]
enum State {
    Alabama,
    California,
    Wisconsin,
    Washington,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn to_cent(c : &Coin) -> u8 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

fn is_quarter(c: &Coin) -> bool {
    if let Coin::Quarter(_) = c {
        true
    } else {
        false
    }
}

fn main() {
    let p = Coin::Penny;
    println!("In cents, one {:?} is {}.", p, to_cent(&p));

    let q = Coin::Quarter(State::Alabama);
    println!("In cents, one {:?} is {}.", q, to_cent(&q));
    println!("Is it quarter? {:?} is {}.", q, is_quarter(&q));
}
