use super::encryption::decrypt;
use super::print;
use crate::config::{Settings, load, save, settings::Server};
use anyhow::Result;
use wl_clipboard_rs::copy::{MimeType, Options, Source};

pub fn fnthost(text: &str, ip: &str, config: &Settings, config_dir: &str) -> Result<()> {
    let text: Vec<&str> = text.split("|").collect();
    if !(text.len() == 2 || text.len() == 3) || text[0].is_empty() || text[1].is_empty() {
        return Err(anyhow::anyhow!("Invalid input"));
    }

    let text_true = decrypt(&config.key, &text[1], text[0]).unwrap();

    print(&text_true)?;
    let opts = Options::new();
    opts.copy(
        Source::Bytes(text_true.into_bytes().into()),
        MimeType::Autodetect,
    )?;

    if text.len() == 3
        && !load(&config_dir).connections.contains(&Server {
            host: ip.to_string(),
            port: text[2].parse::<u16>()?,
        })
    {
        let mut config_mut = load(&config_dir);
        config_mut.new_connection(ip, text[2].parse::<u16>()?);
        save(&config_mut, &config_dir)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_config::tests::{test_config_dir, test_load_config};

    #[test]
    fn fnthost_text_invalid_input() {
        let config = test_load_config();
        assert!(
            fnthost(
                "test|test|2|:1",
                "117.0.0.1:91",
                &config,
                &test_config_dir()
            )
            .is_err()
        );
        assert!(fnthost("cat :>|", "123.0.0.1:8080", &config, &test_config_dir()).is_err());
        assert!(fnthost("|", "127.0.11.1:8080", &config, &test_config_dir()).is_err());
        assert!(
            fnthost(
                "|cat :>||||||||||||||||||||||||||||||cat||||:)||",
                "127.0.2.3:8080",
                &config,
                &test_config_dir()
            )
            .is_err()
        );
    }
}
