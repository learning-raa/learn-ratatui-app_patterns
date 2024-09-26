use anyhow::Result;
use super::model::AppModel as Model;
use ratatui::crossterm::event as xEvent;


pub enum Message {
    InputEvent(xEvent::Event),
    Quit,
}


pub fn update(model: &mut Model, msg: &Message) -> Result<Option<Message>> {
    match msg {
        Message::Quit => model.exiting = true,
        _ => {}
    }
    Ok(None)
}
