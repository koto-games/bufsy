use anyhow::Result;

use crate::config::settings::Settings;
use std::fs;

pub fn load(path: &str) -> Settings {
    let file_path = format!("{}/config.toml", path);
    let content = fs::read_to_string(&file_path)
        .unwrap_or_else(|e| panic!("failed to read config file {}: {}", file_path, e));

    toml::from_str(&content)
        .unwrap_or_else(|e| panic!("failed to parse config file {}: {}", file_path, e))
}

pub fn save(config: &Settings, path: &str) -> Result<()> {
    let file_path = format!("{}/config.toml", path);
    let content = toml::to_string_pretty(config)
        .unwrap_or_else(|e| panic!("failed to serialize config file {}: {}", file_path, e));

    fs::write(&file_path, content)
        .unwrap_or_else(|e| panic!("failed to write config file {}: {}", file_path, e));

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use crate::config::settings::Server;

    use super::*;

    #[test]
    fn save_load() {
        let config_dir = "test_dir/save_load";

        fs::create_dir_all(config_dir).unwrap();
        let config = test_load_config();
        save(&config, config_dir).unwrap();
        let loaded_config = load(config_dir);
        assert_eq!(config, loaded_config);
    }

    pub fn test_load_config() -> Settings {
        Settings {
            server: Server {
                host: "localhost".to_owned(),
                port: 8086,
            },
            key: "149a44cb0b9a4a56450c1da0cf8f107db8778b7e26c7b95fc4b36b9392c3b67b".to_owned(),
            connections: Vec::new(),
            // connections: vec![Server {
            //     host: "localhost".to_owned(),
            //     port: 8086,
            // }],
        }
    }
    pub fn test_config_dir() -> String {
        "test_dir/bufsy".to_owned()
    }
}
