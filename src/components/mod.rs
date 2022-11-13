use crate::ServerCruncherApp;
type App = ServerCruncherApp;

mod status_bar;
pub use status_bar::StatusBar;

mod api_prefs_window;
pub use api_prefs_window::{ApiPerfsData, ApiPerfsWindow};

mod error_window;
pub use error_window::ErrorWindow;

mod application_window;
pub use application_window::ApplicationWindow;
