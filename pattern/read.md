Patterns are a special syntax in Rust for matching against the structure of types, both complex and simple. Using patterns in conjunction with match expressions and other constructs gives you more control over a program’s control flow. A pattern consists of some combination of the following:

Literals
Destructured arrays, enums, structs, or tuples
Variables
Wildcards
Placeholders
For example, here’s the match expression from Listing 6-5 that matches on an Option<i32> value in the variable x:

match x {
    None => None,
    Some(i) => Some(i + 1),
}

The downside of using if let expressions is that the compiler doesn’t check for exhaustiveness, whereas with match expressions it does. If we omitted the last else block and therefore missed handling some cases, the compiler would not alert us to the possible logic bug.

while let Conditional Loops


Similar in construction to if let, the while let conditional loop allows a while loop to run for as long as a pattern continues to match. In Listing 19-4, we show a while let loop that waits on messages sent between threads, but in this case checking a Result instead of an Option.

Patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible value passed are irrefutable. An example would be x in the statement let x = 5; because x matches anything and therefore cannot fail to match. Patterns that can fail to match for some possible value are refutable. An example would be Some(x) in the expression if let Some(x) = a_value because if the value in the a_value variable is None rather than Some, the Some(x) pattern will not match.

If we have a refutable pattern where an irrefutable pattern is needed, we can fix it by changing the code that uses the pattern: Instead of using let, we can use let...else. Then, if the pattern doesn’t match, the code in the curly brackets will handle the value. Listing 19-9 shows how to fix the code in Listing 19-8.

    let Some(x) = some_option_value else {
        return;
    };

    To create a match expression that compares the values of the outer x and y, rather than introducing a new variable that shadows the existing y variable, we would need to use a match guard conditional instead. We’ll talk about match guards later in the

        let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
If x is 1, 2, 3, 4, or 5, the first arm will match. This syntax is more convenient for multiple match values than using the | operator to express the same idea; if we were to use |, we would have to specify 1 | 2 | 3 | 4 | 5. Specifying a range is much shorter, especially if we want to match, say, any number between 1 and 1,000!

The compiler checks that the range isn’t empty at compile time, and because the only types for which Rust can tell if a range is empty or not are char and numeric values, ranges are only allowed with numeric or char values.

Here is an example using ranges of char values:


    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    //Matching Multiple Patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    Structs and Tuples
We can mix, match, and nest destructuring patterns in even more complex ways. The following example shows a complicated destructure where we nest structs and tuples inside a tuple and destructure all the primitive values out:

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    
    We can also use underscores in multiple places within one pattern to ignore particular values. Listing 19-19 shows an example of ignoring the second and fourth values in a tuple of five items.

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
However, using .. must be unambiguous. If it is unclear which values are intended for matching and which should be ignored, Rust will give us an error. Listing 19-25 shows an example of using .. ambiguously, so it will not compile.

Filename: src/main.rs
This code does not compile!
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {second}")
        },
    }
}
Listing 19-25: An attempt to use .. in an ambiguous way
When we compile this example, we get this error:

$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error: `..` can only be used once per tuple pattern

Summary
Rust’s patterns are very useful in distinguishing between different kinds of data. When used in match expressions, Rust ensures that your patterns cover every possible value, or your program won’t compile. Patterns in let statements and function parameters make those constructs more useful, enabling the destructuring of values into smaller parts and assigning those parts to variables. We can create simple or complex patterns to suit our needs.