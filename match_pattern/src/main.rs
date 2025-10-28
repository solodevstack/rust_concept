#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}



fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    // let cents = value_in_cents(coin);
    let cents =describe_state_quarter(coin)
        .unwrap_or_else(|| String::from("Not a quarter!"));
    println!("{cents}");

}

// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     let state = if let Coin::Quarter(state) = coin {
//         state
//     } else {
//         return None;
//     };

//     if state.existed_in(1900) {
//         Some(format!("{state:?} is pretty old, for America!"))
//     } else {
//         Some(format!("{state:?} is relatively new."))
//     }
// }


// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     if let Coin::Quarter(state) = coin {
//         if state.existed_in(1900) {
//             Some(format!("{state:?} is pretty old, for America!"))
//         } else {
//             Some(format!("{state:?} is relatively new."))
//         }
//     } else {
//         None
//     }
// }



// fn main(){
//     let dice_roll = 7;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other),
//     }

//     fn add_fancy_hat() {
//          println!("ADD");
//     }
//     fn remove_fancy_hat() {}
//     fn move_player(num_spaces: u8) {
//          println!("The value of the coin is: {num_spaces}");
//     }

   

// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         }
//     }
// }
