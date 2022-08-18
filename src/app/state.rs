use super::open_files_data::OpenFilesData;

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        write_mode: bool,
        scroll_offset: (u16, u16),
        files_data: OpenFilesData,
    },
}

impl AppState {
    pub fn initialized() -> Self {
        Self::Initialized {
            write_mode: false,
            scroll_offset: (0, 0),
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
            Self::Initialized { files_data, write_mode,  .. } => {
                if !*write_mode {
                    let mut out: String = files_data.get_currently_selected_file_content();
                    out.push_str(" (input mode)");
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

    pub fn get_path(&mut self) -> String {
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

    pub fn get_scroll_offset(&self) -> &(u16, u16) {
        match self {
            Self::Initialized { scroll_offset, .. } => scroll_offset,
            _ => &(0, 0),
        }
    }

    pub fn scroll_vertical(&mut self, delta: i32) -> Result<(), String> {
        let text = self.get_text();
        if let Self::Initialized { scroll_offset, .. } = self {
            let (x, y) = scroll_offset;
            if delta > 0 {
                if *y < text.lines().count() as u16 {
                    *scroll_offset = (*x, *y + delta as u16);
                    Ok(())
                } else {
                    Err("Cannot scroll past end of file".to_owned())
                }
            } else if delta < 0 {
                if *y > 0 {
                    *scroll_offset = (*x, (*y as i32 + delta).max(0) as u16);
                    Ok(())
                } else {
                    Err("Cannot scroll past start of file".to_owned())
                }
            } else {
                Ok(())
            }
        } else {
            Err("Not initialized".to_owned())
        }
    }

    pub fn scroll_horizontal(&mut self, delta: i32) -> Result<(), String> {
        let text = self.get_text();
        if let Self::Initialized { scroll_offset, .. } = self {
            let (x, y) = scroll_offset;
            if delta > 0 {
                if *x < text.lines().nth(*y as usize).unwrap().len() as u16 {
                    *scroll_offset = (*x + delta as u16, *y);
                    Ok(())
                } else {
                    Err("Cannot scroll past end of line".to_owned())
                }
            } else if delta < 0 {
                if *x > 0 {
                    *scroll_offset = ((*x as i32 + delta).max(0) as u16, *y);
                    Ok(())
                } else {
                    Err("Cannot scroll past start of line".to_owned())
                }
            } else {
                Ok(())
            }
        } else {
            Err("Not initialized".to_owned())
        }
    }

    pub fn reset_scroll(&mut self) {
        if let Self::Initialized { scroll_offset, .. } = self {
            *scroll_offset = (0, 0);
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
