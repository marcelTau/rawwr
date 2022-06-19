#[derive(Debug, PartialEq, Clone)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: String)  -> Self {
        Self {
            name
        }
    }
}

impl std::string::ToString for Class {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}
