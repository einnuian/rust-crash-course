#![deny(clippy::all)]

use std::fmt;

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

//display trait
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} is {} years old",
            self.first_name, self.last_name, self.age
        )
    }
}

trait HasFullName {
    fn full_name(&self) -> String;
}

//implement a trait on another trait
trait HasName {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
}

trait HasFullName2
where
    Self: HasName,
{
    fn full_name2(&self) -> String;
}

impl<T> HasFullName2 for T
where
    T: HasName,
{
    fn full_name2(&self) -> String {
        format!("{} {}", self.first_name(), self.last_name())
    }
}

//trait as a parameter
fn print_full_name_and_age(value: &impl HasFullName) {
    println!("{}", value.full_name());
}

//generic and multiple traits
fn print_details<T: HasFullName + CanRun>(value: &T) {
    println!("{}", value.full_name());
    value.run();
}

trait CanRun {
    fn run(&self);
}

impl CanRun for Person {
    fn run(&self) {
        //nothing
    }
}

impl HasName for Person {
    fn first_name(&self) -> &str {
        &self.first_name
    }
    fn last_name(&self) -> &str {
        &self.last_name
    }
}

//traits have to be implemented
impl HasFullName for Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

trait CanInitializeWithFullName {
    fn new(full_name: &str) -> Self;
}

impl CanInitializeWithFullName for Person {
    fn new(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split(' ').collect();
        Person {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            age: 0,
        }
    }
}

fn main() {
    let person = Person::new("John Doe");
    println!("{}", person);
    print_full_name_and_age(&person);
    let full_name = person.full_name2();
    println!("{}", full_name);
}
