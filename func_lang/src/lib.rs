
//The code in Listing 13-14 doesn’t do anything; the closure we’ve specified never gets called. The warning reminds us why: Iterator adapters are lazy, and we need to consume the iterator here.
//You can chain multiple calls to iterator adapters to perform complex actions in a readable way. But because all iterators are lazy, you have to call one of the consuming adapter methods to get results from calls to iterator adapters.
pub fn prod_other_iter() {
    //    let v1: Vec<i32> = vec![1, 2, 3];

    //       v1.iter().map(|x| x + 1);
    //         println!("Other function called");
        let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
    println!("Other function called");
    println!("v2: {:?}", v2);
    println!("v1: {:?}", v1);

  
}