#[macro_use]
extern crate serde;

mod models;
mod trainers;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
