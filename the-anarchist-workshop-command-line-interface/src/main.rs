//! Command-line executable code for The Anarchist Workshop.
use cursive::views::{Dialog, TextView};
use structopt::StructOpt;
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, prelude::*};
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

#[derive(Copy, Clone, Debug, StructOpt)]
pub enum Verbosity {
    None,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl From<u8> for Verbosity {
    fn from(v: u8) -> Self {
        match v {
            0 => Verbosity::None,
            1 => Verbosity::Error,
            2 => Verbosity::Warning,
            3 => Verbosity::Info,
            4 => Verbosity::Debug,
            _ => Verbosity::Trace,
        }
    }
}

impl From<Verbosity> for LevelFilter {
    fn from(v: Verbosity) -> LevelFilter {
        use Verbosity::*;
        match v {
            None => LevelFilter::OFF,
            Error => LevelFilter::ERROR,
            Warning => LevelFilter::WARN,
            Info => LevelFilter::INFO,
            Debug => LevelFilter::DEBUG,
            Trace => LevelFilter::TRACE,
        }
    }
}

/// This is the function that gets executed when you type `taw` on your terminal.
/// Who knows? Maybe you want to spawn it on a thread somewhere. Maybe you want
/// to stick to the main thread.
pub fn main() {
    let cli = TheCliApp::from_args();
    let verbosity: Verbosity = cli.verbose.into();
    let level: LevelFilter = verbosity.into();

    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let stdout_layer = tracing_subscriber::fmt::layer().with_writer(non_blocking);
    let _registry = tracing_subscriber::registry()
        .with(level)
        .with(stdout_layer)
        .init();

    let _taw_span = tracing::info_span!("global").entered();
    tracing::info!("taw!");
    tracing::trace!("{:?}", cli);
    tracing::trace!("verbose level: {:?}", level);
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
