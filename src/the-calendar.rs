//! Calendars are witnesses to the passage of time. On a wall, on a screen, on paper, or a good old
//! chalkboard, and we write little notes on certain dates. Things that we want to remember, but most
//! importantly, things that we don't wish to forget.
//!
//! Calendars are simple. Let's be the same way.
//!
//! At any given time, there can be many calendars around The Shop.
use chrono::prelude::{Date, Utc};

/// TheCalendar
#[derive(Default, Debug, PartialEq)]
pub struct TheCalendar<T> {
    all_posts: AListOfPosts<T>,
}

impl<T> TheCalendar<T> {
    pub fn total_posts_count(&self) -> usize {
        self.all_posts.len()
    }
}

/// AListOfPosts
#[derive(Clone, Default, Debug, PartialEq)]
struct AListOfPosts<T> {
    inner: Vec<T>,
}

impl<T> AListOfPosts<T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// CalendarPost
#[derive(Debug, PartialEq)]
pub struct CalendarPost {
    pub date: Date<Utc>,
}

impl Default for CalendarPost {
    fn default() -> Self {
        CalendarPost {
            date: Utc::now().date(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_calendar_should_implement_default_n_debug_n_partial_eq() {
        assert_eq!(TheCalendar::<()>::default(), TheCalendar::<()>::default());
    }

    #[test]
    fn the_calendar_has_empty_count_of_events_by_default() {
        let calendar: TheCalendar<()> = Default::default();
        assert_eq!(calendar.total_posts_count(), 0);
    }

    #[test]
    fn a_calendar_post_should_implement_default_n_debug_n_partial_eq() {
        assert_eq!(CalendarPost::default(), CalendarPost::default());
    }
}
