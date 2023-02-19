
fn main() {
    let a = "test";
    println!("Hello, world!");
    
    let result = checked_division(10, 0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    let result = checked_division(10, 2).unwrap_or(0);

}

fn<T> thing() -> T {}

// function that demonstrates
// the Option type
fn checked_division(dividend: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        Err("Division by zero occurred!".to_string())
    } else {
        Ok(dividend / divisor)
    }
}

