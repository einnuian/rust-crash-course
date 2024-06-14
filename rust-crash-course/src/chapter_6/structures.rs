#![deny(clippy::all)]

struct Person {
    name: String,
    age: u8,
}

fn _create_person(name: String, age: u8) {
    let _person = Person {
        //no need to specify the field if the name of the args is the same as the fields
        name,
        age,
    };
}

/*
 * Tuple */

//debug trait
#[derive(Debug)]
struct Point(f64, f64, f64);

//instance methods
impl Point {
    fn describe(&self) {
        println!("Point is at ({}, {}, {})", self.0, self.1, self.2)
    }

    //non-method associated functions
    fn zero() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}

//a second block of implementation
impl Point {
    fn twice(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }

    //mutate the current instance instead of creating a new instance
    fn make_twice(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }
}

fn main() {
    let person = Person {
        name: "John".to_string(),
        age: 30,
    };
    println!("{} is {} years old", person.name, person.age);

    //structure update syntax
    let _person2 = Person {
        name: "Doe".to_string(),
        ..person //bring all the properties of 'person' into 'person2'
    };

    //tuple instance
    let point = Point(0.0, 1.0, 2.0);
    println!("x is {}, y is {}, z is {}", point.0, point.1, point.2);

    //instance methods
    let p = Point(1.0, 2.0, 3.0);
    p.describe();

    //debug
    println!("{:?}", p);

    //twice() & make_twice()
    let mut twice = point.twice();
    println!("Twice is at ({}, {}, {})", twice.0, twice.1, twice.2);
    twice.make_twice();
    println!("Twice is at ({}, {}, {})", twice.0, twice.1, twice.2);

    let zero = Point::zero();
    println!("Zero is at ({}, {}, {})", zero.0, zero.1, zero.2);
}
