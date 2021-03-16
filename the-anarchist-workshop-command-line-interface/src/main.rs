//! Command-line executable code for The Anarchist Workshop.
use cursive::views::{Dialog, TextView};
use structopt::StructOpt;
//use the_anarchist_workshop::the_calendar::TheCalendar;

pub const CLI_APP_NAME: &str = "taw";
pub const CLI_APP_VERSION: &str = "v0.1.0";

#[derive(Debug, StructOpt)]
#[structopt(name = CLI_APP_NAME, version = CLI_APP_VERSION)]
pub struct TheCliApp {
    #[structopt(
        short,
        long,
        help = "Sets verbosity.\n-v     ERROR\n-vv    WARN\n-vvv   INFO\n-vvvv  DEBUG\n-vvvvv TRACE",
        parse(from_occurrences)
    )]
    verbose: u8,
    #[structopt(subcommand)]
    cmd: Option<TAWcommands>,
}

#[derive(Debug, StructOpt)]
pub enum TAWcommands {
    #[structopt(
        about = "Run the interactive interface on your shell",
        long_about = "Reminds me of the midnight commander, but less cool."
    )]
    Commander,
}

/// This is the function that gets executed when you type `taw` on your terminal.
/// Who knows? Maybe you want to spawn it on a thread somewhere. Maybe you want
/// to stick to the main thread.
pub fn main() {
    let cli = TheCliApp::from_args();
    println!("{:?}", cli);
    match cli.cmd {
        Some(cmd) => {
            match cmd {
                TAWcommands::Commander => {
                    {
                        let mut siv = cursive::default();

                        siv.add_layer(
                            Dialog::around(TextView::new("TAW"))
                                .title("The Anarchist's Workshop")
                                .button("Salir", |s| s.quit()),
                        );

                        siv.run();
                    }
                }
            }
        }
        None => {
            println!("Type \"taw -h\" or \"taw --help\" to begin.")
        }
    }
}
