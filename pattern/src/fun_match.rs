fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}
//The x part is a pattern! As we did with let, we could match a tuple in a function’s arguments to the pattern. Listing 19-7 splits the values in a tuple as we pass it to a function.

pub fn fun_main() {
    let point = (3, 5);
    print_coordinates(&point);
}