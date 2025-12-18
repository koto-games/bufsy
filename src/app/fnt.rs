use super::encryption::decrypt;
use super::print;
use crate::KEY;
use anyhow::Result;
use wl_clipboard_rs::copy::{MimeType, Options, Source};

pub fn fnthost(text: String, ip: String) -> Result<()> {
    let text: Vec<&str> = text.split("|").collect();
    if !(text.len() == 2 || text.len() == 3) || text[0].is_empty() || text[1].is_empty() {
        return Err(anyhow::anyhow!("Invalid input"));
    }

    // let nonce = hex::encode(generate_nonce());
    // println!(
    //     "{} {}",
    //     encrypt("crates.io/crates/hex", &*KEY, &nonce).unwrap(),
    //     nonce
    // );
    let text_true = decrypt(&KEY, &text[1], text[0]).unwrap();

    print(&text_true)?;
    // let opts = Options::new();
    // opts.copy(
    //     Source::Bytes(text_true.into_bytes().into()),
    //     MimeType::Autodetect,
    // )?;

    if text.len() == 3 {
        let addr = format!("{}:{}", ip, &text[2]);
        // print(&addr)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnthost_text_invalid_input() {
        assert!(fnthost("test|test|2|:1".to_string(), "117.0.0.1:91".to_string()).is_err());
        assert!(fnthost("cat :>|".to_string(), "123.0.0.1:8080".to_string()).is_err());
        assert!(fnthost("|".to_string(), "127.0.11.1:8080".to_string()).is_err());
        assert!(
            fnthost(
                "|cat :>||||||||||||||||||||||||||||||cat||||:)||".to_string(),
                "127.0.2.3:8080".to_string()
            )
            .is_err()
        );
    }
}
