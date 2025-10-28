
    use std::collections::HashMap;

   fn main(){
//Updating a Value Based on the Old Value


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
         
         println!("{count:?}: {word}");
        *count += 1;
       
    }

    println!("{map:?}");


//Adding a Key and Value Only If a Key Isn’t Present
    // use std::collections::HashMap;

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{scores:?}");



//For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values, as demonstrated in Listing 8-22.
    //  let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(&field_name, &field_value);
    // println!("{field_name}: {field_value}");

    

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    //We can iterate over each key-value pair in a hash map in a similar manner as we do with vectors, using a for loop:

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }
    //We can get a value out of the hash map by providing its key to the get method, as shown in Listing
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    // println!("Score for {}: {}", team_name, score);


    }
    