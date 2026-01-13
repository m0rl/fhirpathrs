use crate::Value;
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DatePrecision {
    Year,
    Month,
    Day,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DateTimePrecision {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
}

impl DateTimePrecision {
    pub fn comparable_to(self, other: Self) -> bool {
        self == other
            || matches!(
                (self, other),
                (Self::Second, Self::Millisecond) | (Self::Millisecond, Self::Second)
            )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimePrecision {
    Hour,
    Minute,
    Second,
    Millisecond,
}

impl TimePrecision {
    pub fn comparable_to(self, other: Self) -> bool {
        self == other
            || matches!(
                (self, other),
                (Self::Second, Self::Millisecond) | (Self::Millisecond, Self::Second)
            )
    }
}

/// Represents either a fixed duration or calendar-based period
#[derive(Debug, Clone, PartialEq)]
pub enum TimeInterval {
    /// Fixed duration (days, hours, minutes, seconds, etc.)
    Duration(TimeDelta),
    /// Calendar months (proper month arithmetic)
    Months(i32),
}

pub fn format_date(d: NaiveDate, p: DatePrecision) -> String {
    match p {
        DatePrecision::Year => d.format("%Y").to_string(),
        DatePrecision::Month => d.format("%Y-%m").to_string(),
        DatePrecision::Day => d.format("%Y-%m-%d").to_string(),
    }
}

pub fn format_datetime(
    dt: NaiveDateTime,
    p: DateTimePrecision,
    tz: &Option<FixedOffset>,
) -> String {
    let base = match p {
        DateTimePrecision::Year => dt.format("%Y").to_string(),
        DateTimePrecision::Month => dt.format("%Y-%m").to_string(),
        DateTimePrecision::Day => dt.format("%Y-%m-%d").to_string(),
        DateTimePrecision::Hour => dt.format("%Y-%m-%dT%H").to_string(),
        DateTimePrecision::Minute => dt.format("%Y-%m-%dT%H:%M").to_string(),
        DateTimePrecision::Second => dt.format("%Y-%m-%dT%H:%M:%S").to_string(),
        DateTimePrecision::Millisecond => dt.format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
    };
    format!("{}{}", base, format_tz_suffix(tz))
}

pub fn format_time(t: NaiveTime, p: TimePrecision) -> String {
    match p {
        TimePrecision::Hour => t.format("%H").to_string(),
        TimePrecision::Minute => t.format("%H:%M").to_string(),
        TimePrecision::Second => t.format("%H:%M:%S").to_string(),
        TimePrecision::Millisecond => t.format("%H:%M:%S%.3f").to_string(),
    }
}

pub fn format_tz_suffix(offset: &Option<FixedOffset>) -> String {
    match offset {
        Some(o) => {
            let total_seconds = o.local_minus_utc();
            if total_seconds == 0 {
                "Z".to_string()
            } else {
                let hours = total_seconds / 3600;
                let minutes = (total_seconds.abs() % 3600) / 60;
                format!("{:+03}:{:02}", hours, minutes)
            }
        }
        None => String::new(),
    }
}

pub fn to_utc_naive(dt: NaiveDateTime, offset: &FixedOffset) -> NaiveDateTime {
    match TimeDelta::try_seconds(offset.local_minus_utc() as i64) {
        Some(delta) => dt - delta,
        None => dt,
    }
}

pub fn detect_datetime_precision(s: &str) -> DateTimePrecision {
    let base = s
        .split_once('+')
        .or_else(|| {
            let idx = s.rfind('-').filter(|&i| i > 10)?;
            Some((&s[..idx], &s[idx..]))
        })
        .map_or(s, |(b, _)| b);
    let base = base.trim_end_matches('Z');

    if let Some((date_part, time_part)) = base.split_once('T') {
        if time_part.is_empty() {
            match date_part.matches('-').count() {
                0 => DateTimePrecision::Year,
                1 => DateTimePrecision::Month,
                _ => DateTimePrecision::Day,
            }
        } else if time_part.contains('.') {
            DateTimePrecision::Millisecond
        } else {
            match time_part.matches(':').count() {
                0 => DateTimePrecision::Hour,
                1 => DateTimePrecision::Minute,
                _ => DateTimePrecision::Second,
            }
        }
    } else {
        match base.matches('-').count() {
            0 => DateTimePrecision::Year,
            1 => DateTimePrecision::Month,
            _ => DateTimePrecision::Day,
        }
    }
}

pub fn detect_time_precision(s: &str) -> TimePrecision {
    if s.contains('.') {
        TimePrecision::Millisecond
    } else {
        match s.matches(':').count() {
            0 => TimePrecision::Hour,
            1 => TimePrecision::Minute,
            _ => TimePrecision::Second,
        }
    }
}

#[allow(clippy::cast_precision_loss)]
pub fn calendar_duration_date(d1: NaiveDate, precision: DatePrecision, d2: NaiveDate) -> Value {
    match precision {
        DatePrecision::Year => {
            let years = d2.year() - d1.year();
            Value::Quantity(years as f64, "year".to_string(), None)
        }
        DatePrecision::Month => {
            let months = (d2.year() - d1.year()) * 12 + (d2.month() as i32 - d1.month() as i32);
            Value::Quantity(months as f64, "month".to_string(), None)
        }
        DatePrecision::Day => {
            let days = (d2 - d1).num_days();
            Value::Quantity(days as f64, "day".to_string(), None)
        }
    }
}

#[allow(clippy::cast_precision_loss)]
pub fn calendar_duration_datetime(
    dt1: NaiveDateTime,
    precision: DateTimePrecision,
    dt2: NaiveDateTime,
) -> Value {
    match precision {
        DateTimePrecision::Year => {
            let years = dt2.date().year() - dt1.date().year();
            Value::Quantity(years as f64, "year".to_string(), None)
        }
        DateTimePrecision::Month => {
            let months = (dt2.date().year() - dt1.date().year()) * 12
                + (dt2.date().month() as i32 - dt1.date().month() as i32);
            Value::Quantity(months as f64, "month".to_string(), None)
        }
        DateTimePrecision::Day => {
            let days = (dt2.date() - dt1.date()).num_days();
            Value::Quantity(days as f64, "day".to_string(), None)
        }
        DateTimePrecision::Hour => {
            let hours = (dt2 - dt1).num_hours();
            Value::Quantity(hours as f64, "hour".to_string(), None)
        }
        DateTimePrecision::Minute => {
            let minutes = (dt2 - dt1).num_minutes();
            Value::Quantity(minutes as f64, "minute".to_string(), None)
        }
        DateTimePrecision::Second => {
            let seconds = (dt2 - dt1).num_seconds();
            Value::Quantity(seconds as f64, "second".to_string(), None)
        }
        DateTimePrecision::Millisecond => {
            let millis = (dt2 - dt1).num_milliseconds();
            Value::Quantity(millis as f64, "millisecond".to_string(), None)
        }
    }
}

#[allow(clippy::cast_precision_loss)]
pub fn calendar_duration_time(t1: NaiveTime, precision: TimePrecision, t2: NaiveTime) -> Value {
    let diff = t2 - t1;
    match precision {
        TimePrecision::Hour => Value::Quantity(diff.num_hours() as f64, "hour".to_string(), None),
        TimePrecision::Minute => {
            Value::Quantity(diff.num_minutes() as f64, "minute".to_string(), None)
        }
        TimePrecision::Second => {
            Value::Quantity(diff.num_seconds() as f64, "second".to_string(), None)
        }
        TimePrecision::Millisecond => Value::Quantity(
            diff.num_milliseconds() as f64,
            "millisecond".to_string(),
            None,
        ),
    }
}

#[allow(clippy::cast_precision_loss)]
pub fn physical_difference_date(d1: NaiveDate, precision: DatePrecision, d2: NaiveDate) -> Value {
    match precision {
        DatePrecision::Year => {
            let days = (d2 - d1).num_days();
            let years = days as f64 / 365.25;
            Value::Quantity(years.trunc(), "year".to_string(), None)
        }
        DatePrecision::Month => {
            let days = (d2 - d1).num_days();
            let months = days as f64 / 30.4375;
            Value::Quantity(months.trunc(), "month".to_string(), None)
        }
        DatePrecision::Day => {
            let days = (d2 - d1).num_days();
            Value::Quantity(days as f64, "day".to_string(), None)
        }
    }
}

#[allow(clippy::cast_precision_loss)]
pub fn physical_difference_datetime(
    dt1: NaiveDateTime,
    precision: DateTimePrecision,
    dt2: NaiveDateTime,
) -> Value {
    match precision {
        DateTimePrecision::Year => {
            let days = (dt2.date() - dt1.date()).num_days();
            let years = days as f64 / 365.25;
            Value::Quantity(years.trunc(), "year".to_string(), None)
        }
        DateTimePrecision::Month => {
            let days = (dt2.date() - dt1.date()).num_days();
            let months = days as f64 / 30.4375;
            Value::Quantity(months.trunc(), "month".to_string(), None)
        }
        DateTimePrecision::Day => {
            let days = (dt2.date() - dt1.date()).num_days();
            Value::Quantity(days as f64, "day".to_string(), None)
        }
        DateTimePrecision::Hour => {
            let hours = (dt2 - dt1).num_hours();
            Value::Quantity(hours as f64, "hour".to_string(), None)
        }
        DateTimePrecision::Minute => {
            let minutes = (dt2 - dt1).num_minutes();
            Value::Quantity(minutes as f64, "minute".to_string(), None)
        }
        DateTimePrecision::Second => {
            let seconds = (dt2 - dt1).num_seconds();
            Value::Quantity(seconds as f64, "second".to_string(), None)
        }
        DateTimePrecision::Millisecond => {
            let millis = (dt2 - dt1).num_milliseconds();
            Value::Quantity(millis as f64, "millisecond".to_string(), None)
        }
    }
}

pub fn normalize_dt(dt: NaiveDateTime, tz: &Option<FixedOffset>) -> NaiveDateTime {
    match tz {
        Some(offset) => to_utc_naive(dt, offset),
        None => dt,
    }
}

pub fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
    if month == 12 {
        NaiveDate::from_ymd_opt(year, 12, 31).unwrap_or_default()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
            .and_then(|d| d.pred_opt())
            .unwrap_or_default()
    }
}
