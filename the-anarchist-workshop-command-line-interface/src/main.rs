//! Command-line executable code for The Anarchist Workshop.
use clap::App;
use the_anarchist_workshop::the_calendar::TheCalendar;

/// This is the function that gets executed when you type `taw` on your terminal.
/// Who knows? Maybe you want to spawn it on a thread somewhere. Maybe you want
/// to stick to the main thread.
pub fn main() {
    App::new("taw").version("v0.1.0").get_matches();
}
