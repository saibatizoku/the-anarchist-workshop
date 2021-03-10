//! Command-line executable code for The Anarchist Workshop.
use structopt::StructOpt;
//use the_anarchist_workshop::the_calendar::TheCalendar;

pub const CLI_APP_NAME: &str = "taw";
pub const CLI_APP_VERSION: &str = "v0.1.0";

#[derive(Debug, StructOpt)]
#[structopt(name = CLI_APP_NAME, version = CLI_APP_VERSION)]
pub struct TheCliApp {
}

/// This is the function that gets executed when you type `taw` on your terminal.
/// Who knows? Maybe you want to spawn it on a thread somewhere. Maybe you want
/// to stick to the main thread.
pub fn main() {
    let cli = TheCliApp::from_args();
    println!("{:?}", cli);
}
