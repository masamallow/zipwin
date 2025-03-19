use anyhow::Result;
use encoding_rs::SHIFT_JIS;

/// Convert UTF-8 file name to CP932 encoding.
pub fn convert_to_cp932(filename: &str) -> Result<String> {
    let (encoded, _, _) = SHIFT_JIS.encode(filename);
    Ok(String::from_utf8_lossy(&encoded).to_string())
}
