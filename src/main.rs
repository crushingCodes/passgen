use anyhow::Result;
use clap::Parser;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use passgen::Error;

#[derive(Parser)]
#[command(name = "passgen")]
#[command(about = "CLI password manager, generate password to clipboard")]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long)]
    length: Option<usize>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut ctx = ClipboardContext::new().map_err(|_| Error::ClipboardReadError)?;
    let pass_len = cli.length;
    let password = passgen::generate_password(pass_len)?;
    ctx.set_contents(password.to_owned())
        .map_err(|_| Error::ClipboardWriteError)?;
    println!("Password copied to clipboard ✅");
    Ok(())
}
