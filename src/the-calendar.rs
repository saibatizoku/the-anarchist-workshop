//! Calendars are witnesses to the passage of time. On a wall, on a screen, on paper, or a good old
//! chalkboard, and we write little notes on certain dates. Things that we want to remember, but most
//! importantly, things that we don't wish to forget.
//!
//! Calendars are simple. Let's be the same way.
//!
//! At any given time, there can be many calendars around The Shop.
use chrono::prelude::{Date, Datelike, TimeZone, Utc};
use std::collections::HashMap;

/// The Calendar
///
/// A type that holds a list inside a `Vec<THING>`, where
/// `THING` is anything that compiles (i.e. derives `Clone`, `Default`, etc...)
#[derive(Clone, Default, Debug, PartialEq)]
pub struct TheCalendar<THING> {
    pub calendar_things: AListOfThings<THING>,
}

impl<T> TheCalendar<T> {
    /// Tell how many calendar things this calendar has.
    pub fn total_things(&self) -> usize {
        self.calendar_things.len()
    }
}

/// `AListOfThings` is a wrapper for  `Vec<T>`, where the generic type `T` is anything which
/// needs to derive the same traits as this type.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct AListOfThings<T> {
    pub list_of_things: Vec<T>,
}

impl<T> AListOfThings<T> {
    pub fn len(&self) -> usize {
        self.list_of_things.len()
    }
}

/// A Calendar Post can be used as a `THING` that goes into a Calendar.
#[derive(Clone, Debug, PartialEq)]
pub struct CalendarPost {
    /// The date this is posted on.
    pub date: Date<Utc>,
    /// The text that is posted.
    pub text: String,
}

impl CalendarPost {
    /// Create a new calendar post with today's date.
    pub fn new(text: &str) -> Self {
        CalendarPost::new_with_date(text, Utc::now().date())
    }

    /// Create a new calendar post with some date.
    pub fn new_with_date(text: &str, date: Date<Utc>) -> Self {
        CalendarPost {
            date,
            text: text.to_string(),
        }
    }

    /// Create a new calendar post with some `year`, `month`, and `day`.
    pub fn new_with_ymd(text: &str, year: i32, month: u32, day: u32) -> Self {
        CalendarPost::new_with_date(text, Utc.ymd(year, month, day))
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
#[derive(Clone, Default, Debug, PartialEq)]
pub struct CalendarCollection<T: Clone> {
    pub calendars: HashMap<String, TheCalendar<T>>,
}

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
    fn the_calendar_has_empty_count_of_things_by_default() {
        let calendar: TheCalendar<()> = Default::default();
        assert_eq!(calendar.total_things(), 0);
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
        let calendar = TheCalendar {
            calendar_things: AListOfThings {
                list_of_things: vec![
                    CalendarPost::new("Calendar begins"),
                    CalendarPost::new_with_date(
                        "Calendar continues",
                        Utc::now().date() + Duration::days(5),
                    ),
                ],
            },
        };
        assert_eq!(calendar.total_things(), 2);
        assert_eq!(
            calendar.calendar_things.list_of_things[0].text,
            "Calendar begins"
        );
        assert_eq!(
            calendar.calendar_things.list_of_things[1].text,
            "Calendar continues"
        );
    }

    #[test]
    fn a_calendar_collection_is_a_hash_map_() {
        const CALENDAR_NAME: &str = "2021 Stuff";
        let mut collection: CalendarCollection<CalendarPost> = CalendarCollection::default();
        // When someting is new, `insert` returns None
        assert_eq!(
            collection
                .calendars
                .insert(CALENDAR_NAME.to_string(), Default::default()),
            None
        );
        // When someting already exists, `insert` returns `Some(replaced)`
        assert!(collection
            .calendars
            .insert(CALENDAR_NAME.to_string(), Default::default())
            .is_some());

        assert!(collection.calendars.contains_key(CALENDAR_NAME));
    }
}
