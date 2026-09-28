//! Partial calendar dates (`YYYY`, `YYYY-MM`, `YYYY-MM-DD`) shared by timeline parsing and rendering.
//!
//! A partial date names a whole period — a year, a month, or a day. As a point or a period
//! start it sits at the first day of that period; as a period end it is inclusive, so it
//! reaches the first day of the *next* period.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Precision {
    Year,
    Month,
    Day,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PartialDate {
    year: i64,
    month: i64,
    day: i64,
    precision: Precision,
}

impl PartialDate {
    /// Parses `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`, rejecting out-of-range months and days.
    pub(crate) fn parse(s: &str) -> Option<PartialDate> {
        let b = s.as_bytes();
        let digits = |r: std::ops::Range<usize>| -> Option<i64> {
            let part = b.get(r)?;
            if part.iter().all(u8::is_ascii_digit) {
                Some(part.iter().fold(0i64, |acc, &d| acc * 10 + (d - b'0') as i64))
            } else {
                None
            }
        };
        let (year, month, day, precision) = match b.len() {
            4 => (digits(0..4)?, 1, 1, Precision::Year),
            7 if b[4] == b'-' => (digits(0..4)?, digits(5..7)?, 1, Precision::Month),
            10 if b[4] == b'-' && b[7] == b'-' => (digits(0..4)?, digits(5..7)?, digits(8..10)?, Precision::Day),
            _ => return None,
        };
        if !(1..=12).contains(&month) || day < 1 || day > days_in_month(year, month) {
            return None;
        }
        Some(PartialDate { year, month, day, precision })
    }

    /// Day number of the first day of the period.
    pub(crate) fn start_day(&self) -> i64 {
        day_number(self.year, self.month, self.day)
    }

    /// Day number of the first day *after* the period — the exclusive bound of an inclusive end.
    pub(crate) fn end_day_exclusive(&self) -> i64 {
        match self.precision {
            Precision::Year => day_number(self.year + 1, 1, 1),
            Precision::Month if self.month == 12 => day_number(self.year + 1, 1, 1),
            Precision::Month => day_number(self.year, self.month + 1, 1),
            Precision::Day => self.start_day() + 1,
        }
    }
}

fn is_leap(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        2 if is_leap(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// Julian Day Number (proleptic Gregorian calendar).
fn day_number(y: i64, m: i64, d: i64) -> i64 {
    let a = (14 - m) / 12;
    let yr = y + 4800 - a;
    let mo = m + 12 * a - 3;
    d + (153 * mo + 2) / 5 + 365 * yr + yr / 4 - yr / 100 + yr / 400 - 32045
}

#[cfg(test)]
mod tests {
    use super::PartialDate;

    fn p(s: &str) -> PartialDate {
        PartialDate::parse(s).unwrap_or_else(|| panic!("`{s}` should parse"))
    }

    #[test]
    fn accepts_three_precisions() {
        assert!(PartialDate::parse("2024").is_some());
        assert!(PartialDate::parse("2024-02").is_some());
        assert!(PartialDate::parse("2024-02-29").is_some());
    }

    #[test]
    fn rejects_malformed_and_out_of_range() {
        for s in ["January 2024", "24", "2024-1", "2024/01", "2024-00", "2024-13", "2023-02-29", "2024-04-31", "2024-01-00"] {
            assert!(PartialDate::parse(s).is_none(), "`{s}` should be rejected");
        }
    }

    #[test]
    fn start_orders_chronologically() {
        assert!(p("2023-12-31").start_day() < p("2024").start_day());
        assert_eq!(p("2024").start_day(), p("2024-01-01").start_day());
        assert_eq!(p("2024-06").start_day(), p("2024-06-01").start_day());
    }

    #[test]
    fn inclusive_end_reaches_next_period() {
        assert_eq!(p("2024").end_day_exclusive(), p("2025-01-01").start_day());
        assert_eq!(p("2024-03").end_day_exclusive(), p("2024-04-01").start_day());
        assert_eq!(p("2024-12").end_day_exclusive(), p("2025-01-01").start_day());
        assert_eq!(p("2024-02-29").end_day_exclusive(), p("2024-03-01").start_day());
    }
}
