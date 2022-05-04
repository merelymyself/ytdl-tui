use std::path::PathBuf;
use dirs::home_dir;

pub struct App {
    pub url: String,
    pub folder: String,
    pub border: i32,
    pub format_status: i32,
    pub format: i32,
    pub should_quit: bool
}

impl App {
    pub fn new() -> App {
        return App {
            url: String::from("youtube.com/"),
            folder: String::from(home_dir().unwrap_or(PathBuf::from("")).to_string_lossy().to_string()),
            border: 0,
            format_status: 0,
            format: 0,
            should_quit: false
        }
    }
}
