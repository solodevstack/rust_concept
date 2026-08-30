Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures. The implementor of a trait will specify the concrete type to be used instead of the placeholder type for the particular implementation. That way, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented.
One example of a trait with an associated type is the Iterator trait that the standard library provides. The associated type is named Item and stands in for the type of the values the type implementing the Iterator trait is iterating over. The definition of the Iterator trait is as shown in Listing 20-13.

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
This syntax seems comparable to that of generics. So, why not just define the Iterator trait with generics, as shown in Listing 20-14?

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
The difference is that when using generics, as in Listing 20-14, we must annotate the types in each implementation; because we can also implement Iterator<String> for Counter or any other type, we could have multiple implementations of Iterator for Counter.

Using Default Generic Parameters and Operator Overloading

A great example of a situation where this technique is useful is with operator overloading, in which you customize the behavior of an operator (such as +) in particular situations.

Rust doesn’t allow you to create your own operators or overload arbitrary operators. But you can overload the operations and corresponding traits listed in std::ops by implementing the traits associated with the operator. For example, in Listing 20-15, we overload the + operator to add two Point instances together. We do this by implementing the Add trait on a Point struct.

Using Supertraits
Sometimes you might write a trait definition that depends on another trait: For a type to implement the first trait, you want to require that type to also implement the second trait. You would do this so that your trait definition can make use of the associated items of the second trait. The trait your trait definition is relying on is called a supertrait of your trait.