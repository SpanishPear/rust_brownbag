fn main() {
    println!("Hello, world!");
    
    let option = get_admin_name();
    match option {
        Some(value) => println!("Option: {}", value),
        None => println!("Option: None"),
    }
}

// function taht demonstrates
// the option type
fn get_admin_name() -> Option<String> {
    let admin = "admin".to_string();
    Some(admin)
}
