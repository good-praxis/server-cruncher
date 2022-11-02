#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::ServerCruncherApp;

mod api;
mod utils;

mod components;
pub use components::ServerWindow;
