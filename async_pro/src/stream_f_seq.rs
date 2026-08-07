//The async recv method produces a sequence of items over time. This is an instance of a much more general pattern known as a stream
use trpl::StreamExt;
pub fn stream_iter() {
   trpl::block_on(async {
     let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = values.iter().map(|n| n * 2);
    let mut stream = trpl::stream_from_iter(iter);
    while let Some(value) = stream.next().await {
        println!("The value was: {value}");
    }
   })
}
//The Stream trait defines a low-level interface that effectively combines the Iterator and Future traits. StreamExt supplies a higher-level set of APIs on top of Stream, including the next method as well as other utility methods similar to those provided by the Iterator trait. Stream and StreamExt are not yet part of Rust’s standard library, but most ecosystem crates use similar definitions.