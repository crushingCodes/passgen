use rand::Rng;

mod error;
pub use error::Error;

pub fn generate_password() -> Result<String, anyhow::Error> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            ()*&^%$#@!~";

    let password_len: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..password_len)
        .map(|_| {
            let char_idx = rng.gen_range(0..CHARSET.len());
            CHARSET[char_idx] as char
        })
        .collect();
    Ok(password)
}
