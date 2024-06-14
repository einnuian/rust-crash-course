#![deny(clippy::all)]

use std::fmt::Arguments;

fn main() {
    let value = Some(10);
    let name: Option<&str> = None;
    match name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("No name"),
    }

    //unwrapping name unsafely
    //let unwrapped_name = name.expect("name was not provided");
    //println!("Name is {}", unwrapped_name);

    //mutating optional values
    let mut age: Option<i8> = Some(20);
    match age.as_mut() {
        Some(age) => *age += 10,
        None => {}
    };
    println!("Age is {}", age.unwrap());

    //unwrapping multiple optionals
    let age1: Option<i8> = Some(20);
    let age2: Option<i8> = Some(30);
    let age3: Option<i8> = Some(40);
    if let (Some(age_1), Some(age_2), Some(age_3)) = (age1, age2, age3) {
        println!("{}, {}, {}", age_1, age_2, age_3);
    }

    //unwrap with default value
    let name: Option<&str> = None;
    let unwrapped = name.unwrap_or("John Doe");
    println!("Name is {}", unwrapped);

    let age: Option<i32> = None;
    let age = age.unwrap_or_default();
    println!("Age is {}", age);

    //check value of optional
    if name.is_some() {
        println!("There is a value");
    } else {
        println!("There is no value");
    }

    //mapping an option
    let age: Option<i32> = Some(20);
    let age_times_two = age.map(|age| age * 2);
    println!("{}", age_times_two.unwrap_or_default());
}
