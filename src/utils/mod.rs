mod utils {
    use std::fs;
    use std::io::{self, Write};

    pub fn read_file_to_string(file_path: &str) -> io::Result<String> {
        fs::read_to_string(file_path)
    }

    pub fn write_string_to_file(file_path: &str, content: &str) -> io::Result<()> {
        let mut file = fs::File::create(file_path)?;
        file.write_all(content.as_bytes())
    }

    pub fn append_string_to_file(file_path: &str, content: &str) -> io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(file_path)?;
        file.write_all(content.as_bytes())
    }
}