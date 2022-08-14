use std::path::Path;

#[derive(Clone)]
pub struct OpenFilesData {
    file_paths: Vec<String>,
    file_contents: Vec<String>,
    currently_selected_file_index: usize,
}

impl OpenFilesData {
    pub fn new() -> Self {
        Self {
            file_paths: vec![],
            file_contents: vec![],
            currently_selected_file_index: 0,
        }
    }

    pub fn open_file(&mut self, file_path: &str) -> Result<(), String> {
        if self.file_paths.contains(&file_path.to_owned()) {
            return Err(format!("File {} already opened", file_path));
        } else {
            if !Path::new(file_path).exists() {
                Err(format!("File {} does not exist", file_path))
            } else {
                let file_content = std::fs::read_to_string(file_path);
                if let Ok(file_content) = file_content {
                    self.file_paths.push(file_path.to_owned());
                    self.file_contents.push(file_content);
                    self.currently_selected_file_index = self.file_paths.len() - 1;
                    Ok(())
                } else {
                    Err(format!("Error while reading file {}", file_path))
                }
            }
        }
    }

    pub fn close_file(&mut self) -> Result<(), String> {
        if self.file_paths.is_empty() {
            return Err("No file to close".to_owned());
        } else {
            self.file_paths.remove(self.currently_selected_file_index);
            self.file_contents.remove(self.currently_selected_file_index);
            self.select_previous_file();
            Ok(())
        }
    }

    pub fn get_open_file_paths(&self) -> &Vec<String> {
        &self.file_paths
    }

    pub fn get_open_file_names(&self) -> Vec<String> {
        self.file_paths.iter()
            .map(|file_path| file_path.split("/").last().unwrap().to_owned())
            .collect::<Vec<String>>()
    }

    pub fn get_open_file_contents(&self) -> &Vec<String> {
        &self.file_contents
    }

    pub fn get_currently_selected_file_content(&self) -> String {
        if self.currently_selected_file_index < self.file_contents.len() {
            self.file_contents[self.currently_selected_file_index].clone()
        } else {
            "".to_owned()
        }
    }

    pub fn replace_currently_selected_file_content(&mut self, new_content: &str) {
        if self.currently_selected_file_index < self.file_contents.len() {
            self.file_contents[self.currently_selected_file_index] = new_content.to_owned();
        } else {
            self.file_contents.push(new_content.to_owned());
        }
    }

    pub fn get_currently_selected_file_path(&self) -> String {
        if self.currently_selected_file_index < self.file_paths.len() {
            self.file_paths[self.currently_selected_file_index].clone()
        } else {
            "/(unsaved)".to_owned()
        }
    }

    pub fn get_currently_selected_file_name(&self) -> String {
        let path = self.get_currently_selected_file_path();
        path.split("/").last().unwrap().to_owned()
    }

    pub fn select_next_file(&mut self) {
        if self.file_contents.len() > 0 {
            self.currently_selected_file_index = (self.currently_selected_file_index + 1) % self.file_contents.len();
        } else {
            self.currently_selected_file_index = 0;
        }
    }

    pub fn select_previous_file(&mut self) {
        if self.file_contents.len() > 0 {
            self.currently_selected_file_index = (self.currently_selected_file_index + self.file_paths.len() - 1) % self.file_paths.len();
        } else {
            self.currently_selected_file_index = 0;
        }
    }

    pub fn save_file(&mut self) -> Result<(), String> {
        let file_content = self.file_contents[self.currently_selected_file_index].clone();
        if self.currently_selected_file_index < self.file_paths.len() {
            let file_path = self.file_paths[self.currently_selected_file_index].clone();
            std::fs::write(&file_path, file_content).map_err(|e| format!("Error while writing file {}: {}", file_path, e))
        } else {
            let random_file_name: String = names::Generator::default().next().unwrap();
            // Take file path input from user
            self.file_paths.push(random_file_name.clone());
            std::fs::write(&random_file_name, file_content).map_err(|e| format!("Error while writing file {}: {}", random_file_name, e))
        }
    }
}