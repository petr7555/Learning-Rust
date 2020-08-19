use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write, Error};

struct PasswordEntry {
    url: String,
    login: String,
    password: String,
}

impl PasswordEntry {
    fn from(url: String, login: String, password: String) -> PasswordEntry {
        PasswordEntry {
            url,
            login,
            password,
        }
    }
}

fn write_header(writer: &mut BufWriter<File>) -> Result<usize, Error> {
    writer.write(b"title, url, login, password\n")
}

fn main() -> std::io::Result<()> {
    let arguments: Vec<_> = env::args().collect();
    if arguments.len() != 3 {
        panic!("Usage: {} <input file> <output file>", arguments[0])
    };

    let input_file = File::open(&arguments[1])?;
    let input_reader = BufReader::new(input_file);

    let mut password_entries = HashMap::new();

    let mut entry_vec = Vec::new();
    for line in input_reader.lines() {
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

    let output_file = File::create(&arguments[2])?;
    let mut output_writer = BufWriter::new(output_file);

    write_header(&mut output_writer)?;

    for (key, password_entry) in password_entries {
        output_writer.write_fmt(
            format_args!("{}, {}, {}, {}\n",
                        key, password_entry.url, password_entry.login, password_entry.password))?;
    }

    Ok(())
}