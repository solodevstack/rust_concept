pub fn ignore_value() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");


    //We’ll receive an error because the s value will still be moved into _s, which prevents us from using s again. However, using the underscore by itself doesn’t ever bind to the value.
        let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");

}

struct Point {
        x: i32,
        y: i32,
        z: i32,
}
//We list the x value and then just include the .. pattern. This is quicker than having to list y: _ and z: _, particularly when we’re working with structs that have lots of fields in situations where only one or two fields are relevant.

pub fn ignore_remaining() {

 

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"),
        
    }
     let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last,p) => {
            println!("Some numbers: {first}, {last},{p}");
        }
    }


}