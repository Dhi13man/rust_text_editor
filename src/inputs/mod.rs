use self::key::Key;

pub mod events;
pub mod key;

pub enum InputEvent {
    /// An input event occurred.
    Input(Key),
    Tick,
}
