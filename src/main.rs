use cli_clipboard::{ClipboardContext, ClipboardProvider};
// use std::io;
use anyhow::Result;
use passgen::Error;

fn main() -> Result<()> {
    let mut ctx = ClipboardContext::new().map_err(|_| Error::ClipboardReadError)?;
    let password = passgen::generate_password()?;
    ctx.set_contents(password.to_owned())
        .map_err(|_| Error::ClipboardWriteError)?;
    println!("Password copied to clipboard!");

    //TODO: add the following to sqlite db
    // let mut name = String::new();
    // println!("> Enter a name:");
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to read name");
    // println!("{}", password);
    Ok(())
}
