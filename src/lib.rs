use rand::{Rng, rng};

mod error;
pub use error::Error;

pub fn generate_password(password_len: Option<usize>) -> Result<String, anyhow::Error> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            ()*&^%$#@!~";

    let password_len = password_len.unwrap_or(30);
    let mut rng = rng();

    let password: String = (0..password_len)
        .map(|_| {
            let char_idx = rng.random_range(0..CHARSET.len());
            CHARSET[char_idx] as char
        })
        .collect();
    Ok(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password_produces_result() {
        let result = generate_password(None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_password_has_default_length() {
        let result = generate_password(None).unwrap();
        assert_eq!(result.chars().count(), 30);
    }

    #[test]
    fn test_generate_password_has_respects_length() {
        let result = generate_password(Some(5)).unwrap();
        assert_eq!(result.chars().count(), 5);
    }
}
