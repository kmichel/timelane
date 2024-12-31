//! Extra [`crate::Scaler`] functions to handle sub-second resolutions: milli, micro and nanoseconds.
//!
//! Using these lanes combined with larger ones such as years will greatly reduce the range of the large lanes:
//! even when using 64 bits signed numbers for Mark, we run out of nanoseconds after about 292 years.
//!
//! However, the default year lane is centered around [`super::EPOCH_YEAR`]=2000.
//! All years from 1708 to 2292 are usable with a nanoseconds resolution.
use crate::Mark;

use super::divide_towards_negative_infinity;
use super::divide_towards_positive_infinity;

/// Converts a nanosecond [`Mark`] to a rounded down second [`Mark`].
///
/// # Examples
/// ```
/// use timelane::subsecond::nanosecond_to_second;
/// assert_eq!(nanosecond_to_second(0), 0);
/// assert_eq!(nanosecond_to_second(999_999_999), 0);
/// assert_eq!(nanosecond_to_second(1_000_000_000), 1);
/// use timelane::Mark;
/// assert_eq!(nanosecond_to_second(Mark::MIN), -9_223_372_037);
/// assert_eq!(nanosecond_to_second(Mark::MAX), 9_223_372_036);
/// ```
pub const fn nanosecond_to_second(mark: Mark) -> Mark {
    divide_towards_negative_infinity(mark, 1_000_000_000)
}

/// Converts a nanosecond [`Mark`] to a rounded up second [`Mark`].
///
/// # Examples
/// ```
/// use timelane::subsecond::nanosecond_to_second_up;
/// assert_eq!(nanosecond_to_second_up(0), 0);
/// assert_eq!(nanosecond_to_second_up(1), 1);
/// assert_eq!(nanosecond_to_second_up(1_000_000_000), 1);
/// use timelane::Mark;
/// assert_eq!(nanosecond_to_second_up(Mark::MIN), -9_223_372_036);
/// assert_eq!(nanosecond_to_second_up(Mark::MAX), 9_223_372_037);
/// ```
pub const fn nanosecond_to_second_up(mark: Mark) -> Mark {
    divide_towards_positive_infinity(mark, 1_000_000_000)
}

/// Converts a second [`Mark`] to a nanosecond [`Mark`].
///
/// # Examples
/// ```
/// use timelane::subsecond::second_to_nanosecond;
/// assert_eq!(second_to_nanosecond(0), 0);
/// assert_eq!(second_to_nanosecond(1), 1_000_000_000);
/// use timelane::Mark;
/// assert_eq!(second_to_nanosecond(-9_223_372_036), Mark::MIN + 854_775_808);
/// assert_eq!(second_to_nanosecond(9_223_372_036), Mark::MAX - 854_775_807);
/// ```
pub const fn second_to_nanosecond(mark: Mark) -> Mark {
    mark * 1_000_000_000
}

/// Converts a microsecond [`Mark`] to a rounded down second [`Mark`].
///
/// # Examples
/// ```
/// use timelane::subsecond::microsecond_to_second;
/// assert_eq!(microsecond_to_second(0), 0);
/// assert_eq!(microsecond_to_second(999_999), 0);
/// assert_eq!(microsecond_to_second(1_000_000), 1);
/// use timelane::Mark;
/// assert_eq!(microsecond_to_second(Mark::MIN), -9_223_372_036_855);
/// assert_eq!(microsecond_to_second(Mark::MAX), 9_223_372_036_854);
/// ```
pub const fn microsecond_to_second(mark: Mark) -> Mark {
    divide_towards_negative_infinity(mark, 1_000_000)
}

/// Converts a microsecond [`Mark`] to a rounded up second [`Mark`].
///
/// # Examples
/// ```
/// use timelane::subsecond::microsecond_to_second_up;
/// assert_eq!(microsecond_to_second_up(0), 0);
/// assert_eq!(microsecond_to_second_up(1), 1);
/// assert_eq!(microsecond_to_second_up(1_000_000), 1);
/// use timelane::Mark;
/// assert_eq!(microsecond_to_second_up(Mark::MIN), -9_223_372_036_854);
/// assert_eq!(microsecond_to_second_up(Mark::MAX), 9_223_372_036_855);
/// ```
pub const fn microsecond_to_second_up(mark: Mark) -> Mark {
    divide_towards_positive_infinity(mark, 1_000_000)
}

/// Converts a second [`Mark`] to a microsecond [`Mark`].
///
/// # Examples
/// ```
/// use timelane::subsecond::second_to_microsecond;
/// assert_eq!(second_to_microsecond(0), 0);
/// assert_eq!(second_to_microsecond(1), 1_000_000);
/// use timelane::Mark;
/// assert_eq!(second_to_microsecond(-9_223_372_036_854), Mark::MIN + 775_808);
/// assert_eq!(second_to_microsecond(9_223_372_036_854), Mark::MAX - 775_807);
/// ```
pub const fn second_to_microsecond(mark: Mark) -> Mark {
    mark * 1_000_000
}

/// Converts a millisecond [`Mark`] to a rounded down second [`Mark`].
///
/// # Examples
/// ```
/// use timelane::subsecond::millisecond_to_second;
/// assert_eq!(millisecond_to_second(0), 0);
/// assert_eq!(millisecond_to_second(999), 0);
/// assert_eq!(millisecond_to_second(1_000), 1);
/// use timelane::Mark;
/// assert_eq!(millisecond_to_second(Mark::MIN), -9_223_372_036_854_776);
/// assert_eq!(millisecond_to_second(Mark::MAX), 9_223_372_036_854_775);
/// ```
pub const fn millisecond_to_second(mark: Mark) -> Mark {
    divide_towards_negative_infinity(mark, 1_000)
}

/// Converts a millisecond [`Mark`] to a rounded up second [`Mark`].
///
/// # Examples
/// ```
/// use timelane::subsecond::millisecond_to_second_up;
/// assert_eq!(millisecond_to_second_up(0), 0);
/// assert_eq!(millisecond_to_second_up(1), 1);
/// assert_eq!(millisecond_to_second_up(1_000), 1);
/// use timelane::Mark;
/// assert_eq!(millisecond_to_second_up(Mark::MIN), -9_223_372_036_854_775);
/// assert_eq!(millisecond_to_second_up(Mark::MAX), 9_223_372_036_854_776);
/// ```
pub const fn millisecond_to_second_up(mark: Mark) -> Mark {
    divide_towards_positive_infinity(mark, 1_000)
}

/// Converts a second [`Mark`] to a millisecond [`Mark`].
///
/// # Examples
/// ```
/// use timelane::subsecond::second_to_millisecond;
/// assert_eq!(second_to_millisecond(0), 0);
/// assert_eq!(second_to_millisecond(1), 1_000);
/// use timelane::Mark;
/// assert_eq!(second_to_millisecond(-9_223_372_036_854_775), Mark::MIN + 808);
/// assert_eq!(second_to_millisecond(9_223_372_036_854_775), Mark::MAX - 807);
/// ```
pub const fn second_to_millisecond(mark: Mark) -> Mark {
    mark * 1_000
}
