use crate::config::settings::Settings;
use std::fs;

pub fn load(path: &str) -> Settings {
    let file_path = format!("{}/config.toml", path);
    let content = fs::read_to_string(&file_path)
        .unwrap_or_else(|e| panic!("failed to read config file {}: {}", file_path, e));

    toml::from_str(&content)
        .unwrap_or_else(|e| panic!("failed to parse config file {}: {}", file_path, e))

    // Config::builder()
    //     .add_source(File::with_name(file_path).required(false))
    //     .add_source(Environment::with_prefix("APP").separator("__"))
    //     .build()
    //     .expect("config build failed")
    //     .try_deserialize::<Settings>()
    //     .expect("config deserialize failed")
}

#[cfg(test)]
pub mod tests {
    use crate::config::settings::Server;

    use super::*;

    pub fn test_load_config() -> Settings {
        Settings {
            server: Server {
                host: "localhost".to_owned(),
                port: 8086,
            },
            key: "149a44cb0b9a4a56450c1da0cf8f107db8778b7e26c7b95fc4b36b9392c3b67b".to_owned(),
        }
    }
}
