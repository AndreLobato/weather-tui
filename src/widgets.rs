use crate::{app::AppState, settings::UserSettings};
use color_eyre::eyre::Result;
use crossterm::event::{Event, KeyEvent, MouseEvent};
use ratatui::{
    buffer::Buffer, layout::{Rect, Size}
};

pub mod home;
mod location;
mod map;
mod summary;
mod timeseries;


pub trait WeatherStatefulWidgetRef {
    fn name(&self) -> &str;
    fn handle_events(&mut self, event: Option<Event>) -> Option<Result<()>> {
        match event {
            Some(Event::Key(key_event)) => self.handle_key_event(key_event),
            Some(Event::Mouse(mouse_event)) => self.handle_mouse_event(mouse_event),
            _ => None,
        };
        Some(Ok(()))
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<Result<()>> {
        let _ = key;
        Some(Ok(()))
    }

    fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Option<Result<()>> {
        let _ = mouse; // to appease clippy
        Some(Ok(()))
    }

    fn update(&mut self) -> Result<()> {
        Ok(())
    }

    fn render(self, area: Rect, buf: &mut Buffer, state:&mut AppState){
        let _ = area;
        let _ = buf;
        let _ = state;
    }
}

