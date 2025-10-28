use std::io;

enum CarFunc {
    Start,
    Clotch,
    Gear,
    Accelerate,
    Brake,
}

impl CarFunc {
    fn perform(&self) {
        match self {
            CarFunc::Start => println!("Car started."),
            CarFunc::Clotch => println!("Clutch engaged."),
            CarFunc::Gear => println!("Gear shifted."),
            CarFunc::Accelerate => println!("Accelerating..."),
            CarFunc::Brake => println!("Braking!"),
        }
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let expected_sequence = [
        ("Start", CarFunc::Start),
        ("Clotch", CarFunc::Clotch),
        ("Gear", CarFunc::Gear),
        ("Accelerate", CarFunc::Accelerate),
        ("Brake", CarFunc::Brake),
    ];

    for (expected_command, car_func) in &expected_sequence {
        loop {
            let action = read_input(&format!("Please enter '{}':", expected_command));

            if action.eq_ignore_ascii_case("Quit") {
                println!("Quitting the program.");
                return;
            }

            if action == *expected_command {
                car_func.perform();
                break;
            } else {
                println!("Invalid action. Please enter '{}'.", expected_command);
            }
        }
    }

    println!("Car sequence complete!");
}





// use std::io;
// enum CarFunc {
//     Start,
//     TurnOff,
//     Clotch,
//     Gear,
//     Accelerate,
//     Brake,
  
// }
// impl CarFunc {
//     fn perform(&self) {
//         match self {
//             CarFunc::Start => println!("Car is starting."),
//             CarFunc::Clotch => println!("Car is in clutch mode."),
//             CarFunc::Gear => println!("Car is in gear mode."),
//             CarFunc::TurnOff => println!("Car is OFF."),
//             CarFunc::Accelerate => println!("Car is accelerating."),
//             CarFunc::Brake => println!("Car is braking."),
//         }
//     }
// }

// fn main() {
//     // let car_start = CarFunc::Start;
//     // car_action.perform();

//  println!("Enter car action (Start, TurnOff, Accelerate, Brake):");

//     let mut car_action = String::new();
//    io::stdin()
//         .read_line(&mut car_action)
//         .expect("Failed to read line");

//     let car_action = car_action.trim();

//     if car_action == "Start" {
//         CarFunc::Start.perform();
//          let mut car_action = String::new();
//          io::stdin()
//         .read_line(&mut car_action)
//         .expect("Failed to read line");
//         let car_action = car_action.trim();

//         if car_action == "Clotch" {
//             CarFunc::Clotch.perform();
//             let mut car_action = String::new();
//          io::stdin()
//         .read_line(&mut car_action)
//         .expect("Failed to read line");
//         let car_action = car_action.trim();
        
//             if car_action == "Gear" {
//                 CarFunc::Gear.perform();
//                 let mut car_action = String::new();
//                 io::stdin()
//                 .read_line(&mut car_action)
//                 .expect("Failed to read line");
//                 let car_action = car_action.trim();

//                 if car_action == "Accelerate" {
//                     CarFunc::Accelerate.perform();
//                     let mut car_action = String::new();
//                     io::stdin()
//                     .read_line(&mut car_action)
//                     .expect("Failed to read line");
//                     let car_action = car_action.trim();

//                     if car_action == "Brake" {
//                         CarFunc::Brake.perform();
//                     } else {
//                         println!("Invalid action, please enter 'Brake' to stop the car.");
//                         return;
//                     }
//                 } else {
//                     println!("Invalid action, please enter 'Accelerate' to speed up.");
//                     return;
//                 }
//             } else {
//                 println!("Invalid action, please enter 'Gear' to shift gears.");
//                 return;
//             }
//         } else {
//             println!("Invalid action, please enter 'Clotch' to engage the clutch.");
//             return;

//         }

            
//         } else {
//         println!("Invalid action, please enter 'Start' to begin.");
//         return;
//         }
    


// }
   
    // else if car_action == "TurnOff" {
    //     CarFunc::TurnOff.perform();
    // } else if car_action == "Accelerate" {
    //     CarFunc::Accelerate.perform();
    // } else if car_action == "Brake" {
    //     CarFunc::Brake.perform();
    // } else {
    //     println!("Invalid action");
    // }

    // let action = match car_action {
    //     "Start" => CarFunc::Start,
    //     "TurnOff" => CarFunc::TurnOff,
    //     "Accelerate" => CarFunc::Accelerate,
    //     "Brake" => CarFunc::Brake,
    //     _ => {
    //         println!("Invalid action");
    //         return;
    //     }
    // };

    // action.perform();