use std::io::Result;

pub mod kaspersky;

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

pub trait Converter {
    fn convert(self) -> Result<()>;
}