//! Calendar dates and inclusive date ranges.

use std::fmt;

use serde::{Serialize, Serializer};

/// A Gregorian calendar date written as `YYYY-MM-DD`.
///
/// Field order makes the derived ordering chronological.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CalendarDate {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DateError {
    /// Not exactly `YYYY-MM-DD` with ASCII digits.
    InvalidFormat,
    /// Well formed, but the year, month, or day does not exist.
    NonexistentDate,
}

impl CalendarDate {
    pub(crate) fn parse(value: &str) -> Result<Self, DateError> {
        let bytes = value.as_bytes();
        let well_formed = bytes.len() == 10
            && bytes[4] == b'-'
            && bytes[7] == b'-'
            && bytes
                .iter()
                .enumerate()
                .all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit());
        if !well_formed {
            return Err(DateError::InvalidFormat);
        }

        let number = |range: std::ops::Range<usize>| {
            bytes[range]
                .iter()
                .fold(0u16, |acc, b| acc * 10 + u16::from(b - b'0'))
        };
        let (year, month, day) = (number(0..4), number(5..7), number(8..10));
        if year == 0 || !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month) {
            return Err(DateError::NonexistentDate);
        }
        Ok(Self {
            year,
            month: month as u8,
            day: day as u8,
        })
    }
}

fn days_in_month(year: u16, month: u16) -> u16 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn is_leap_year(year: u16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

impl fmt::Display for CalendarDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:04}-{:02}-{:02}",
            self.year, self.month, self.day
        )
    }
}

impl Serialize for CalendarDate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// An inclusive range of dates with `start <= end`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub(crate) struct DateRange {
    start: CalendarDate,
    end: CalendarDate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DateRangeError {
    pub(crate) start: CalendarDate,
    pub(crate) end: CalendarDate,
}

impl DateRange {
    pub(crate) fn new(start: CalendarDate, end: CalendarDate) -> Result<Self, DateRangeError> {
        if start > end {
            return Err(DateRangeError { start, end });
        }
        Ok(Self { start, end })
    }

    pub(crate) fn start(&self) -> CalendarDate {
        self.start
    }

    pub(crate) fn end(&self) -> CalendarDate {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(value: &str) -> CalendarDate {
        CalendarDate::parse(value).unwrap()
    }

    #[test]
    fn parses_and_displays_dates() {
        for value in ["2026-09-01", "2026-12-31", "0001-01-01", "9999-12-31"] {
            assert_eq!(date(value).to_string(), value);
        }
    }

    #[test]
    fn applies_leap_year_rules() {
        assert!(CalendarDate::parse("2028-02-29").is_ok());
        assert!(CalendarDate::parse("2000-02-29").is_ok());
        assert_eq!(
            CalendarDate::parse("2026-02-29"),
            Err(DateError::NonexistentDate)
        );
        assert_eq!(
            CalendarDate::parse("1900-02-29"),
            Err(DateError::NonexistentDate)
        );
    }

    #[test]
    fn rejects_nonexistent_dates() {
        for value in [
            "2026-13-01",
            "2026-00-10",
            "2026-04-31",
            "2026-01-00",
            "0000-01-01",
        ] {
            assert_eq!(
                CalendarDate::parse(value),
                Err(DateError::NonexistentDate),
                "{value}"
            );
        }
    }

    #[test]
    fn rejects_malformed_dates() {
        for value in [
            "2026-9-1",
            "2026/09/01",
            "20260901",
            "2026-09-01 ",
            "２０２６-09-01",
            "",
        ] {
            assert_eq!(
                CalendarDate::parse(value),
                Err(DateError::InvalidFormat),
                "{value:?}"
            );
        }
    }

    #[test]
    fn orders_dates_chronologically() {
        assert!(date("2026-09-30") < date("2026-10-01"));
        assert!(date("2025-12-31") < date("2026-01-01"));
        assert!(date("2026-09-01") < date("2026-09-02"));
    }

    #[test]
    fn accepts_single_day_range() {
        let day = date("2026-09-01");
        let range = DateRange::new(day, day).unwrap();
        assert_eq!((range.start(), range.end()), (day, day));
    }

    #[test]
    fn rejects_range_ending_before_start() {
        let (start, end) = (date("2026-09-05"), date("2026-09-01"));
        assert_eq!(
            DateRange::new(start, end),
            Err(DateRangeError { start, end })
        );
    }

    #[test]
    fn serializes_dates_as_strings() {
        let range = DateRange::new(date("2026-09-01"), date("2026-09-05")).unwrap();
        assert_eq!(
            serde_json::to_string(&range).unwrap(),
            r#"{"start":"2026-09-01","end":"2026-09-05"}"#
        );
    }
}
