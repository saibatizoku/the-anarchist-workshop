//! Calendars are witnesses to the passage of time. On a wall, on a screen, on paper, or a good old
//! chalkboard, and we write little notes on certain dates. Things that we want to remember, but most
//! importantly, things that we don't wish to forget.
//!
//! Calendars are simple. Let's be the same way.
//!
//! At any given time, there can be many calendars around The Shop.
use std::collections::HashMap;
use time::{Date, Month, OffsetDateTime};

/// The Calendar
///
#[derive(Clone, Default, Debug, PartialEq)]
pub struct TheCalendar<KEY: PartialEq + Eq + std::hash::Hash, THING> {
    pub things: HashMap<KEY, THING>,
}

impl<K, T> TheCalendar<K, T>
where
    K: PartialEq + Eq + std::hash::Hash,
{
    /// Tell how many calendar things this calendar has.
    pub fn total_things(&self) -> usize {
        self.things.len()
    }
}

/// A Calendar Post can be used as a `THING` that goes into a Calendar.
#[derive(Clone, Debug, PartialEq)]
pub struct CalendarPost {
    /// The date this is posted on.
    pub date: Date,
    /// The text that is posted.
    pub text: String,
}

impl CalendarPost {
    /// Create a new calendar post with today's date.
    pub fn new(text: &str) -> Self {
        CalendarPost::new_with_date(text, OffsetDateTime::now_utc().date())
    }

    /// Create a new calendar post with some date.
    pub fn new_with_date(text: &str, date: Date) -> Self {
        CalendarPost {
            date,
            text: text.to_string(),
        }
    }

    /// Create a new calendar post with some `year`, `month`, and `day`.
    pub fn new_with_ymd(text: &str, year: i32, month: Month, day: u8) -> Self {
        CalendarPost::new_with_date(text, Date::from_calendar_date(year, month, day).unwrap())
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
        write!(f, "{}-{:0>2}-{:0>2}: {}", year, month as u8, day, self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Duration;

    const YEAR: i32 = 2000;
    const MONTH: Month = Month::March;
    const DAY: u8 = 8;
    const POST_TEXT: &str = "Don't forget!";
    const POST_EXPECTED_DISPLAY: &str = "2000-03-08: Don't forget!";

    #[test]
    fn the_calendar_should_implement_default_n_debug_n_partial_eq() {
        assert_eq!(
            TheCalendar::<(), ()>::default(),
            TheCalendar::<(), ()>::default()
        );
    }

    #[test]
    fn the_calendar_has_empty_count_of_things_by_default() {
        let calendar: TheCalendar<(), ()> = Default::default();
        assert_eq!(calendar.total_things(), 0);
    }

    #[test]
    fn a_calendar_post_should_implement_default_n_debug_n_partial_eq() {
        assert_eq!(CalendarPost::default(), CalendarPost::default());
    }

    #[test]
    fn a_new_calendar_post_has_todays_date() {
        let post = CalendarPost::new("Remember to improve the calendar");
        assert_eq!(post.date, OffsetDateTime::now_utc().date());
    }

    #[test]
    fn a_new_calendar_post_can_set_its_date() {
        let some_date = Date::from_calendar_date(YEAR, MONTH, DAY).unwrap();
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
        let five_days_from_now = OffsetDateTime::now_utc().date() + Duration::days(5);
        let (year, month, day) = (
            five_days_from_now.year(),
            five_days_from_now.month(),
            five_days_from_now.day(),
        );
        let post = CalendarPost::new_with_date("Calendar continues", five_days_from_now);
        let expected = format!("{}-{:0>2}-{:0>2}: {}", year, month as u8, day, post.text);
        assert_eq!(format!("{}", post), expected);
    }

    #[test]
    fn a_hash_map_of_calendar_posts() {
        const POST_TITLE: &str = "2021 Stuff";
        let mut collection: TheCalendar<String, CalendarPost> = TheCalendar::default();
        // When someting is new, `insert` returns None
        assert_eq!(
            collection
                .things
                .insert(POST_TITLE.to_string(), Default::default()),
            None
        );
        // When someting already exists, `insert` returns `Some(replaced)`
        assert!(collection
            .things
            .insert(POST_TITLE.to_string(), Default::default())
            .is_some());

        assert!(collection.things.contains_key(POST_TITLE));
    }

    #[test]
    fn a_hash_map_of_dates_for_keys_and_a_vec_of_strings_for_values() {
        // we need to our KEY type to impl Default
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        struct DateKey(Date);
        impl Default for DateKey {
            fn default() -> Self {
                DateKey(OffsetDateTime::now_utc().date())
            }
        }
        let today = DateKey(OffsetDateTime::now_utc().date());
        let five_days_from_now = DateKey(OffsetDateTime::now_utc().date() + Duration::days(5));

        let mut collection: TheCalendar<DateKey, Vec<String>> = TheCalendar::default();
        // When someting is new, `insert` returns None
        assert_eq!(collection.things.insert(today, Default::default()), None);
        // When someting already exists, `insert` returns `Some(replaced)`
        assert!(collection
            .things
            .insert(five_days_from_now, Default::default())
            .is_none());

        assert!(collection.things.contains_key(&today));
        assert!(collection.things.contains_key(&five_days_from_now));
    }
}
