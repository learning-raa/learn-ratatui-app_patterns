#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::model::{AppModel as Model, AppModelState as ModelState};
use anyhow::Result;
use ratatui::crossterm::event as xEvent;

#[derive(Debug)]
pub enum Message {
    InputEvent(xEvent::Event),
    Quit,
}

pub fn update(model: &mut Model, msg: &Message) -> Result<Option<Message>> {
    match (&model.state, msg) {
        (_, Message::Quit) => {
            model.state = ModelState::Exiting;
            return Ok(None);
        }
        (ModelState::OffFocused, Message::InputEvent(ev)) => {
            if let xEvent::Event::Key(key) = ev {
                if key.code == xEvent::KeyCode::Char('q') {
                    return Ok(Some(Message::Quit));
                }
                if key.code == xEvent::KeyCode::Char('e') {
                    model.state = ModelState::EditorFocused;
                    return Ok(None);
                }
            }
            return Ok(None);
        }
        (ModelState::EditorFocused, Message::InputEvent(ev)) => {
            if let xEvent::Event::Key(key) = ev {
                if key.code == xEvent::KeyCode::Char('e') {
                    if key.modifiers.contains(xEvent::KeyModifiers::CONTROL) {
                        model.state = ModelState::OffFocused;
                        return Ok(None);
                    }
                }
            }
            model.ed_handler.on_event(ev.clone(), &mut model.ed_state);
            return Ok(None);
        }
        _ => {
            trace!("unprocessed Message:\n{:?}", msg);
            return Ok(None);
        }
    }
}
