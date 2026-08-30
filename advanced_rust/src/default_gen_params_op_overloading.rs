//Using Default Generic Parameters and Operator Overloading
//You specify a default type when declaring a generic type with the <PlaceholderType=ConcreteType> syntax.

//A great example of a situation where this technique is useful is with operator overloading, in which you customize the behavior of an operator (such as +) in particular situations.

//trait Add<Rhs=Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

//This code should look generally familiar: a trait with one method and an associated type. The new part is Rhs=Self: This syntax is called default type parameters. The Rhs generic type parameter (short for “right-hand side”) defines the type of the rhs parameter in the add method. If we don’t specify a concrete type for Rhs when we implement the Add trait, the type of Rhs will default to Self, which will be the type we’re implementing Add on.
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn ops() {
    let added_p = Point { x: 1, y: 0 } + Point { x: 2, y: 3 };
    println!("print added {:?}", added_p);
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}


//To add Millimeters and Meters, we specify impl Add<Meters> to set the value of the Rhs type parameter instead of using the default of Self.

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
pub fn nongeneric_rhs(){
    let added = Millimeters(10) + Meters(1);
    print!("add {:?}", added);
}