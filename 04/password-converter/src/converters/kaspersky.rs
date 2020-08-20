use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Result, Write};

use super::{Converter, PasswordEntry};

pub struct KasperskyPasswordManager {
    input: BufReader<File>,
    output: BufWriter<File>,
}

impl KasperskyPasswordManager {
    pub fn create(input: BufReader<File>, output: BufWriter<File>) -> KasperskyPasswordManager {
        KasperskyPasswordManager {
            input,
            output,
        }
    }
}

impl Converter for KasperskyPasswordManager {
    fn convert(mut self) -> Result<()> {
        let mut password_entries = HashMap::new();

        let mut entry_vec = Vec::new();
        for line in self.input.lines() {
            let current_line = line?;

            if current_line.contains(':') {
                let values: Vec<&str> = current_line.split(':').collect();
                entry_vec.push(String::from(values[1].trim()));
            }

            if current_line.contains("---") {
                let password_entry = PasswordEntry::from(
                    String::from(&entry_vec[0]),
                    String::from(&entry_vec[1]),
                    String::from(&entry_vec[2]));
                let key = format!("{} ({})", entry_vec[0], entry_vec[1]);
                password_entries.insert(key, password_entry);
                entry_vec.clear();
            }
        }

        self.output.write_all(b"title, url, login, password\n")?;

        for (key, password_entry) in password_entries {
            self.output.write_fmt(
                format_args!("{}, {}, {}, {}\n",
                             key, password_entry.url, password_entry.login, password_entry.password))?;
        }

        Ok(())
    }
}