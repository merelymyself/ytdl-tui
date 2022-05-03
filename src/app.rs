use std::path::PathBuf;
use dirs::home_dir;

pub struct App {
    pub url: String,
    pub folder: String,
    pub border: i32
}

impl App {
    pub fn new() -> App {
        return App {
            url: String::from("youtube.com/"),
            folder: String::from(home_dir().unwrap_or(PathBuf::from("")).to_string_lossy().to_string()),
            border: 0
        }
    }
}