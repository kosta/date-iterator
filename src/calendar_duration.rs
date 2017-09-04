use std::cmp::min;
use std::ops::{Add, Div, Mul, Neg, Sub};

use chrono::{Datelike, DateTime, Duration as OldDuration, NaiveDate, NaiveDateTime, TimeZone};

use super::last_day_of_month_0;

/// A `Duration` type that is aware of months and years.
///
/// It tries to mimic chrono's `Duration` type
/// (used as `OldDuration` as seems to be chrono's style) as much as possible.
///
/// Note: Adding months can be a bit weird, because they have varying length. While
/// you can always add a month to e.g. 2017-05-01, you cannot "sanely" add a month
/// to e.g. 2017-01-30, as the 30th February doesn't exist. Still, such operations
/// can make sense, e.g. in the context of a date iterator. This struct tries to make
/// sense of such operations by giving the next best date in a month, e.g. returning
/// 2017-02-28 in the example above.
///
/// `CalendarDuration` is internally represented by a `Duration` (in seconds, nanoseconds),
/// a number of months, and the number of years.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalendarDuration {
    duration: OldDuration,
    months: i32,
    years: i32,
}

impl CalendarDuration {
    pub fn years(years: i32) -> CalendarDuration {
        CalendarDuration {
            duration: OldDuration::zero(),
            months: 0,
            years: years,
        }
    }

    pub fn months(months: i32) -> CalendarDuration {
        CalendarDuration {
            duration: OldDuration::zero(),
            months: months,
            years: 0,
        }
    }

    pub fn weeks(weeks: i64) -> CalendarDuration {
        CalendarDuration {
            duration: OldDuration::weeks(weeks),
            months: 0,
            years: 0,
        }
    }

    pub fn days(days: i64) -> CalendarDuration {
        CalendarDuration {
            duration: OldDuration::days(days),
            months: 0,
            years: 0,
        }
    }

    pub fn hours(hours: i64) -> CalendarDuration {
        CalendarDuration {
            duration: OldDuration::hours(hours),
            months: 0,
            years: 0,
        }
    }

    pub fn minutes(minutes: i64) -> CalendarDuration {
        CalendarDuration {
            duration: OldDuration::minutes(minutes),
            months: 0,
            years: 0,
        }
    }

    pub fn seconds(seconds: i64) -> CalendarDuration {
        CalendarDuration {
            duration: OldDuration::seconds(seconds),
            months: 0,
            years: 0,
        }
    }

    pub fn milliseconds(milliseconds: i64) -> CalendarDuration {
        CalendarDuration {
            duration: OldDuration::milliseconds(milliseconds),
            months: 0,
            years: 0,
        }
    }

    pub fn microseconds(microseconds: i64) -> CalendarDuration {
        CalendarDuration {
            duration: OldDuration::microseconds(microseconds),
            months: 0,
            years: 0,
        }
    }

    pub fn nanoseconds(nanoseconds: i64) -> CalendarDuration {
        CalendarDuration {
            duration: OldDuration::nanoseconds(nanoseconds),
            months: 0,
            years: 0,
        }
    }

    pub fn zero() -> CalendarDuration {
        CalendarDuration {
            duration: OldDuration::zero(),
            months: 0,
            years: 0,
        }
    }

    pub fn duration_part(&self) -> &OldDuration {
        &self.duration
    }

    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        Some(CalendarDuration {
                 duration: try_opt!(self.duration.checked_add(&other.duration)),
                 months: try_opt!(self.months.checked_add(other.months)),
                 years: try_opt!(self.years.checked_add(other.years)),
             })
    }

    //TODO: Implement checked_mul once there is a new chrono::Duration type
    // pub fn checked_mul(&self, factor: i32) -> Option<CalendarDuration> {
    //     Some(CalendarDuration {
    //         duration: try_opt!(self.duration.checked_mut(factor)),
    //         months: try_opt!(self.months.checked_mul(factor)),
    //         years: try_opt!(self.years.checked_mul(factor)),
    //     })
    // }
}

pub fn add_years<Tz: TimeZone>(dt: &DateTime<Tz>, years: i32) -> Option<DateTime<Tz>> {
    dt.with_year(try_opt!(dt.year().checked_add(years)))
}

pub fn add_months_naive_date(date: &NaiveDate, months: i32) -> Option<NaiveDate> {
    let next_month_0 = try_opt!((date.month0() as i64).checked_add(months as i64));
    let additional_years = next_month_0 / 12;
    let next_month_0 = (next_month_0 % 12) as u32;
    let additional_years = if additional_years >= (i32::max_value() as i64) {
        return None;
    } else {
        additional_years as i32
    };
    let next_year = try_opt!(date.year().checked_add(additional_years));
    let next_day = min(date.day(), last_day_of_month_0(next_year, next_month_0));
    NaiveDate::from_ymd_opt(next_year, next_month_0 + 1, next_day)
}

pub fn add_months_naive_dt(dt: &NaiveDateTime, months: i32) -> Option<NaiveDateTime> {
    add_months_naive_date(&dt.date(), months).map(|date| NaiveDateTime::new(date, dt.time()))
}

pub fn add_months_dt<Tz: TimeZone>(dt: &DateTime<Tz>, months: i32) -> Option<DateTime<Tz>> {
    add_months_naive_dt(&dt.naive_utc(), months).map(|naive| {
                                                         DateTime::from_utc(naive,
                                                                            dt.offset().clone())
                                                     })
}

/// Add the `CalendarDuration` to given dt, returning None on overflow.
/// Note that adding e.g. one month to January 30th will return February 28th.
/// See `CalendarDuration` for more details.
///
/// As we cannot extend `DateTime` here, simply use a free function.
///
/// TODO: In which order should these operations be applied? Here we first add the
/// duration in seconds, then the years, then the months which seems a bit arbitrary.
/// python's datutil first add years, then months, then seconds: http://labix.org/python-dateutil#head-72c4689ec5608067d118b9143cef6bdffb6dad4e
pub fn checked_add<Tz: TimeZone>(dt: &DateTime<Tz>,
                                 duration: &CalendarDuration)
                                 -> Option<DateTime<Tz>> {
    dt.clone()
        .checked_add_signed(duration.duration)
        .and_then(|dt| add_years(&dt, duration.years))
        .and_then(|dt| add_months_dt(&dt, duration.months))
}

/// As this crate does not define `DateTime`, it cannot implement `Add`. Hence this free function.
pub fn add<Tz: TimeZone>(dt: &DateTime<Tz>, duration: &CalendarDuration) -> DateTime<Tz> {
    checked_add(dt, duration).expect("add(DateTime, CalendarDuration) overflowed")
}

impl From<OldDuration> for CalendarDuration {
    fn from(duration: OldDuration) -> Self {
        CalendarDuration {
            duration: duration,
            months: 0,
            years: 0,
        }
    }
}

impl Add for CalendarDuration {
    type Output = CalendarDuration;

    fn add(self, rhs: CalendarDuration) -> CalendarDuration {
        CalendarDuration {
            duration: self.duration + rhs.duration,
            months: self.months + rhs.months,
            years: self.years + rhs.years,
        }
    }
}

impl Sub for CalendarDuration {
    type Output = CalendarDuration;

    fn sub(self, rhs: CalendarDuration) -> CalendarDuration {
        CalendarDuration {
            duration: self.duration - rhs.duration,
            months: self.months - rhs.months,
            years: self.years - rhs.years,
        }
    }
}

// TODO: Does it make sense to implement for &'a?
impl<'a> Mul<i32> for &'a CalendarDuration {
    type Output = CalendarDuration;

    fn mul(self, rhs: i32) -> CalendarDuration {
        CalendarDuration {
            duration: self.duration * rhs,
            months: self.months * rhs,
            years: self.years * rhs,
        }
    }
}

impl Div<i32> for CalendarDuration {
    type Output = CalendarDuration;

    fn div(self, rhs: i32) -> CalendarDuration {
        CalendarDuration {
            duration: self.duration / rhs,
            months: self.months / rhs,
            years: self.years / rhs,
        }
    }
}

impl Neg for CalendarDuration {
    type Output = CalendarDuration;

    fn neg(self) -> CalendarDuration {
        CalendarDuration {
            duration: -self.duration,
            months: -self.months,
            years: -self.years,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use chrono::Utc;

    use super::*;

    #[test]
    pub fn add_simple() {
        let input = "1996-12-19T16:39:57.123Z";
        let dt = DateTime::<Utc>::from_str(input).unwrap();
        assert_eq!(input, format!("{:?}", dt));

        let duration = CalendarDuration::days(3) + CalendarDuration::hours(1) +
                       CalendarDuration::months(5) +
                       CalendarDuration::years(1);

        let result = add(&dt, &duration);
        assert_eq!("1998-05-22T17:39:57.123Z", format!("{:?}", result));
    }

    #[test]
    #[should_panic]
    pub fn add_overflow() {
        let input = "1996-12-19T16:39:57.123Z";
        let dt = DateTime::<Utc>::from_str(input).unwrap();
        assert_eq!(input, format!("{:?}", dt));

        let duration = CalendarDuration::years(300_000);

        add(&dt, &duration);
    }

    #[test]
    pub fn add_overflow_checked() {
        let input = "1996-12-19T16:39:57.123Z";
        let dt = DateTime::<Utc>::from_str(input).unwrap();
        assert_eq!(input, format!("{:?}", dt));

        let duration = CalendarDuration::years(300_000);

        assert_eq!(None, checked_add(&dt, &duration));
    }

    #[test]
    pub fn add_adjusted() {
        let input = "1996-12-31T16:39:57.123Z";
        let dt = DateTime::<Utc>::from_str(input).unwrap();
        assert_eq!(input, format!("{:?}", dt));

        let duration = CalendarDuration::months(2);
        let result = add(&dt, &duration);
        //Note how february doesn't have a 31st day...
        assert_eq!("1997-02-28T16:39:57.123Z", format!("{:?}", result));

        let duration = CalendarDuration::months(4);
        let result = add(&dt, &duration);
        //...and neither has april
        assert_eq!("1997-04-30T16:39:57.123Z", format!("{:?}", result));

        let duration = CalendarDuration::months(5);
        let result = add(&dt, &duration);
        //But May is ok
        assert_eq!("1997-05-31T16:39:57.123Z", format!("{:?}", result));
    }
}
