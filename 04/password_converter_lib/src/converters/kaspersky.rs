use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Read, Result, Write};

use super::{Converter, PasswordEntry};

/// # KasperskyPasswordManager
/// Basic structure to store input and output
///
/// # Examples
/// ```
/// use std::fs::File;
/// use std::io::{BufReader, BufWriter};
/// use password_converter_lib::converters::kaspersky::KasperskyPasswordManager;
/// use password_converter_lib::converters::Converter;
///
/// let input_file = File::open("exported_passwords.txt").unwrap();
/// let input = BufReader::new(input_file);
///
/// let output_file = File::create("converted_passwords.csv").unwrap();
/// let output = BufWriter::new(output_file);
///
/// let mut manager = KasperskyPasswordManager::create(input, output);
/// manager.convert();
/// ```
pub struct KasperskyPasswordManager<T: Read, U: Write> {
    input: BufReader<T>,
    output: BufWriter<U>,
    password_entries: HashMap<String, PasswordEntry>,
}

impl<T: Read, U: Write> KasperskyPasswordManager<T, U> {
    pub fn create(input: BufReader<T>, output: BufWriter<U>) -> KasperskyPasswordManager<T, U> {
        KasperskyPasswordManager {
            input,
            output,
            password_entries: HashMap::new(),
        }
    }

    /// Adds new password entry
    pub fn add_password(&mut self, url: String, login: String, password: String) {
        let entry = PasswordEntry::from(
            String::from(&url),
            String::from(&login),
            String::from(&password));
        let title = format!("{} ({})", String::from(&url), String::from(&login));
        self.password_entries.insert(title, entry);
    }

    fn write_header(&mut self) -> Result<()> {
        self.output.write_all(b"title, url, login, password\n")?;
        Ok(())
    }

    fn write_entries(&mut self) -> Result<()> {
        for (key, password_entry) in &self.password_entries {
            self.output.write_fmt(
                format_args!("{}, {}, {}, {}\n",
                             key, password_entry.url, password_entry.login, password_entry.password))?;
        }

        Ok(())
    }
}

impl<T: Read, U: Write> Converter for KasperskyPasswordManager<T, U> {
    /// Converts kaspersky password file to csv
    fn convert(&mut self) -> Result<()> {
        let mut entry_vec = Vec::new();

        let mut line = String::new();
        while self.input.read_line(&mut line).unwrap() > 0 {
            // for l in self.input.by_ref().lines(){
            //     let line = l?;
            if line.contains(':') {
                let values: Vec<&str> = line.split(':').collect();
                entry_vec.push(String::from(values[1].trim()));
            }

            if line.contains("---") {
                self.add_password(String::from(&entry_vec[0]),
                                  String::from(&entry_vec[1]),
                                  String::from(&entry_vec[2]));
                entry_vec.clear();
            }
            line.clear();
        }

        self.write_header()?;
        self.write_entries()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::converters::Converter;
    use crate::converters::kaspersky::KasperskyPasswordManager;

    #[test]
    fn kaspersky_converts_passwords() {
        use std::fs::File;
        use std::io::{BufReader, BufWriter};
        let input_file = File::open("exported_passwords.txt").unwrap();
        let input = BufReader::new(input_file);
        let output_file = File::create("converted_passwords.csv").unwrap();
        let output = BufWriter::new(output_file);
        let mut manager = KasperskyPasswordManager::create(input, output);
        manager.convert().unwrap();
    }
}