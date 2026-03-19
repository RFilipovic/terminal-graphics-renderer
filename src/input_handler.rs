use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::disable_raw_mode;

pub fn handle_input() -> Option<(char, f32)> {
    if event::poll(std::time::Duration::from_millis(100)).unwrap()
        && let Event::Key(key_event) = event::read().unwrap()
    {
        match key_event.code {
            KeyCode::Up => return Some(('x', 1.0)), // rotate x positive
            KeyCode::Down => return Some(('x', -1.0)), // rotate x negative
            KeyCode::Right => return Some(('y', 1.0)), // rotate y positive
            KeyCode::Left => return Some(('y', -1.0)), // rotate y negative
            KeyCode::Char('e') => return Some(('z', 1.0)), // rotate z positive
            KeyCode::Char('r') => return Some(('z', -1.0)), // rotate z negative
            KeyCode::Char('q') => {
                disable_raw_mode().unwrap();
                std::process::exit(0);
            }
            _ => {}
        }
    }
    None
}
