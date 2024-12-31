//! Convert between nanoseconds, seconds, days, years... in `const`.
//!
//! The `timelane` crate defines lanes of time where each lane works at a
//! different scale: yearly, monthly, daily, etc. Each lane has sequentially
//! numbered marks. The crate implements scaler functions to convert a mark in
//! one lane into a mark in another lane.
//!
//! For instance, mark 1 in the month lane is mark 1 is the day lane, mark 2 in
//! the month lane is mark 32 in the day lane.
//!
//! [`Mark`] values are signed integers.
//!
//! [`Scaler`] functions can be composed to convert [`Mark`] from non-adjacent
//! lanes, like converting a year mark into a second mark.
//!
//! A mark from a small scale lane can also be converted to a mark in a larger
//! scale lane, like second to year, but you need to chose between rounding
//! down or rounding up. Scaling a second mark to a year mark and then back to
//! a second mark will give the beginning or the end of the year, depending on
//! the rounding mode.
//!
//! All [`Scaler`] functions are `const` functions. Leap seconds are statically
//! defined.
//!
//! This library will return incorrect results if the International Earth
//! Rotation and Reference Systems Service declares a new leap second.
//!
//! However, the last leap second was in 2017 and the General Conference on
//! Weights and Measures resolved to eliminate leap seconds by or before 2035.
//!
//!  - Years before 1AD use the astronomical year numbering: the year 1BC is
//!     mark `0`, 2BC is `-1`...
//!  - Leap years follow the proleptic gregorian calendar, and are defined even
//!     for negative years: 1BC is a leap year.
//!  - Because of leap seconds, some minutes will contain 61 seconds.
//!  - Before using this library, make sure you actually want to work with UTC.
//!    Many systems use GPS or TAI, which do not include leap seconds.

/// A specific point on a time lane.
pub type Mark = isize;
/// A function to convert a [`Mark`] from one lane to another.
pub type Scaler = fn(mark: Mark) -> Mark;

pub mod subsecond;

/// This year is the one where the first second of January 1st is the [`Mark`] 0.
pub const EPOCH_YEAR: Mark = 2000;

const ZMONTH_STARTS: [Mark; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

const ZMONTH_STARTS_LEAP_YEAR: [Mark; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

/// All known leap seconds, as minute [`Mark`].
pub const LEAP_SECONDS_MARKS: [Mark; 27] = [
    year_month_to_minute(1972, 7),
    year_month_to_minute(1973, 1),
    year_month_to_minute(1974, 1),
    year_month_to_minute(1975, 1),
    year_month_to_minute(1976, 1),
    year_month_to_minute(1977, 1),
    year_month_to_minute(1978, 1),
    year_month_to_minute(1979, 1),
    year_month_to_minute(1980, 1),
    year_month_to_minute(1981, 7),
    year_month_to_minute(1982, 7),
    year_month_to_minute(1983, 7),
    year_month_to_minute(1985, 7),
    year_month_to_minute(1988, 1),
    year_month_to_minute(1990, 1),
    year_month_to_minute(1991, 1),
    year_month_to_minute(1992, 7),
    year_month_to_minute(1993, 7),
    year_month_to_minute(1994, 7),
    year_month_to_minute(1996, 1),
    year_month_to_minute(1997, 7),
    year_month_to_minute(1999, 1),
    year_month_to_minute(2006, 1),
    year_month_to_minute(2009, 1),
    year_month_to_minute(2012, 7),
    year_month_to_minute(2015, 7),
    year_month_to_minute(2017, 1),
];

const fn year_month_to_minute(year: Mark, month: Mark) -> Mark {
    let zmonth = month - 1;
    hour_to_minute(day_to_hour(month_to_day(zmonth + year_to_month(year))))
}

/// Converts a year [`Mark`] to a month [`Mark`].
///
/// Year [`EPOCH_YEAR`] is month one.
///
/// # Examples
/// ```
/// use timelane::year_to_month;
/// assert_eq!(year_to_month(1999), -11);
/// assert_eq!(year_to_month(2000), 1);
/// assert_eq!(year_to_month(2001), 13);
/// use timelane::Mark;
/// assert_eq!(year_to_month(-768614336404562650), Mark::MIN + 9);
/// assert_eq!(year_to_month(768614336404566650), Mark::MAX - 6);
/// ```
pub const fn year_to_month(year: Mark) -> Mark {
    let zyear = year - EPOCH_YEAR;
    zyear * 12 + 1
}

/// Converts a month [`Mark`] to a day [`Mark`].
///
/// Month one is day one.
///
/// # Examples
/// ```
/// use timelane::month_to_day;
/// assert_eq!(month_to_day(0), -30);
/// assert_eq!(month_to_day(1), 1);
/// assert_eq!(month_to_day(2), 32);
/// use timelane::Mark;
/// assert_eq!(month_to_day(-303032819133198653), Mark::MIN + 26);
/// assert_eq!(month_to_day(303032819133198655), Mark::MAX - 25);
/// ```
pub const fn month_to_day(month: Mark) -> Mark {
    let zmonth = month - 1;
    // We make sure the month is year is actually positive (the modulo operator alone is not enough)
    let zmonth_in_year = zmonth % 12 + if zmonth % 12 < 0 { 12 } else { 0 };
    let zyear = divide_towards_negative_infinity(zmonth, 12);
    // If we're after the month 2, we want the number of leap days including the current year
    let zleap_year = zyear + if zmonth_in_year >= 2 { 1 } else { 0 };
    // Then we rebuild the day using the number of years, the leap days, the month lengths
    // and the 1 offset because we start at day 1
    let leap_days = leap_days_before_year(zleap_year + EPOCH_YEAR);
    let base_leap_days = leap_days_before_year(EPOCH_YEAR);
    zyear * 365 + ZMONTH_STARTS[zmonth_in_year as usize] - base_leap_days + leap_days + 1
}

/// Converts a day [`Mark`] to an hour [`Mark`].
///
/// Day one is hour zero.
///
/// # Examples
/// ```
/// use timelane::day_to_hour;
/// assert_eq!(day_to_hour(0), -24);
/// assert_eq!(day_to_hour(1), 0);
/// assert_eq!(day_to_hour(2), 24);
/// use timelane::Mark;
/// assert_eq!(day_to_hour(-384307168202282324), Mark::MIN + 8);
/// assert_eq!(day_to_hour(384307168202282326), Mark::MAX - 7);
/// ```
pub const fn day_to_hour(day: Mark) -> Mark {
    let zday = day - 1;
    zday * 24
}

/// Converts an hour [`Mark`] to a minute [`Mark`].
///
/// # Examples
/// ```
/// use timelane::hour_to_minute;
/// assert_eq!(hour_to_minute(-1), -60);
/// assert_eq!(hour_to_minute(0), 0);
/// assert_eq!(hour_to_minute(1), 60);
/// use timelane::Mark;
/// assert_eq!(hour_to_minute(-153722867280912930), Mark::MIN + 8);
/// assert_eq!(hour_to_minute(153722867280912930), Mark::MAX - 7);
/// ```
pub const fn hour_to_minute(hour: Mark) -> Mark {
    hour * 60
}

/// Converts a minute [`Mark`] to a second [`Mark`].
///
///  This takes in account leap seconds.
///
/// # Examples
/// ```
/// use timelane::minute_to_second;
/// assert_eq!(minute_to_second(-1), -60);
/// assert_eq!(minute_to_second(0), 0);
/// assert_eq!(minute_to_second(1), 60);
/// use timelane::Mark;
/// assert_eq!(minute_to_second(-153722867280912929), Mark::MIN + 46);
/// assert_eq!(minute_to_second(153722867280912930), Mark::MAX - 2);
/// ```
pub const fn minute_to_second(minute: Mark) -> Mark {
    minute * 60 + leap_seconds_before_minute(minute)
}

/// Converts a second [`Mark`] to a rounded down minute [`Mark`].
///
/// This takes in account leap seconds.
///
/// # Examples
/// ```
/// use timelane::second_to_minute;
/// assert_eq!(second_to_minute(-59), -1);
/// assert_eq!(second_to_minute(-1), -1);
/// assert_eq!(second_to_minute(0), 0);
/// assert_eq!(second_to_minute(59), 0);
/// assert_eq!(second_to_minute(60), 1);
/// use timelane::Mark;
/// assert_eq!(second_to_minute(Mark::MIN), -153722867280912930);
/// assert_eq!(second_to_minute(Mark::MAX), 153722867280912930);
/// ```
pub const fn second_to_minute(second: Mark) -> Mark {
    // The estimate is good-enough because we never have more than +/- 60 leap seconds.
    let estimate = divide_towards_negative_infinity(second, 60);
    divide_towards_negative_infinity(second - leap_seconds_before_minute(estimate), 60)
}

/// Converts a second [`Mark`] to a rounded up minute [`Mark`].
///
/// This takes in account leap seconds.
///
/// # Examples
/// ```
/// use timelane::second_to_minute_up;
/// assert_eq!(second_to_minute_up(-59), 0);
/// assert_eq!(second_to_minute_up(-1), 0);
/// assert_eq!(second_to_minute_up(0), 0);
/// assert_eq!(second_to_minute_up(1), 1);
/// assert_eq!(second_to_minute_up(59), 1);
/// assert_eq!(second_to_minute_up(60), 1);
/// use timelane::Mark;
/// assert_eq!(second_to_minute_up(Mark::MIN), -153722867280912929);
/// assert_eq!(second_to_minute_up(Mark::MAX), 153722867280912931);
/// ```
pub const fn second_to_minute_up(second: Mark) -> Mark {
    // The estimate is good-enough because we never have more than +/- 60 leap seconds.
    let estimate = divide_towards_positive_infinity(second, 60);
    divide_towards_positive_infinity(second - leap_seconds_before_minute(estimate), 60)
}

/// Converts a minute [`Mark`] to a rounded down hour [`Mark`].
///
/// # Examples
/// ```
/// use timelane::minute_to_hour;
/// assert_eq!(minute_to_hour(-59), -1);
/// assert_eq!(minute_to_hour(-1), -1);
/// assert_eq!(minute_to_hour(0), 0);
/// assert_eq!(minute_to_hour(59), 0);
/// assert_eq!(minute_to_hour(60), 1);
/// use timelane::Mark;
/// assert_eq!(minute_to_hour(Mark::MIN), -153722867280912931);
/// assert_eq!(minute_to_hour(Mark::MAX), 153722867280912930);
/// ```
pub const fn minute_to_hour(minute: Mark) -> Mark {
    divide_towards_negative_infinity(minute, 60)
}

/// Converts a minute [`Mark`] to a rounded up hour [`Mark`].
///
/// # Examples
/// ```
/// use timelane::minute_to_hour_up;
/// assert_eq!(minute_to_hour_up(-59), 0);
/// assert_eq!(minute_to_hour_up(-1), 0);
/// assert_eq!(minute_to_hour_up(0), 0);
/// assert_eq!(minute_to_hour_up(59), 1);
/// assert_eq!(minute_to_hour_up(60), 1);
/// use timelane::Mark;
/// assert_eq!(minute_to_hour_up(Mark::MIN), -153722867280912930);
/// assert_eq!(minute_to_hour_up(Mark::MAX), 153722867280912931);
/// ```
pub const fn minute_to_hour_up(minute: Mark) -> Mark {
    divide_towards_positive_infinity(minute, 60)
}

/// Converts an hour [`Mark`] to a rounded down day [`Mark`].
///
/// Hour zero is day one.
///
/// # Examples
/// ```
/// use timelane::hour_to_day;
/// assert_eq!(hour_to_day(-23), 0);
/// assert_eq!(hour_to_day(-1), 0);
/// assert_eq!(hour_to_day(0), 1);
/// assert_eq!(hour_to_day(23), 1);
/// assert_eq!(hour_to_day(24), 2);
/// use timelane::Mark;
/// assert_eq!(hour_to_day(Mark::MIN), -384307168202282325);
/// assert_eq!(hour_to_day(Mark::MAX), 384307168202282326);
/// ```
pub const fn hour_to_day(hour: Mark) -> Mark {
    divide_towards_negative_infinity(hour, 24) + 1
}

/// Converts an hour [`Mark`] to a rounded up day [`Mark`].
///
/// Hour zero is day one.
///
/// # Examples
/// ```
/// use timelane::hour_to_day_up;
/// assert_eq!(hour_to_day_up(-23), 1);
/// assert_eq!(hour_to_day_up(-1), 1);
/// assert_eq!(hour_to_day_up(0), 1);
/// assert_eq!(hour_to_day_up(23), 2);
/// assert_eq!(hour_to_day_up(24), 2);
/// use timelane::Mark;
/// assert_eq!(hour_to_day_up(Mark::MIN), -384307168202282324);
/// assert_eq!(hour_to_day_up(Mark::MAX), 384307168202282327);
/// ```
pub const fn hour_to_day_up(hour: Mark) -> Mark {
    divide_towards_positive_infinity(hour, 24) + 1
}

/// Converts a day [`Mark`] to a rounded down month [`Mark`].
///
/// Day one is month one.
///
/// This takes in account leap years.
///
/// # Examples
/// ```
/// use timelane::day_to_month;
/// assert_eq!(day_to_month(32), 2, "day 32 rounds down to month 2");
/// assert_eq!(day_to_month(31), 1, "day 31 rounds down to month 1");
/// assert_eq!(day_to_month(1), 1, "day 1 rounds down to month 1");
/// assert_eq!(day_to_month(0), 0, "day 0 rounds down to month 0");
/// use timelane::Mark;
/// assert_eq!(day_to_month(Mark::MIN), -303032819133198654); // TODO: check this
/// assert_eq!(day_to_month(Mark::MAX), 303032819133198655); // TODO: check this
/// ```
pub const fn day_to_month(day: Mark) -> Mark {
    let (zyear, zdays_in_year, is_leap_year) = day_to_zyear_and_days(day);
    let month_ends = if is_leap_year {
        ZMONTH_STARTS_LEAP_YEAR
    } else {
        ZMONTH_STARTS
    };
    let mut month = 1;
    while month < month_ends.len() && zdays_in_year >= month_ends[month] {
        month += 1;
    }
    zyear * 12 + month as Mark
}

/// Converts a day [`Mark`] to a rounded up month [`Mark`].
///
/// Day one is month one.
///
/// This takes in account leap years.
///
/// # Examples
/// ```
/// use timelane::day_to_month_up;
/// assert_eq!(day_to_month_up(32), 2, "day 32 rounds up to month 2");
/// assert_eq!(day_to_month_up(31), 2, "day 31 rounds up to month 2");
/// assert_eq!(day_to_month_up(2), 2, "day 2 rounds up to month 2");
/// assert_eq!(day_to_month_up(1), 1, "day 1 rounds up to month 1");
/// assert_eq!(day_to_month_up(0), 1, "day rounds up to is month 1");
/// use timelane::Mark;
/// assert_eq!(day_to_month_up(Mark::MIN), -303032819133198653); // TODO: check this
/// assert_eq!(day_to_month_up(Mark::MAX), 303032819133198656); // TODO: check this
/// ```
pub const fn day_to_month_up(day: Mark) -> Mark {
    let (zyear, zdays_in_year, is_leap_year) = day_to_zyear_and_days(day);
    let month_ends = if is_leap_year {
        ZMONTH_STARTS_LEAP_YEAR
    } else {
        ZMONTH_STARTS
    };
    let mut month = 1;
    while month <= month_ends.len() && zdays_in_year > month_ends[month - 1] {
        month += 1;
    }
    zyear * 12 + month as Mark
}

const fn day_to_zyear_and_days(day: Mark) -> (Mark, Mark, bool) {
    if day == Mark::MIN {
        // This avoids underflow when doing day - 1 in the other branch
        let (zyear, days_in_year, is_leap_year) = day_to_zyear_and_days(day + 97 + 400 * 365);
        return (zyear - 400, days_in_year, is_leap_year);
    }
    let zday = day - 1;
    // We do a first guess of the zyear containing this zday
    let mut zyear = divide_towards_negative_infinity(
        zday - divide_towards_negative_infinity(zday, 97 + 400 * 365) * 97,
        365,
    );
    // Then we compute the day that this year would have started, taking in account leap days, it should be before the zday
    let mut leap_days =
        leap_days_before_year(zyear + EPOCH_YEAR) - leap_days_before_year(EPOCH_YEAR);
    let mut zstart_of_year = zyear * 365 + leap_days;
    // If it's not, we move back one year
    if zstart_of_year > zday {
        zyear -= 1;
        leap_days = leap_days_before_year(zyear + EPOCH_YEAR) - leap_days_before_year(EPOCH_YEAR);
        zstart_of_year = zyear * 365 + leap_days;
    }
    let is_leap_year = (leap_days_before_year(zyear + 1 + EPOCH_YEAR)
        - leap_days_before_year(EPOCH_YEAR))
        > leap_days;
    (zyear, zday - zstart_of_year, is_leap_year)
}

/// Converts a month [`Mark`] to a rounded down year [`Mark`].
///
/// Month one is year [`EPOCH_YEAR`].
/// # Examples
/// ```
/// use timelane::month_to_year;
/// assert_eq!(month_to_year(0), 1999);
/// assert_eq!(month_to_year(1), 2000);
/// assert_eq!(month_to_year(12), 2000);
/// assert_eq!(month_to_year(13), 2001);
/// use timelane::Mark;
/// assert_eq!(month_to_year(Mark::MIN), -768614336404562651); // TODO: check this
/// assert_eq!(month_to_year(Mark::MAX), 768614336404566650); // TODO: check this
/// ```
pub const fn month_to_year(month: Mark) -> Mark {
    if month == Mark::MIN {
        // This avoids underflow when doing month - 1 in the other branch
        month_to_year(month + 12) - 1
    } else {
        divide_towards_negative_infinity(month - 1, 12) + EPOCH_YEAR
    }
}

/// Converts a month [`Mark`] to a rounded up year [`Mark`].
///
/// Month one is year [`EPOCH_YEAR`].
/// Month two is rounded up to year [`EPOCH_YEAR`] plus one.
///
/// # Examples
/// ```
/// use timelane::month_to_year_up;
/// assert_eq!(month_to_year_up(0), 2000);
/// assert_eq!(month_to_year_up(1), 2000);
/// assert_eq!(month_to_year_up(2), 2001);
/// assert_eq!(month_to_year_up(12), 2001);
/// assert_eq!(month_to_year_up(13), 2001);
/// use timelane::Mark;
/// assert_eq!(month_to_year_up(Mark::MIN), -768614336404562650); // TODO: check this
/// assert_eq!(month_to_year_up(Mark::MAX), 768614336404566651); // TODO: check this
/// ```
pub const fn month_to_year_up(month: Mark) -> Mark {
    if month == Mark::MIN {
        // This avoids underflow when doing month - 1 in the other branch
        month_to_year_up(month + 12) - 1
    } else {
        divide_towards_positive_infinity(month - 1, 12) + EPOCH_YEAR
    }
}

/// Returns the number of leap days between year 1 and a given year according to the proleptic gregorian calendar.
///
/// Years before 1AD follow the ISO8601 convention: 1BC is year zero, 2BC is year -1...
///
///  Year 1BC (zero) is a leap year.
///
/// Negative years return negative number of leap days.
///
/// # Arguments
/// * `year` - A year
/// # Examples
/// ```
/// use timelane::leap_days_before_year;
/// assert_eq!(1, leap_days_before_year(5));
/// assert_eq!(0, leap_days_before_year(1));
/// assert_eq!(-1, leap_days_before_year(0));
/// assert_eq!(-2, leap_days_before_year(-4));
/// ```
pub const fn leap_days_before_year(year: Mark) -> Mark {
    if year == Mark::MIN {
        // This avoids underflow when doing year - 1 in the other branch
        leap_days_before_year(year + 400) - 97
    } else {
        let years_count = year - 1;
        divide_towards_negative_infinity(years_count, 4)
            - divide_towards_negative_infinity(years_count, 100)
            + divide_towards_negative_infinity(years_count, 400)
    }
}

/// Returns the number of leap seconds between day 1 of [`EPOCH_YEAR`] and a given year according to UTC.
///
/// The last leap second was in 2017, this crate will require an update if a new leap second is declared.
///
/// # Examples
/// ```
/// use timelane::leap_seconds_before_minute;
/// assert_eq!(leap_seconds_before_minute(0), 0);
/// // We had 5 leap seconds between EPOCH_YEAR and EPOCH_YEAR+20
/// assert_eq!(leap_seconds_before_minute(20 * 365 * 24 * 60), 5);
/// use timelane::Mark;
/// assert_eq!(leap_seconds_before_minute(Mark::MIN), -22);
/// ```
pub const fn leap_seconds_before_minute(minute: Mark) -> Mark {
    let mut leap_seconds = LEAP_SECONDS_MARKS.len();
    while leap_seconds > 0 && minute < LEAP_SECONDS_MARKS[leap_seconds - 1] {
        leap_seconds -= 1;
    }
    let mut leap_seconds_offset = LEAP_SECONDS_MARKS.len();
    while leap_seconds_offset > 0
        && year_month_to_minute(EPOCH_YEAR, 1) < LEAP_SECONDS_MARKS[leap_seconds_offset - 1]
    {
        leap_seconds_offset -= 1;
    }
    leap_seconds as Mark - leap_seconds_offset as Mark
}

/// Divides two [`Mark`], rounding towards negative infinity.
const fn divide_towards_negative_infinity(a: Mark, b: Mark) -> Mark {
    a / b - if a % b < 0 { 1 } else { 0 }
}

/// Divides two [`Mark`], rounding towards positive infinity.
const fn divide_towards_positive_infinity(a: Mark, b: Mark) -> Mark {
    a / b + if a % b > 0 { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn year_month_day_hour_minute_to_second(
        year: Mark,
        month: Mark,
        day: Mark,
        hour: Mark,
        minute: Mark,
    ) -> Mark {
        let zmonth = month - 1;
        let zday = day - 1;
        minute_to_second(
            minute
                + hour_to_minute(
                    hour + day_to_hour(zday + month_to_day(zmonth + year_to_month(year))),
                ),
        )
    }

    #[test]
    fn day_0_is_hour_minus_24() {
        assert_eq!(day_to_hour(0), -24);
    }

    #[test]
    fn day_1_is_hour_0() {
        assert_eq!(day_to_hour(1), 0);
    }

    #[test]
    fn day_2_is_hour_24() {
        assert_eq!(day_to_hour(2), 24);
    }

    #[test]
    fn hour_0_is_minute_0() {
        assert_eq!(hour_to_minute(0), 0);
    }

    #[test]
    fn hour_1_is_minute_60() {
        assert_eq!(hour_to_minute(1), 60);
    }

    #[test]
    fn month_0_is_day_minus_30() {
        assert_eq!(month_to_day(0), -30);
    }

    #[test]
    fn month_1_is_day_1() {
        assert_eq!(month_to_day(1), 1);
    }

    #[test]
    fn month_2_is_day_32() {
        assert_eq!(month_to_day(2), 32);
    }

    #[test]
    fn month_2_of_year_1_is_28_days() {
        let fourth_year_first_month = year_to_month(1);
        let start_of_february = month_to_day(fourth_year_first_month + 1);
        let end_of_february = month_to_day(fourth_year_first_month + 2);
        assert_eq!(end_of_february - start_of_february, 28);
    }

    #[test]
    fn month_2_of_year_4_is_29_days() {
        let fourth_year_first_month = year_to_month(4);
        let start_of_february = month_to_day(fourth_year_first_month + 1);
        let end_of_february = month_to_day(fourth_year_first_month + 2);
        assert_eq!(end_of_february - start_of_february, 29);
    }

    #[test]
    fn month_2_of_year_100_is_28_days() {
        let fourth_year_first_month = year_to_month(100);
        let start_of_february = month_to_day(fourth_year_first_month + 1);
        let end_of_february = month_to_day(fourth_year_first_month + 2);
        assert_eq!(end_of_february - start_of_february, 28);
    }

    #[test]
    fn month_2_of_year_400_is_29_days() {
        let fourth_year_first_month = year_to_month(400);
        let start_of_february = month_to_day(fourth_year_first_month + 1);
        let end_of_february = month_to_day(fourth_year_first_month + 2);
        assert_eq!(end_of_february - start_of_february, 29);
    }

    #[test]
    fn epoch_year_is_month_1() {
        assert_eq!(year_to_month(EPOCH_YEAR), 1);
    }

    #[test]
    fn year_after_epoch_year_is_month_13() {
        assert_eq!(year_to_month(EPOCH_YEAR + 1), 13);
    }

    #[test]
    fn year_before_epoch_year_is_month_minus_11() {
        assert_eq!(year_to_month(EPOCH_YEAR - 1), -11);
    }

    #[test]
    fn year_minus_400_is_leap_year() {
        assert_eq!(leap_days_before_year(-399) - leap_days_before_year(-400), 1);
    }

    #[test]
    fn year_minus_100_is_not_leap_year() {
        assert_eq!(leap_days_before_year(-99) - leap_days_before_year(-100), 0);
    }

    #[test]
    fn year_minus_4_is_leap_year() {
        assert_eq!(leap_days_before_year(-3) - leap_days_before_year(-4), 1);
    }

    #[test]
    fn year_0_is_leap_year() {
        assert_eq!(leap_days_before_year(1) - leap_days_before_year(0), 1);
    }

    #[test]
    fn year_1_is_not_leap_year() {
        assert_eq!(leap_days_before_year(2) - leap_days_before_year(1), 0);
    }

    #[test]
    fn year_4_is_leap_year() {
        assert_eq!(leap_days_before_year(5) - leap_days_before_year(4), 1);
    }

    #[test]
    fn year_100_is_not_leap_year() {
        assert_eq!(leap_days_before_year(101) - leap_days_before_year(100), 0);
    }

    #[test]
    fn year_400_is_leap_year() {
        assert_eq!(leap_days_before_year(401) - leap_days_before_year(400), 1);
    }

    #[test]
    fn minute_0_is_second_0() {
        assert_eq!(minute_to_second(0), 0);
    }

    #[test]
    fn minute_1_is_second_60() {
        assert_eq!(minute_to_second(1), 60);
    }

    #[test]
    fn month_minus_11_to_0_is_year_before_epoch_year() {
        for month in -11..1 {
            assert_eq!(
                month_to_year(month),
                EPOCH_YEAR - 1,
                "month {} should be year {}",
                month,
                EPOCH_YEAR - 1
            );
        }
    }

    #[test]
    fn month_1_to_12_are_epoch_year() {
        for month in 1..13 {
            assert_eq!(month_to_year(month), EPOCH_YEAR);
        }
    }

    #[test]
    fn month_13_is_year_after_epoch_year() {
        assert_eq!(month_to_year(13), EPOCH_YEAR + 1);
    }

    #[test]
    fn days_minus_30_to_0_are_month_0() {
        for day in -30..1 {
            assert_eq!(day_to_month(day), 0, "day {} should be in month 0", day);
        }
    }

    #[test]
    fn days_1_to_31_are_month_1() {
        for day in 1..32 {
            assert_eq!(day_to_month(day), 1);
        }
    }

    #[test]
    fn day_32_is_month_2() {
        assert_eq!(day_to_month(32), 2);
    }

    #[test]
    fn hours_0_to_23_are_day_1() {
        for hour in 0..24 {
            assert_eq!(hour_to_day(hour), 1);
        }
    }

    #[test]
    fn hour_24_is_day_2() {
        assert_eq!(hour_to_day(24), 2);
    }

    #[test]
    fn minutes_0_to_59_is_hour_0() {
        for minute in 0..60 {
            assert_eq!(minute_to_hour(minute), 0);
        }
    }

    #[test]
    fn minute_60_is_hour_1() {
        assert_eq!(minute_to_hour(60), 1);
    }

    #[test]
    fn seconds_0_to_59_are_minute_0() {
        for second in 0..60 {
            assert_eq!(
                second_to_minute(second),
                0,
                "second {} should be minute 0",
                second
            );
        }
    }

    #[test]
    fn second_60_is_minute_1() {
        assert_eq!(second_to_minute(60), 1);
    }

    #[test]
    fn leap_years_are_every_4_years_except_100_except_400() {
        for year in 1..2021 {
            let leap_days = leap_days_before_year(year + 1) - leap_days_before_year(year);
            let is_leap_year = year % 4 == 0 && !(year % 100 == 0 && !(year % 400 == 0));
            assert_eq!(
                leap_days,
                if is_leap_year { 1 } else { 0 },
                "year {} should be leap year: {}",
                year,
                is_leap_year
            );
        }
    }

    #[test]
    fn normal_years_have_standard_month_lengths() {
        let zmonth_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for month_in_year in 1..13 {
            let zmonth = month_in_year - 1;
            let days = zmonth_days[zmonth as usize];
            let month = year_to_month(3) + zmonth;
            assert_eq!(
                month_to_day(month + 1) - month_to_day(month),
                days,
                "month {} should have {} days during normal years",
                month_in_year,
                days
            );
        }
    }

    #[test]
    fn leap_years_have_29_days_in_february() {
        let zmonth_days_leap_year = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for month_in_year in 1..13 {
            let zmonth = month_in_year - 1;
            let days = zmonth_days_leap_year[zmonth as usize];
            let month = year_to_month(4) + zmonth;
            assert_eq!(
                month_to_day(month + 1) - month_to_day(month),
                days,
                "month {} should have {} days during leap years",
                month_in_year,
                days
            );
        }
    }

    #[test]
    fn leap_seconds_are_at_end_of_june_in_eleven_years() {
        let june_leap_second_years = [
            1972, 1981, 1982, 1983, 1985, 1992, 1993, 1994, 1997, 2012, 2015,
        ];
        for year in 1..2021 {
            let is_leap = june_leap_second_years.contains(&year);
            assert_eq!(
                year_month_day_hour_minute_to_second(year, 7, 1, 0, 0)
                    - year_month_day_hour_minute_to_second(year, 6, 30, 23, 59),
                if is_leap { 61 } else { 60 },
                "June {} should be leap: {}",
                year,
                is_leap
            );
        }
    }

    #[test]
    fn leap_seconds_are_at_end_of_december_in_sixteen_years() {
        let december_leap_second_years = [
            1972, 1973, 1974, 1975, 1976, 1977, 1978, 1979, 1987, 1989, 1990, 1995, 1998, 2005,
            2008, 2016,
        ];
        for year in 1..2021 {
            let is_leap = december_leap_second_years.contains(&year);
            assert_eq!(
                year_month_day_hour_minute_to_second(year + 1, 1, 1, 0, 0)
                    - year_month_day_hour_minute_to_second(year, 12, 31, 23, 59),
                if is_leap { 61 } else { 60 },
                "December of year {} should be leap: {}",
                year,
                is_leap
            );
        }
    }

    #[test]
    fn days_of_normal_year_match_standard_months() {
        let zmonth_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for year in 1..3000 {
            if leap_days_before_year(year + 1) == leap_days_before_year(year) {
                let mut start_of_month = month_to_day(year_to_month(year));
                for zmonth in 0..12 {
                    let month = year_to_month(year) + zmonth;
                    let end_of_month = start_of_month + zmonth_days[zmonth as usize];
                    for day in start_of_month..end_of_month {
                        assert_eq!(
                            day_to_month(day),
                            month,
                            "day {} of normal year {} is in month {} (zmonth: {}, start: {}, end:{})",
                            day,
                            year,
                            month,
                            zmonth,
                            start_of_month,
                            end_of_month
                        );
                    }
                    start_of_month = end_of_month;
                }
            }
        }
    }

    #[test]
    fn days_of_leap_year_match_leap_year_months() {
        let zmonth_days = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for year in 1..3000 {
            if leap_days_before_year(year + 1) > leap_days_before_year(year) {
                let mut start_of_month = month_to_day(year_to_month(year));
                for zmonth in 0..12 {
                    let month = year_to_month(year) + zmonth;
                    let end_of_month = start_of_month + zmonth_days[zmonth as usize];
                    for day in start_of_month..end_of_month {
                        assert_eq!(
                            day_to_month(day), month,
                            "day {} of leap year {} is in month {} (zmonth: {}, start: {}, end: {})",
                            day, year, month, zmonth, start_of_month, end_of_month);
                    }
                    start_of_month = end_of_month;
                }
            }
        }
    }

    #[test]
    fn minutes_with_leap_seconds_are_61_seconds_at_end_of_june_in_eleven_years() {
        let june_leap_second_years = [
            1972, 1981, 1982, 1983, 1985, 1992, 1993, 1994, 1997, 2012, 2015,
        ];
        for year in 1..2021 {
            let is_leap = june_leap_second_years.contains(&year);
            let start_of_minute = year_month_day_hour_minute_to_second(year, 6, 30, 23, 59);
            let end_of_minute = start_of_minute + if is_leap { 61 } else { 60 };
            assert_eq!(
                second_to_minute(end_of_minute) - second_to_minute(start_of_minute),
                1,
                "Last minute of June of year {} should be leap: {}",
                year,
                is_leap
            );
            assert_eq!(
                second_to_minute(end_of_minute) - second_to_minute(start_of_minute) + 1,
                2,
                "Last minute of June of year {} should be leap: {}",
                year,
                is_leap
            );
        }
    }

    #[test]
    fn minutes_with_leap_seconds_are_61_seconds_at_end_of_december_in_sixteen_years() {
        let december_leap_second_years = [
            1972, 1973, 1974, 1975, 1976, 1977, 1978, 1979, 1987, 1989, 1990, 1995, 1998, 2005,
            2008, 2016,
        ];
        for year in 1..2021 {
            let is_leap = december_leap_second_years.contains(&year);
            let start_of_minute = year_month_day_hour_minute_to_second(year, 12, 31, 23, 59);
            let end_of_minute = start_of_minute + if is_leap { 61 } else { 60 };
            assert_eq!(
                second_to_minute(end_of_minute) - second_to_minute(start_of_minute),
                1,
                "Last minute of December of year {} should be leap: {}",
                year,
                is_leap
            );
            assert_eq!(
                second_to_minute(end_of_minute) - second_to_minute(start_of_minute) + 1,
                2,
                "Last minute of December of year {} should be leap: {}",
                year,
                is_leap
            );
        }
    }

    #[test]
    fn second_to_minute_roundtrip_near_limits() {
        let low_start = second_to_minute_up(Mark::MIN);
        let high_end = second_to_minute(Mark::MAX);
        for second in low_start..low_start + 120 {
            let delta = second - minute_to_second(second_to_minute(second));
            assert!(delta >= 0 && delta < 60);
        }
        for second in high_end - 120..=high_end {
            let delta = second - minute_to_second(second_to_minute(second));
            assert!(delta >= 0 && delta < 60);
        }
    }

    #[test]
    fn second_to_minute_up_roundtrip_near_limits() {
        let low_start = second_to_minute_up(Mark::MIN);
        let high_end = second_to_minute(Mark::MAX);
        for second in low_start..low_start + 120 {
            let delta = minute_to_second(second_to_minute_up(second)) - second;
            assert!(delta >= 0 && delta < 60);
        }
        for second in high_end - 120..=high_end {
            let delta = minute_to_second(second_to_minute_up(second)) - second;
            assert!(delta >= 0 && delta < 60);
        }
    }

    #[test]
    fn divide_towards_negative_infinity_rounds_down() {
        assert_eq!(divide_towards_negative_infinity(-60, 60), -1);
        assert_eq!(divide_towards_negative_infinity(-59, 60), -1);
        assert_eq!(divide_towards_negative_infinity(0, 60), 0);
        assert_eq!(divide_towards_negative_infinity(1, 60), 0);
        assert_eq!(divide_towards_negative_infinity(60, 60), 1);
        assert_eq!(
            divide_towards_negative_infinity(-9223372036, 60),
            -153722868
        )
    }

    #[test]
    fn divide_towards_positive_infinity_rounds_up() {
        assert_eq!(divide_towards_positive_infinity(-60, 60), -1);
        assert_eq!(divide_towards_positive_infinity(-59, 60), 0);
        assert_eq!(divide_towards_positive_infinity(0, 60), 0);
        assert_eq!(divide_towards_positive_infinity(1, 60), 1);
        assert_eq!(divide_towards_positive_infinity(60, 60), 1);
    }
}
