use std::sync::Arc;

use eyre::Result;
use log::{error, info, warn};
use copypasta::{ClipboardContext, ClipboardProvider};

use super::IoEvent;
use crate::app::{App};

/// In the IO thread, we handle IO event without blocking the UI thread
pub struct IoAsyncHandler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    /// We could be async here
    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::ToggleWriteMode(write_mode) => self.toggle_write_mode(write_mode).await,
            IoEvent::OpenFile => self.open_file().await,
            IoEvent::SaveFile => self.save_file().await,
            IoEvent::NextFile => self.next_file().await,
            IoEvent::PreviousFile => self.previous_file().await,
            IoEvent::CloseFile => self.close_file().await,
            IoEvent::ScrollDown => self.scroll_vertical(1).await,
            IoEvent::ScrollUp => self.scroll_vertical(-1).await,
            IoEvent::ScrollLeft => self.scroll_horizontal(-1).await,
            IoEvent::ScrollRight => self.scroll_horizontal(1).await,
        };

        if let Err(err) = result {
            error!("Oops, something wrong happen: {:?}", err);
        }

        let mut app = self.app.lock().await;
        app.loaded();
    }

    /// Initialize App
    async fn do_initialize(&mut self) -> Result<()> {
        info!("🚀 Initialize the application");
        let mut app = self.app.lock().await;
        app.initialized(); // we could update the app state
        info!("👍 Application initialized");
        Ok(())
    }

    /// Toggle between Write and Input mode
    async fn toggle_write_mode(&mut self, new_write_mode: bool) -> Result<()> {
        info!("Setting Write Mode to {:?}...", new_write_mode);
        // Notify the app for having slept
        let mut app = self.app.lock().await;
        app.toggle_write_mode(new_write_mode);
        Ok(())
    }

    /// Open a file
    async fn open_file(&mut self) -> Result<()> {
        let mut ctx = ClipboardContext::new().unwrap();
        if let Some(clipboard_text) = ctx.get_contents().ok() {
            let mut app = self.app.lock().await;
            let result = app.open_files_data_mut().open_file(&clipboard_text);
            match result {
                Ok(()) => {
                    info!("📄 Opened file: {}", clipboard_text);
                    Ok(())
                },
                Err(err) => {
                    error!("📄 Failed to open file: {}", err);
                    Ok(())
                }
            }
        } else {
            warn!("📄 Open file: <empty>");
            Ok(())
        }
    }

    /// Close the file
    async fn close_file(&mut self) -> Result<()> {
        let mut app = self.app.lock().await;
        let current_opened_file_path = app.open_files_data_mut().get_currently_selected_file_path();
        let result = app.open_files_data_mut().close_file();
        match result {
            Ok(()) => {
                info!("📄 Closed file: {}", current_opened_file_path);
                Ok(())
            },
            Err(err) => {
                error!("📄 Failed to Close file: {}", err);
                Ok(())
            }
        }
    }

    /// Save the file
    async fn save_file(&mut self) -> Result<()> {
        let mut app = self.app.lock().await;
        let current_opened_file_path = app.open_files_data_mut().get_currently_selected_file_path();
        let result = app.open_files_data_mut().save_file();
        match result {
            Ok(()) => {
                info!("📄 Saved file: {}", current_opened_file_path);
                Ok(())
            }
            Err(err) => {
                error!("📄 Failed to save file: {}", err);
                Ok(())
            }
        }
    }

    /// Next file
    async fn next_file(&mut self) -> Result<()> {
        let mut app = self.app.lock().await;
        app.open_files_data_mut().select_next_file();
        app.reset_scroll();
        Ok(())
    }

    /// Previous file
    async fn previous_file(&mut self) -> Result<()> {
        let mut app = self.app.lock().await;
        app.open_files_data_mut().select_previous_file();
        app.reset_scroll();
        Ok(())
    }

    /// Scroll vertical
    /// direction: 1 for down, -1 for up
    async fn scroll_vertical(&mut self, direction: i32) -> Result<()> {
        let mut app = self.app.lock().await;
        match app.scroll_horizontal(direction) {
            Ok(()) => {
                info!("↨ Scrolled vertical. Current Scroll Offset: {}", direction);
                Ok(())
            },
            Err(err) => {
                error!("Failed to scroll vertical: {}", err);
                Ok(())
            }
        }
    }

    /// Scroll horizontal
    /// direction: 1 for right, -1 for left
    async fn scroll_horizontal(&mut self, direction: i32) -> Result<()> {
        let mut app = self.app.lock().await;
        match app.scroll_horizontal(direction) {
            Ok(()) => {
                info!("🔛 Scrolled horizontal. Current Scroll Offset: {}", direction);
                Ok(())
            },
            Err(err) => {
                error!("Failed to scroll horizonta: {}", err);
                Ok(())
            }
        }
    }
}
