//! Command-line executable code for The Anarchist Workshop.
use clap::App;

pub fn main() {
    App::new("taw").version("v0.1.0").get_matches();
}
