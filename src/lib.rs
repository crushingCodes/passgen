use rand::{Rng, thread_rng};
use rand::{Rng, rng};

mod error;
pub use error::Error;

pub fn generate_password(password_len: Option<usize>) -> Result<String, anyhow::Error> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            ()*&^%$#@!~";

    let password_len = password_len.unwrap_or(30);
    let mut rng = thread_rng();
    let mut rng = rng();

    let password: String = (0..password_len)
        .map(|_| {
            let char_idx = rng.random_range(0..CHARSET.len());
            CHARSET[char_idx] as char
        })
        .collect();
    Ok(password)
}
