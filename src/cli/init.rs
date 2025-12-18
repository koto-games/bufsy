use crate::{CONFIG_DIR, app::encryption::generate_key};
use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::Result;

pub async fn init(config_dir: Option<String>) -> Result<()> {
    let config_dir = if config_dir.is_some() {
        format!("{}/bufsy/", config_dir.unwrap())
    } else {
        CONFIG_DIR.clone()
    };
    fs::create_dir_all(&config_dir)?;

    let mut file =
        File::create(format!("{}/key", config_dir)).expect("Failed to create or open the file");
    // Write to the file
    file.write_all(hex::encode(generate_key()).as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_dir() {
        assert!(!fs::exists("test_dir/bufsy").expect("error test_dir/bufsy exists"));
        init(Some("test_dir".to_string())).await.unwrap();
        assert!(fs::exists("test_dir/bufsy/key").expect("error test_dir/bufsy/key exists"));
        fs::remove_dir_all("test_dir/bufsy").expect("error removing test_dir/bufsy");
    }
}
