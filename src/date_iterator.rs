
use chrono::{DateTime, TimeZone};

use calendar_duration::{CalendarDuration, checked_add};

/// Iterator as returned by `date_iterator_from`
///
/// TODO: How to make this generic over Datelike?
pub struct OpenEndedDateIterator<Tz: TimeZone> {
    from: DateTime<Tz>,
    duration: CalendarDuration,
    iterations: i32,
}

/// returns an open ended `Iterator`, that will first yield `dt`
///
/// TODO: How to make this generic over Datelike?
pub fn date_iterator_from<Tz: TimeZone, D: Into<CalendarDuration>>(dt: DateTime<Tz>, duration: D) -> OpenEndedDateIterator<Tz> {
    OpenEndedDateIterator {
        from: dt,
        duration: duration.into(),
        iterations: 0,
    }
}

impl<Tz: TimeZone> Iterator for OpenEndedDateIterator<Tz> {
    type Item = DateTime<Tz>;

    fn next(&mut self) -> Option<Self::Item> {
        //TODO: This should be checked_mul as well but we'll wait for a better `Duration` type for that...
        let next = checked_add(&self.from, &self.duration * self.iterations);
        self.iterations += 1;
        next
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use chrono::Utc;

    use super::*;

    #[test]
    pub fn test_date_iterator_from() {
        let input = "1996-12-25T16:39:57.123Z";
        let dt = DateTime::<Utc>::from_str(input).unwrap();
        assert_eq!(input, format!("{:?}", dt));

        let duration = CalendarDuration::years(3) +
            CalendarDuration::months(1) +
            CalendarDuration::days(2) +
            CalendarDuration::minutes(4);

        let iter = date_iterator_from(dt, duration);
        let expected = vec![
            "1996-12-25T16:39:57.123Z",
            "2000-01-27T16:43:57.123Z",
            "2003-02-28T16:47:57.123Z",
            "2006-03-31T16:51:57.123Z",
            ];

        assert_eq!(expected, iter.take(4).map(|d| format!("{:?}", d)).collect::<Vec<_>>());
    }

}
