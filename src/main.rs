use std::rc::Rc;
use std::cell::RefCell;

mod frontend {
    pub mod ui;
}

fn main() -> Result<()> {
    let app = Rc::new(RefCell::new(App::new()));
    frontend::ui::start_ui(app)?;
    Ok(())
}
