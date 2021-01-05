use std::collections::HashMap;
use crate::converters::PasswordEntry;

pub struct LastpassPasswordManager {
    pub password_entries: HashMap<String, PasswordEntry>,
}

impl LastpassPasswordManager {
    pub fn create() -> LastpassPasswordManager {
        LastpassPasswordManager {
            password_entries: HashMap::new(),
        }
    }

    /// Adds new password entry
    /// ```
    /// use password_converter_lib::converters::lastpass::LastpassPasswordManager;
    ///
    /// let mut manager = LastpassPasswordManager::create();
    /// manager.add_password(String::from("www.amazon.com"),
    ///                   String::from("user"),
    ///                   String::from("password"));
    /// assert_eq!(1, manager.password_entries.len());
    /// ```
    pub fn add_password(&mut self, url: String, login: String, password: String) {
        let entry = PasswordEntry::from(
            String::from(&url),
            String::from(&login),
            String::from(&password));
        let title = format!("{} ({})", String::from(&url), String::from(&login));
        self.password_entries.insert(title, entry);
    }
}

#[cfg(test)]
mod tests {
    use crate::converters::lastpass::LastpassPasswordManager;

    #[test]
    fn after_adding_one_password_len_is_one() {
        let mut manager = LastpassPasswordManager::create();
        manager.add_password(String::from("www.amazon.com"),
                             String::from("user"),
                             String::from("password"));
        assert_eq!(1, manager.password_entries.len());
    }
}