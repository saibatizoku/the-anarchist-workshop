//! # The Anarchist's Workshop
//!
//! This is a a bunch of code put together in order to run things at workshops.
//!
//! As in, places where people work. Workshops.
//!
//! The end goal is to have some Free Open-Source Software which can be installed
//! relatively easily on a computer, and be helpful from the start when running.
//!
//! ## Using
//!
//! To use as a dependency, add the crate from source at github (for  now):
//! ```toml
//! [dependencies]
//! the-archist-workshop = { git = "https://github.com/saibatizoku/the-anarchist-workshop.git" }
//! ```
//!
//! To clone the git repository from source at github:
//! ```sh
//! git clone https://github.com/saibatizoku/the-anarchist-workshop.git
//! cd the-anarchist-workshop
//! ```
//!
//! To push code to the project, fork the project from github, add a reasonably named branch,
//! create a pull request. Simplicity and one-thing-at-a-time-edness is greatly appreciated. Play
//! nice, do your thing.
//!
//! ## Motivation And Anarchical Theming
//!
//! The title is inspired by the work of one Christopher M. Schwarz, well-known woodworker,
//! bookwriter, publisher, etc. By reading his book about building his design for "The Anarchist Workbench".
//! Yes, from reading his book. It's good, and it's licensed with Creative Commons over at the
//! [lostartpress.com](https://lostartpress.com), it's worth taking the time to look for it there, or somewhere in the
//! [blog.lostartpress.com](https://blog.lostartpress.com).
#[path = "the-calendar.rs"]
pub mod the_calendar;

use crate::the_calendar::{CalendarPost, TheCalendar};
use std::collections::HashMap;

/// TheShop
#[derive(Default, Debug, PartialEq)]
pub struct TheShop {
    calendars: HashMap<String, TheCalendar<CalendarPost>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shop_should_implement_default_n_debug_n_partial_eq() {
        assert_eq!(TheShop::default(), TheShop::default());
    }
}
