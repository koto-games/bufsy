use anyhow::Result;
use notify_rust::Notification;

pub fn print(text: &str) -> Result<()> {
    Notification::new().summary("Bufsy").body(text).show()?;
    println!("LOG {}", text);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() -> Result<()> {
        let test_message = "Hello, Bufsy test!";

        print(test_message)?;
        Ok(())
    }
}
