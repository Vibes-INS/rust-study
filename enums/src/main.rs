#[derive(Debug)] // 这样可以可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn coin_part() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

fn some_part() {
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three")
    }
}

fn main() {
    coin_part();
    some_part();
}
