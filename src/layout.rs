
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::{
    app::AppState,
    settings::UserSettings,
    widgets::{
        WeatherStatefulWidgetRef,
        home::HomeWidget
    }
};

#[derive(Debug)]
pub enum LayoutMode {
    LargeScreen
}

#[derive(Debug)]
pub struct MainWidget {
    widgets: Vec<Box<dyn WeatherStatefulWidgetRef>>
}

impl Default for MainWidget {
    fn default() -> Self {
        let settings = UserSettings::default();
        let widgets = Vec::from([
            Box::new(HomeWidget::default())
        ]);
        Self {
            widgets: Vec::from(widgets),
        }
    }
}

impl StatefulWidget for &MainWidget {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        for widget in self.widgets.into_iter() {
            widget.render(area, buf, state);
        }
    }
}

impl MainWidget {
    fn handle_events(&mut self) -> io::Result<()> {
        for widget in self.widgets.into_iter(){
            widget.handle_events();
        }
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.main_layout.handle_key_events(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(event: KeyEvent) {
    }
}
