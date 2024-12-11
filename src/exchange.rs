package src;

struct Exchange {
    name: String,
    id: String,
    r#type: String,
}

impl Exchange {
    fn to_string(&self) -> String {
        if self.name.is_empty() && self.id.is_empty() && self.r#type.is_empty() {
            return "Exchange { name: N/A, id: N/A, type: N/A }".to_string();
        }
        format!(
            "Exchange {{ name: {}, id: {}, type: {} }}",
            if self.name.is_empty() { "N/A" } else { &self.name },
            if self.id.is_empty() { "N/A" } else { &self.id },
            if self.r#type.is_empty() { "N/A" } else { &self.r#type }
        )
    }
}