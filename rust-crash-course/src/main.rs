#![deny(clippy::all)]

fn say_hello_world() -> String {
    String::from("Hello, world!")
}

fn print_message(message: String) {
    println!("{}", message);
}

fn say_hello(to_person: String) -> String {
    format!("Hello, {}!", to_person)
}

//function as an arg
fn _process_name(name: &str, callback: fn(&str) -> ()) {
    callback(name);
}

fn main() {
    let message = say_hello_world();
    println!("{}", message);
    print_message(message);

    let hello = say_hello(String::from("John"));
    println!("{}", hello);

    /*
     * inline functions
     */
    let say_hello_to = |name: &str| format!("Hello, {}!", name);
    println!("{}", say_hello_to("John"));

    //multiple args
    let full_name = |first_name: &str, last_name: &str| format!("{} {}", first_name, last_name);
    println!("Hello, {}!", full_name("John", "Smith"));

    //function pointer
    let multiply_by_2 = |x: i32| x * 2;
    let ptr = multiply_by_2;
    let result = ptr(10);
    println!("{}", result);
}
