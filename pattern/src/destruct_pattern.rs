//Destructuring to Break Apart Values

//We can also use patterns to destructure structs, enums, and tuples to use different parts of these values. Let’s walk through each value.

struct Point {
    x: i32,
    y: i32,
}

pub fn des_fn (){
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    
}


pub fn des_2main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
pub fn pattern_main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
    //Structs and Tuples
    
     let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
     println!("{feet}");
     
     println!("print struct {x}" )

}