#![deny(clippy::all)]

fn get_user_name() -> Result<String, ()> {
    Ok("John".to_string())
}

fn get_err() -> Result<String, ()> {
    Err(())
}

fn get_first_name() -> Result<String, String> {
    Ok("John".to_string())
}

fn get_last_name() -> Result<String, String> {
    Err("No last name".to_string())
}

fn get_full_name() -> Result<String, String> {
    //question mark at the end of the function checks for error
    let first = get_first_name()?;
    let last = get_last_name()?;
    Ok(format!("{} {}", first, last))
}

fn main() {
    // 'value' is either a string slice or an error that sits in the heap
    let value: Result<&str, Box<dyn std::error::Error>> = Ok("Hello");
    match value {
        Ok(value) => println!("Value is {}", value),
        Err(error) => println!("Error is {}", error),
    }

    // 'value' is either a string slice or something else
    let value: Result<&str, ()> = Ok("Hello");
    match value {
        Ok(value) => println!("Value is {}", value),
        Err(_) => println!("Some error occured"),
    }

    let unwrapped = value.expect("I was expecting a username");
    let user_name = get_user_name().expect("Failed to get username");
    println!("Hello, {}", user_name);

    let user_name = get_err().expect_err("I didn't expect a username");

    //check OK and Err with if
    let is_ok = get_user_name().is_ok();
    let is_err = get_user_name().is_err();
    println!("{} {}", is_ok, is_err);

    //Early exit from Result errors
    let full = get_full_name(); //the error from get_last_name will propagate through to here
    match full {
        Ok(full) => println!("Full name is {}", full),
        Err(_) => println!("Error!"),
    }

    //Map Ok in Result
    let first = get_first_name();
    let length = first.map(|s| s.len()).unwrap_or_default();
    println!("{}", length);

    let full = get_full_name();
    let error_length = full.map_err(|e| e.len());
    println!("{:?}", error_length);
}
