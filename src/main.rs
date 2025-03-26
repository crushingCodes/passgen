use cli_clipboard::{ClipboardContext, ClipboardProvider};
// use std::io;

fn main() {
    let mut ctx = ClipboardContext::new().unwrap();
    let password = passgen::generate_password().unwrap();
    ctx.set_contents(password.to_owned()).unwrap();
    println!("Password copied to clipboard!");

    //TODO:
    // let mut name = String::new();
    // println!("> Enter a name:");
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to read name");
    // println!("{}", password);
}
