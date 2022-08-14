pub mod handler;
// For this dummy application we only need two IO event
#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize,      // Launch to initialize the application
    ToggleWriteMode(bool), // Toggle whether Write Mode is active
    OpenFile,        // Open a file
    SaveFile,        // Save a file
    NextFile,        // Go to next file
    PreviousFile,    // Go to previous file
    CloseFile,      // Close the current file
    ScrollDown,      // Scroll down
    ScrollUp,        // Scroll up
    ScrollLeft,      // Scroll left
    ScrollRight,     // Scroll right
}
 