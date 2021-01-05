use std::io::Result;

/// # Module converters
/// Provides a common interface for all available password format converters.
pub mod kaspersky;
pub mod lastpass;

pub struct PasswordEntry {
    pub url: String,
    pub login: String,
    pub password: String,
}

impl PasswordEntry {
    pub fn from(url: String, login: String, password: String) -> PasswordEntry {
        PasswordEntry {
            url,
            login,
            password,
        }
    }
}

/// # Converter trait
pub trait Converter {
    /// Converts passwords
    fn convert(& mut self) -> Result<()>;
}