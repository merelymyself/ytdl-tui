use std::{path::PathBuf, str::Chars, iter, process::Command};
use dirs::home_dir;

pub struct App {
    pub url: String,
    pub folder: String,
    pub border: i32,
    pub format_status: i32,
}

impl App {
    pub fn new() -> App {
        return App {
            url: String::from("youtube.com/"),
            folder: String::from(home_dir().unwrap_or(PathBuf::from("")).to_string_lossy().to_string()),
            border: 0,
            format_status: 1,
        }
    }
    pub fn on_key(&mut self, c:char) {
        if self.border == 0 {
            self.url.push(c);
        }
        else if self.border == 1 {
            self.folder.push(c);
        }
    }
    pub fn on_backspace(&mut self) {
        if self.border == 0 {
            self.url.pop();
        }
        else if self.border == 1 {
            self.folder.pop();
        }
    }
    pub fn on_down(&mut self) {
        if self.border == 1 || self.border == 0 {
            self.border += 1;
        }
        else if self.border == 2 && self.format_status != 9 {
            self.format_status += 1;
        }
    }
    pub fn on_up(&mut self) {
        if self.border == 2 && self.format_status != 1 {
            self.format_status -= 1;
        }
        else if self.border != 0 {
            self.border -= 1;
        }
    }
    pub fn on_left(&mut self) {
        if self.border != 0{
            self.border -= 1;
        }
    }
    pub fn on_right(&mut self) {
        if self.border < 2 {
            self.border+=1;
        }
    }
    pub fn on_enter(&mut self) {
        if self.border < 2 {
            self.border += 1;
        }
        if self.border == 2 {
            launch_command(self);  
        }
    }
}

fn launch_command(app: &mut App) {
    Command::new("youtube-dl")
        .arg("-o")
        .arg(format!("\"{}%(title)s.%(ext)s\"", app.folder.clone()))
        .spawn();
}
