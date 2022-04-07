fn main() {
    println!("Print your password");
    let password = rpassword::read_password().unwrap();
    println!("your password: {}", password);

    // The same thing
    let password = rpassword::prompt_password("Print your password").unwrap();
    println!("your password: {}", password);
}
