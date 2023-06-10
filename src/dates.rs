mod constants;

use chrono::{DateTime, Datelike, Duration, TimeZone};
use chrono_tz::Tz;
use itertools::Itertools;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
struct Date {
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
}

impl Date {
    fn to_timestamp(self, year: i32, timezone: &Tz) -> DateTime<Tz> {
        // TODO: unwrap
        timezone
            .with_ymd_and_hms(year, self.month, self.day, self.hour, self.min, self.sec)
            .unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct IncompleteQuarter {
    number: u8,
    start: Date,
    end: Date,
}

impl IncompleteQuarter {
    pub fn to_quarter(self, year: i32, timezone: &Tz) -> Quarter {
        Quarter {
            number: self.number,
            start: self.start.to_timestamp(year, timezone),
            end: self.end.to_timestamp(year, timezone),
        }
    }
}

pub struct Quarter {
    number: u8,
    start: DateTime<Tz>,
    end: DateTime<Tz>,
}

impl Quarter {
    pub fn is_timestamp_in_quarter(&self, timestamp: DateTime<Tz>) -> bool {
        let is_before = timestamp < self.start;
        let is_after = timestamp >= self.start + (self.end - self.start);

        !is_before && !is_after
    }

    pub fn duration(&self) -> Duration {
        self.end - self.start
    }
}

pub struct QuarterDuration {
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
}

const ONE_MINUTE: i64 = 60;
const ONE_HOUR: i64 = ONE_MINUTE * 60;
const ONE_DAY: i64 = ONE_HOUR * 24;
const ONE_WEEK: i64 = ONE_DAY * 7;

impl From<Duration> for QuarterDuration {
    fn from(value: Duration) -> Self {
        let secs = value.num_seconds();

        Self {
            weeks: secs / ONE_WEEK,
            days: (secs % ONE_WEEK) / ONE_DAY,
            hours: (secs % ONE_DAY) / ONE_HOUR,
            minutes: (secs % ONE_HOUR) / ONE_MINUTE,
            seconds: secs % ONE_MINUTE,
        }
    }
}

impl Display for QuarterDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let weeks = match &self.weeks {
            0 => None,
            1 => Some("1 week".into()),
            x => Some(format!("{x} weeks")),
        };
        let days = match &self.days {
            0 => None,
            1 => Some("1 day".into()),
            x => Some(format!("{x} days")),
        };
        let hours = match &self.hours {
            0 => None,
            1 => Some("1 hour".into()),
            x => Some(format!("{x} hours")),
        };
        let minutes = match &self.minutes {
            0 => None,
            1 => Some("1 minute".into()),
            x => Some(format!("{x} minutes")),
        };
        let seconds = match &self.seconds {
            0 => None,
            1 => Some("1 second".into()),
            x => Some(format!("{x} seconds")),
        };

        let values = [weeks, days, hours, minutes, seconds];

        write!(f, "{}", values.iter().flatten().join(", "))
    }
}

pub struct CurrentQuarter {
    quarter: Quarter,
    timestamp: DateTime<Tz>,
}

impl CurrentQuarter {
    pub fn new(timestamp: DateTime<Tz>) -> Option<Self> {
        constants::QUARTERS
            .iter()
            .map(|q| q.to_quarter(timestamp.year(), &timestamp.timezone()))
            .find(|quarter| quarter.is_timestamp_in_quarter(timestamp))
            .map(|quarter| Self { quarter, timestamp })
    }

    fn duration_since_start(&self) -> Duration {
        self.timestamp - self.quarter.start
    }

    pub fn pretty_duration_since_start(&self) -> QuarterDuration {
        QuarterDuration::from(self.duration_since_start())
    }

    pub fn percentage_completed(&self) -> f32 {
        let div = self.duration_since_start().num_seconds() as f32
            / self.quarter.duration().num_seconds() as f32;
        div * 100.0
    }

    fn duration_left(&self) -> Duration {
        self.quarter.duration() - self.duration_since_start()
    }

    pub fn pretty_duration_left(&self) -> QuarterDuration {
        QuarterDuration::from(self.duration_left())
    }

    pub fn name(&self) -> String {
        format!("Q{} {}", self.quarter.number, self.timestamp.year())
    }
}
