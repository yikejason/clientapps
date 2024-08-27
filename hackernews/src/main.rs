use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use hackernews::App;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}
