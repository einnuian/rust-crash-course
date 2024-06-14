#![deny(clippy::all)]

//this function will not compile
/*
fn get_full_name() -> &str {
    "John Doe"
}*/
//It is because &str (string slice) is a borrowed value, created in the heap and referenced to via the use of '&'.
//when the funtion returns, the memory for this value in the heap is deallocated

//this function will compile
//because String lasts for the entire duration of the program
fn get_full_name() -> String {
    "John Doe".to_string()
}

//Lifetime in struct
struct Person<'a> {
    name: &'a str, //this attribute lives as long as the struct lives
}

//Rust implicitly assigns lifetime to both the parameter and the return value
//when there is a single input ('full_name' in the function below)
fn get_first_name(full_name: &str) -> &str {
    full_name
}

//Generic lifetime specifiers
//a, b, and the return value has a lifetime of 'l'
fn get_random_name<'l>(a: &'l str, b: &'l str) -> &'l str {
    b
}

impl<'a> Person<'a> {
    fn first_char_of_name(&self) -> &str {
        //lifetime of return value is the same as 'self'
        &self.name[0..1]
    }

    fn get_full_name(&self) -> String {
        self.name.to_string()
    }
}

//Lifetimes in enum
enum Animal<'a> {
    Dog { name: &'a str },
}

fn main() {
    let name = get_random_name("John", "Doe");
    print!("{}", name);
}
