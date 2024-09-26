#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

//  //  //  //  //  //  //  //
pub struct AppModel {
    ed_state: edtui::EditorState,
    ed_handler: edtui::EditorEventHandler,
    pub(super) exiting: bool,
}

impl AppModel {
    pub fn new() -> Self {
        let new_model = Self {
            ed_state: edtui::EditorState::new(edtui::Lines::from("started text.\n\nline 3\nFIN")),
            ed_handler: edtui::EditorEventHandler::default(),
            exiting: false,
        };

        trace!(" + AppModel::new()");
        new_model
    }

    pub fn is_exiting(&self) -> bool {
        self.exiting
    }
}
