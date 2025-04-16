use std::collections::HashMap;

struct ConnectionOptions {
    pub host: String,
    pub user: String,
    pub db_name: String,
    pub password: Option<String>,
}

pub struct CredentialManager {
    pub credentials: HashMap<String, ConnectionOptions>,
}

// TODO: Manage credentials by saving to sqlite
// and save passwords usingn keyring to persist across sessions
impl CredentialManager {
    pub fn new() -> CredentialManager {
        CredentialManager {
            credentials: HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, name: String, options: ConnectionOptions) {
        self.credentials.insert(name, options);
    }

    pub fn get_connection(&self, name: &str) -> Option<&ConnectionOptions> {
        self.credentials.get(name)
    }

    pub fn remove_connection(&mut self, name: &str) {
        self.credentials.remove(name);
    }
}
