
#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        text: String,
        write_mode: bool,
    },
}

impl AppState {
    pub fn initialized() -> Self {
        Self::Initialized {
            text: " ".to_owned(),
            write_mode: false,
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn is_write_mode(&self) -> bool {
        matches!(self, &Self::Initialized { write_mode: true, .. })
    }

    pub fn toggle_write_mode(&mut self, new_write_mode: bool) {
        if let Self::Initialized { write_mode, .. } = self {
            *write_mode = new_write_mode;
        }
    }

    pub fn get_text(&self) -> String {
        match self {
            Self::Initialized { text, write_mode } => {
                if !*write_mode {
                    let mut out: String = text.to_owned();
                    out.push_str("(input mode)");
                    out
                } else {
                    text.to_owned()
                }
            },
            _ => "".to_owned(),
        }
    }

    pub fn replace_text(&mut self, new_text: &str) {
        if let Self::Initialized { text, .. } = self {
            *text = new_text.to_owned();
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
