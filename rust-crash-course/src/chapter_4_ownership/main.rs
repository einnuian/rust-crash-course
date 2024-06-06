#![deny(clippy::all)]

fn greet(name: &String) {
    println!("Hello, {}", name);
}

//mutable reference
fn empty_string(value: &mut String) {
    value.clear();
}

//dangling reference
fn get_name() -> &String {
    &"John".to_string()
}

fn main() {
    let name1: String = String::from("John");
    //let name2 = name1;
    //name2 take over the stack and heap of name 1
    //name1 practically no longer exist
    println!("Hello, {}", name1);
    //println!("Hello, {}", name2);

    //This works with integers, however
    let age1 = 10;
    let age2 = age1;
    println!("You are {} years old", age1);
    println!("You are {} years old", age2);

    //This is because a String object is stored in both the stack and the heap
    //the stack hols [ptr, len, capacity]. The heap holds the data that the ptr points to

    //going back to the String example above, the pointer of name2 points to
    //the same location in the heap as the pointer of name1.

    //when the "main" function is done, the location in the heap for name1 is deallocated
    //subsequently, the location in the heap for name2 is deallocated as well
    //Since name1 and name2 both points to the same location, this results in
    //a double deallocation, failing the compiler

    //REFERENCES (read-only)
    let name2 = &name1; //name2 is a pointer that points to the pointer of name1
    println!("Hello, {}", name2);

    greet(&name1);
    greet(name2);

    let mut name = String::from("John");
    empty_string(&mut name);
}
