use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub struct FullName {
    first_name: String,
    last_name: String,
}

impl FullName {
    pub fn new(first_name: &str, last_name: &str) -> FullName {
        FullName {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }
}
