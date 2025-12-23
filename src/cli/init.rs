use crate::config::Settings;
use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::Result;

pub async fn init(config_dir: &str) -> Result<()> {
    fs::create_dir_all(&config_dir)?;

    let mut file = File::create(format!("{}/config.toml", config_dir))
        .expect("Failed to create or open the file");
    let settings = &Settings::default();
    file.write_all(toml::to_string(settings).unwrap().as_bytes())?;
    println!("✓ Bufsy initialized");
    println!(
        "✓ Configuration saved to {}",
        format!("{}/config.toml", config_dir)
    );
    println!("✓ Encryption key generated");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_dir() {
        // assert!(!fs::exists("test_dir/bufsy").expect("error test_dir/bufsy exists"));
        init(&"test_dir/bufsy".to_string()).await.unwrap();
        assert!(fs::exists("test_dir/bufsy/config.toml").expect("error test_dir/bufsy/key exists"));
        // fs::remove_dir_all("test_dir/bufsy").expect("error removing test_dir/bufsy");
    }
}
