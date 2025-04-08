use cli_log::info;

#[derive(Debug, Clone, Default)]
pub struct DatabaseTable {
    pub name: String,
    pub columns: Vec<String>,
    pub is_focused: bool,
    pub data: Vec<String>,
}

impl DatabaseTable {
    pub fn new(name: String, columns: Vec<String>) -> Self {
        Self {
            name,
            columns,
            is_focused: false,
            data: Vec::new(),
        }
    }

    pub fn set_columns(&mut self, columns: Vec<String>) {
        self.columns = columns;
    }

    pub fn set_data(&mut self, data: Vec<String>) {
        info!("Setting data: {:?}", data);
        self.data = data;
    }
}
