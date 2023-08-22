#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod request;
mod collection;
mod auth;
mod tab_viewer;

pub use app::LettersApp;
