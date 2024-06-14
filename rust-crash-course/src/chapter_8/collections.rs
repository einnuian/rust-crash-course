#![deny(clippy::all)]

use std::collections::HashMap;

use std::vec;

struct Point(f32, f32);

struct Person {
    name: String,
    age: u8,
}

fn get_values() -> (String, String, i32) {
    ("Hello".to_string(), "World".to_string(), 30)
}

fn main() {
    let values = ("Hello", "World", 30);
    //unpacking tuples
    let hello = values.0;
    let world = values.1;
    let (hello, world, age) = values;

    //vectors
    let values: [&str; 2] = ["foo", "bar"];
    for value in values.iter() {
        println!("{}", value);
    }
    //reference to a value inside a vector
    let foo = &values[0];
    let length = values.len();

    //map one collection to another
    let values2: [i32; 2] = [10, 20];
    let double = values2.iter().map(|x| x * 2);

    //shorthand for vectors
    //option to create mutable vectors
    let mut values3 = vec![1, 2, 3];
    values3.push(4);
    let four = values3.pop();
    values3.extend_from_slice(&[4, 5, 6]);
    println!("Values are {:?}", values3);
    values3.clear();
    println!("Values are {:?}", values3);

    //moving elements from one vector to another
    let mut values3 = vec![1, 2, 3];
    let mut values4 = vec![4, 5, 6];
    values3.append(&mut values4);
    println!("Values are {:?}", values3);
    println!("Values are {:?}", values4); //values 4 is cleared

    if values3.contains(&3) {
        println!("yes");
    } else {
        println!("no");
    }

    /*
    HashMap
     */
    let mut values: HashMap<&str, &str> = HashMap::new();
    values.insert("foo", "bar");
    println!("{:?}", values);

    if values.contains_key("name") {
        println!("name exists");
    } else {
        println!("No name");
    }

    values.remove("foo");
    println!("{:?}", values);

    match values.get("foo") {
        Some(value) => println!("{}", value),
        None => println!("Not found"),
    }

    //Iterating ove keys and values
    for (&k, &v) in &values {
        println!("{} {}", k, v);
    }

    //Entry
    let entry = values.entry("foo");

    values.insert("husband", "John Doe");
    values.entry("wife").or_insert("Jane Doe");
    println!("{:?}", values);

    //Using struct in hashmap
    let mut values: HashMap<Person, &str> = HashMap::new();

    //Iterators
    let names = vec!["John", "Jane", "Mary", "Bob"];
    for name in names.iter() {
        println!("{}", name); //here 'name' is a pointer to a string slice
    }
    //owned iteration
    for name in names.into_iter() {
        println!("{}", name); //here the values of the pointers is loaded into 'name'
    }

    let names2 = vec!["Chad", "Sara", "Axolotl"];
    //filter
    for name in names2.into_iter().filter(|name| name.len() > 4) {
        println!("{}", name);
    }
}
