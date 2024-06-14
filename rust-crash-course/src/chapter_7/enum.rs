#![deny(clippy::all)]

use core::f64;

#[derive(PartialEq)]

/*
Enumeration */
enum AnimalType {
    Dog,
    Cat,
    Rabbit,
}

enum Shapes {
    Circle { radius: f64, center: (f64, f64) },
    Rectangle { width: f32, height: f32 },
}

struct Size {
    width: f32,
    height: f32,
}

enum Shapes2 {
    Rectangle(f32, f32, Size),
    Circle(f32, f32, f32),
}

impl Shapes2 {
    fn area(&self) -> f32 {
        match self {
            Shapes2::Rectangle(x, y, size) => size.width * size.height,
            Shapes2::Circle(x, y, radius) => 3.14 * radius * radius,
        }
    }
}

enum Pet {
    Cat { name: String },
    Dog { name: String },
}

fn main() {
    //instance of enum
    let fluffy = AnimalType::Dog;
    match fluffy {
        AnimalType::Dog => println!("Woof!"),
        _ => println!("Not a dog :\\"), //default case
    }

    let rectangle = Shapes::Rectangle {
        width: 3.0,
        height: 4.0,
    };

    //comparing enums with associated values
    if let Shapes::Rectangle { width, height } = rectangle {
        println!("width = {}, height = {}", width, height);
    }

    //matching enums with associated values
    match rectangle {
        Shapes::Rectangle { width, height } => {
            println!("width = {}, height = {}", width, height);
        }
        _ => println!("Not a rectangle"),
    }

    let rectangle = Shapes2::Rectangle(
        1.0,
        2.0,
        Size {
            width: 3.0,
            height: 4.0,
        },
    );
    if let Shapes2::Rectangle(x, y, Size { width, height }) = rectangle {
        println!("{}, {}, {}, {}", x, y, width, height);
    }
    match rectangle {
        Shapes2::Rectangle(x, y, Size { width, height }) => {
            println!("{}, {}, {}, {}", x, y, width, height);
        }
        _ => println!("Not a rectangle"),
    }
    println!("Area is {}", rectangle.area());

    let fluffy = Pet::Cat {
        name: "Fluffy".to_string(),
    };
    let name = match fluffy {
        Pet::Cat { name } => name,
        Pet::Dog { name } => name,
    };
    println!("Hello, {}", name);
}
