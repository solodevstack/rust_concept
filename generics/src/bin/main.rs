
//eliminate_duplicates
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result1 = largest(&number_list);
    println!("The largest number is {result1}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result2 = largest(&number_list);
    let sum = *result1 + *result2;
    println!("The largest number is {result2}");
    
    println!("The sum number is {sum}");
}
//eliminate_duplicates

fn duplicate() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}