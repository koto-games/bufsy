use super::print;
use anyhow::Result;
use wl_clipboard_rs::copy::{MimeType, Options, Source};

pub fn fnthost(text: String) -> Result<()> {
    print(&format!("log: {}", text))?;
    let opts = Options::new();
    opts.copy(
        Source::Bytes(text.to_string().into_bytes().into()),
        MimeType::Autodetect,
    )?;

    Ok(())
}
