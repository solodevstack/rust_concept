pub fn at_binding() {
    //This example will print Found an id in range: 5. By specifying id @ before the range 3..=7, we’re capturing whatever value matched the range in a variable named id while also testing that the value matched the range pattern.
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 10 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        Message::Hello { id: id @ 10..=12 } => {
             println!("Found an id in range: {id}")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}