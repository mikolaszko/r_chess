pub struct Fen {
    pub file_path: String,
}

impl Fen {
    pub fn process(&self) {
        let fields: Vec<&str> = self
            .file_path
            .split(|c| c == '/' || c == ' ')
            .to_owned()
            .collect();
    }
}
