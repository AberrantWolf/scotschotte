// mod components;
// #[macro_use]
// mod ui_util;

mod sc_core;

// use components::schotte_core::SchotteApp;
use std::env::args;

fn main() {
    println!("I. Am. Vulkanized.");
    let app = sc_core::init_app().unwrap();
    app.run();
}
