#![warn(clippy::all, rust_2018_idioms)]

pub mod a;
mod app;
pub mod real;
pub use app::App;
pub mod auth;
pub mod exams;
pub mod stuff;
pub mod line;
pub mod http;
pub mod util;
pub mod user;
pub mod ylt;