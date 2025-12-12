#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter (UsState)
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Luck penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

pub fn dispaly_coin_values() {
    let mut v = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("Value of quarter is {v} cents");
    v = value_in_cents(Coin::Penny);
    println!("Value of penny is {v} cent");
}

pub fn roll_dice() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other)
    }
}

fn add_fancy_hat() {
    println!("add fancy hat");
}

fn remove_fancy_hat() {
    println!("remove fancy hat")
}

fn move_player(num_space: u8) {
    println!("move player by {num_space}");
}