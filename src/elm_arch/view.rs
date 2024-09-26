use super::model::AppModel as Model;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Wrap};


pub fn view(model: &Model, frame: &mut Frame) {
    let area = frame.area();
    // layout
    let [title_area, main_area, status_area] = Layout::vertical([
        Constraint::Length(10),
        Constraint::Min(19),
        Constraint::Min(2),
    ])
    .areas(area);

    frame.render_widget(
        Paragraph::new("main title here")
            .block(Block::bordered().title("title of Main Title")),
        title_area
    );
        //.render(title_area, buf);
    /*
    let main_block = Block::bordered();
    {
        let main_inner = main_block.inner(main_area);
        let [main_left, main_right] =
            Layout::horizontal([Constraint::Length(3), Constraint::Min(16)]).areas(main_inner);

        main_block.render(main_area, buf);
        Paragraph::new("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\nA\nB\nC\nD\nE\nF\n<-->")
            .render(main_left, buf);
        edtui::EditorView::new(&mut self.ed_state).render(main_right, buf);
    }
    // status info
    Paragraph::new(self.status.clone())
        .wrap(Wrap { trim: true })
        .block(Block::bordered().title("debug information:"))
        .render(status_area, buf);
    */
}
