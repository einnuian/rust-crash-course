//add clippy
#![deny(clippy::all)]

const MY_AGE: u8 = 22;

fn main() {
    println!("Hello, world!");
    //let name = "Foo";
    let name = "John";
    println!("Hello, {}!", name);
    //name is an immutable variable so the following line is invalid
    //name = "Doe";

    //let mut allow the variable to be mutable
    let mut first_name = "John";
    first_name = "Doe";

    //types of integer (example: unsigned 8-bit integer)
    let age: u8 = 20u8;
    let population = 62_000_000;
    let red = 0xFA;
    let rgb = 0xFF0000;

    //floating point
    let distance = 5.5;
    let length = 2.2f32;
    let distance2 = 6.2;
    let total = distance + distance2;

    //variable shadowing
    let data = "Foo";
    {
        let data = data.to_string();
    }
    let data = 10;

    //working with constant
    println!("My age is {}", MY_AGE);

    //assigning the constant value to a variable
    let my_age = MY_AGE;

    //tuple
    let personal_data = (22, "John");
    //unpacking data inside tuple
    let (age, name) = personal_data;
    //unpacking tuple using indexing
    let age = personal_data.0;
    let name = personal_data.1;
}
