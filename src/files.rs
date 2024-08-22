use std::{fs::read_to_string, io, path::Path};

pub struct Files {
    pub template: String,
    pub css: String,
    pub config: String,
}

impl Files {
    pub fn load(template: &Path, css: &Path, config: &Path) -> Self {
        Self {
            template: get_file_contents(template),
            css: get_file_contents(css),
            config: get_file_contents(config),
        }
    }
}

fn get_file_contents(path: &Path) -> String {
    match read_to_string(path) {
        Ok(content) => content,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("File does not exist: {}", path.display());
                std::process::exit(1);
            }
            _ => panic!("Error reading file {}", path.display()),
        },
    }
}
