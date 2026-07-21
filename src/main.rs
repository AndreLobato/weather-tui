use std::io;

use crate::app::App;
mod app;
mod cache;
mod input;
mod layout;
mod settings;
mod widgets;

fn main() -> io::Result<()> {
    ratatui::run(
        |terminal| 
            App::default().run(terminal)
    )
} 
