use super::open_files_data::OpenFilesData;


#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        write_mode: bool,
        files_data: OpenFilesData,
    },
}

impl AppState {
    pub fn initialized() -> Self {
        Self::Initialized {
            write_mode: false,
            files_data: OpenFilesData::new(),
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
            Self::Initialized { files_data, write_mode } => {
                if !*write_mode {
                    let mut out: String = files_data.get_currently_selected_file_content();
                    out.push_str("(input mode)");
                    out
                } else {
                    files_data.get_currently_selected_file_content()
                }
            },
            _ => "".to_owned(),
        }
    }

    pub fn replace_text(&mut self, new_text: &str) {
        if let Self::Initialized { files_data, .. } = self {
            files_data.replace_currently_selected_file_content(new_text);
        }
    }

    pub fn get_path(&self) -> String {
        match self {
            Self::Initialized { files_data, .. } => files_data.get_currently_selected_file_path(),
            _ => "..loading..".to_owned(),
        }
    }

    pub fn get_all_open_file_names(&self) -> String {
        match self {
            Self::Initialized { files_data, .. } => files_data.get_open_file_names().join(", "),
            _ => "..loading..".to_owned(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
