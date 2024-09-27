use super::model::AppModel as Model;
use super::model::AppModelState as ModelState;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Wrap};

// edtui requires mut state for new()
pub fn view(model: &mut Model, frame: &mut Frame) {
    //pub fn view(model: &Model, frame: &mut Frame) {
    let area = frame.area();
    // layout
    let [title_area, main_area, status_area] = Layout::vertical([
        Constraint::Length(10),
        Constraint::Min(19),
        Constraint::Min(2),
    ])
    .areas(area);

    let title =
        Paragraph::new("main title here").block(Block::bordered().title("title of Main Title"));
    let main_block = Block::bordered();
    let main_inner = main_block.inner(main_area);
    let [main_left, main_right] =
        Layout::horizontal([Constraint::Length(3), Constraint::Min(16)]).areas(main_inner);
    let main_left_text = Paragraph::new("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\nA\nB\nC\nD\nE\nF\n<-->");
    let main_right_ed = edtui::EditorView::new(&mut model.ed_state);
    let st = match model.state {
        ModelState::EditorFocused => "in focus",
        _ => "out of focus",
    };

    // status info
    let status = Paragraph::new(st)
        .wrap(Wrap { trim: true })
        .block(Block::bordered().title("debug information:"));

    // drawing
    frame.render_widget(title, title_area);
    frame.render_widget(main_block, main_area);
    frame.render_widget(main_left_text, main_left);
    frame.render_widget(main_right_ed, main_right);
    frame.render_widget(status, status_area);
}
