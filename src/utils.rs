// imports
use rust_embed;

// ----- `StaticAssets` object
#[derive(rust_embed::Embed)]
#[folder = "static"]
pub struct StaticAssets;

// get files contents from embedded files i.e `static` directory
pub fn get_embedded_file(filepath: String) -> Option<Result<String, String>> {
    match StaticAssets::get(filepath.as_str()) {
        Some(file_content) => {
            return Some(match String::from_utf8(file_content.data.to_vec()) {
                Ok(safe_value) => Ok(safe_value),
                Err(e) => Err(e.to_string()),
            });
        }
        None => {
            return None;
        }
    }
}
