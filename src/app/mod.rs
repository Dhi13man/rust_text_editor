use log::{debug, error, warn};

use self::actions::Actions;
use self::state::AppState;
use crate::app::actions::Action;
use crate::inputs::key::Key;
use crate::io::IoEvent;

pub mod actions;
pub mod state;
pub mod ui;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

/// The main application, containing the state
pub struct App {
    /// We could dispatch an IO event
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    /// Contextual actions
    actions: Actions,
    /// State
    is_loading: bool,
    state: AppState,
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> Self {
        let actions = vec![Action::Quit].into();
        let is_loading = false;
        let state = AppState::default();

        Self {
            io_tx,
            actions,
            is_loading,
            state,
        }
    }

    /// Handle a user action
    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(value) = self.attempt_write(key) {
            value
        } else if let Some(action) = self.actions.find(key) {
            debug!("Run action [{:?}]", action);
            match action {
                Action::Quit => AppReturn::Exit,
                // Write o clock
                Action::BeginWriteMode => {
                    self.dispatch(IoEvent::ToggleWriteMode(true)).await;
                    AppReturn::Continue
                }
                // No more writing
                Action::EndWriteMode => {
                    self.dispatch(IoEvent::ToggleWriteMode(false)).await;
                    AppReturn::Continue
                }
            }
        } else {
            warn!("No action accociated to {}", key);
            AppReturn::Continue
        }
    }

    fn attempt_write(&mut self, key: Key) -> Option<AppReturn> {
        if self.state.is_write_mode() {
            let mut curr_text = self.state.get_text();
            match key {
                Key::Backspace => {
                    curr_text.pop();
                    self.state.replace_text(&curr_text);
                    Some(AppReturn::Continue)
                },

                Key::Space => {
                    curr_text.push(' ');
                    self.state.replace_text(&curr_text);
                    Some(AppReturn::Continue)
                },

                Key::Char(key_char) => {
                    curr_text.push(key_char);
                    self.state.replace_text(&curr_text);
                    Some(AppReturn::Continue)
                },

                _ => None,
            }
        } else {
            None
        }
    }

    /// Send a network event to the IO thread
    pub async fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async action has finished in io/handler.rs
        self.is_loading = true;
        if let Err(e) = self.io_tx.send(action).await {
            self.is_loading = false;
            error!("Error from dispatch {}", e);
        };
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }
    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn initialized(&mut self) {
        // Update contextual actions
        self.actions = vec![
            Action::Quit,
            Action::BeginWriteMode,
            Action::EndWriteMode,
        ]
        .into();
        self.state = AppState::initialized()
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }

    pub fn toggle_write_mode(&mut self, new_write_mode: bool) {
        self.state.toggle_write_mode(new_write_mode);
    }
}
