use std::io;
fn main() {
    let password = passgen::generate_password().unwrap();
    let mut name = String::new();
    println!("> Enter a name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");
    println!("{}", password);
}
