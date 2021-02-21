//! Calendars are witnesses to the passage of time. On a wall, on a screen, on paper, or a good old
//! chalkboard, and we write little notes on certain dates. Things that we want to remember, but most
//! importantly, things that we don't wish to forget.
//!
//! Calendars are simple. Let's be the same way.
//!
//! At any given time, there can be many calendars around The Shop.
use chrono::prelude::{Date, Datelike, TimeZone, Utc};
use std::collections::HashMap;

/// TheCalendar
#[derive(Default, Debug, PartialEq)]
pub struct TheCalendar<T> {
    /// We are a bit shy of showing readers that we are
    /// using generics without actually knowing what we are
    /// doing _YET_, so, we hide calendar_posts and expose some methods
    /// below.
    calendar_posts: AListOfPosts<T>,
}

impl<T> TheCalendar<T> {
    /// Tell how many calendar posts this calendar has.
    pub fn total_posts_count(&self) -> usize {
        self.calendar_posts.len()
    }
}

/// `AListOfPosts` is a wrapper for  `Vec<T>`, where the generic type `T` is anything which
/// needs to derive the same traits as this type.
#[derive(Clone, Default, Debug, PartialEq)]
struct AListOfPosts<T> {
    pub list_of_posts: Vec<T>,
}

impl<T> AListOfPosts<T> {
    fn len(&self) -> usize {
        self.list_of_posts.len()
    }
}

/// CalendarPost
#[derive(Debug, PartialEq)]
pub struct CalendarPost {
    pub date: Date<Utc>,
    pub text: String,
}

impl CalendarPost {
    pub fn new(text: &str) -> Self {
        CalendarPost {
            date: Utc::now().date(),
            text: text.to_string(),
        }
    }

    pub fn new_with_date(text: &str, date: Date<Utc>) -> Self {
        CalendarPost {
            date,
            text: text.to_string(),
        }
    }

    pub fn new_with_ymd(text: &str, year: i32, month: u32, day: u32) -> Self {
        CalendarPost {
            date: Utc.ymd(year, month, day),
            text: text.to_string(),
        }
    }
}

impl Default for CalendarPost {
    fn default() -> Self {
        CalendarPost::new("")
    }
}

impl std::fmt::Display for CalendarPost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (year, month, day) = (self.date.year(), self.date.month(), self.date.day());
        write!(f, "{}-{:0>2}-{:0>2}: {}", year, month, day, self.text)
    }
}

/// A type to hold uniquely named calendars. Sooner or later, we'll
/// bury the hashmap inside, for now, we're prototyping.
pub type ShopCalendars = HashMap<String, TheCalendar<CalendarPost>>;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    const YEAR: i32 = 2000;
    const MONTH: u32 = 3;
    const DAY: u32 = 8;
    const POST_TEXT: &str = "Don't forget!";
    const POST_EXPECTED_DISPLAY: &str = "2000-03-08: Don't forget!";

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

    #[test]
    fn a_new_calendar_post_has_todays_date() {
        let post = CalendarPost::new("Remember to improve the calendar");
        assert_eq!(post.date, Utc::now().date());
    }

    #[test]
    fn a_new_calendar_post_can_set_its_date() {
        let some_date = Utc.ymd(YEAR, MONTH, DAY);
        let post = CalendarPost::new_with_date(POST_TEXT, some_date);
        assert_eq!(post.date, some_date);
    }

    #[test]
    fn a_new_calendar_post_can_be_created_with_year_month_day() {
        let post = CalendarPost::new_with_ymd(POST_TEXT, YEAR, MONTH, DAY);
        assert_eq!(post.date.year(), YEAR);
        assert_eq!(post.date.month(), MONTH);
        assert_eq!(post.date.day(), DAY);
    }

    #[test]
    fn a_calendar_post_implements_fmt_display() {
        // for our constant post info, we compare with the
        // constant that has the expected display output
        let post = CalendarPost::new_with_ymd(POST_TEXT, YEAR, MONTH, DAY);
        assert_eq!(format!("{}", post), POST_EXPECTED_DISPLAY);

        // for a date that is 5 days from today, we build the expected
        // output by hand
        let five_days_from_now = Utc::now().date() + Duration::days(5);
        let (year, month, day) = (
            five_days_from_now.year(),
            five_days_from_now.month(),
            five_days_from_now.day(),
        );
        let post = CalendarPost::new_with_date("Calendar continues", five_days_from_now);
        let expected = format!("{}-{:0>2}-{:0>2}: {}", year, month, day, post.text);
        assert_eq!(format!("{}", post), expected);
    }

    #[test]
    fn a_calendar_with_two_posts() {
        let five_days_from_now = Utc::now().date() + Duration::days(5);

        let list_of_posts = vec![
            CalendarPost::new("Calendar begins"),
            CalendarPost::new_with_date("Calendar continues", five_days_from_now),
        ];
        let calendar = TheCalendar {
            calendar_posts: AListOfPosts { list_of_posts },
        };
        assert_eq!(calendar.total_posts_count(), 2);
    }
}
