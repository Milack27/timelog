#![feature(try_from)]

pub mod input;

use chrono::prelude::*;
use chrono::Duration;

use core::str::FromStr;

use lazy_static::lazy_static;

use regex::Regex;

use std::fmt::Display;
use std::fmt::Error as FormatError;
use std::fmt::Formatter;

//==============================================================================
//
//                              Type Definitions
//
//==============================================================================

pub type DateTime = chrono::DateTime<Local>;

/// Represents a command containing all the needed parameters to be executed.
pub enum Command<'a> {
    Enter {
        datetime: ForgetableDateTime,
    },
    Exit {
        datetime: ForgetableDateTime,
    },
    Create {
        mnemonic: &'a str,
        code: Option<&'a str>,
    },
    Edit {
        mnemonic: &'a str,
        code: Option<&'a str>,
    },
    Delete {
        mnemonic: &'a str,
    },
    Start {
        mnemonic: &'a str,
        datetime: ForgetableDateTime,
    },
    Stop {
        mnemonic: Option<&'a str>,
        datetime: ForgetableDateTime,
        commit: bool,
    },
    Commit {
        mnemonic: &'a str,
        datetime: DateTime,
    },
    Resolve {
        mnemonic: Option<&'a str>,
    },
    Goal {
        action: GoalAction,
        mnemonic: Option<&'a str>,
    },
    Goals {
        mnemonic: Option<&'a str>,
    },
    Status {
        mnemonic: Option<&'a str>,
    },
}

pub struct ForgetableDateTime {
    pub datetime: DateTime,
    pub forgotten: bool,
}

pub enum GoalPeriod {
    Month,
    Week,
    Day,
    Weekday(Weekday),
}

pub enum GoalAction {
    Set(GoalPeriod, Duration),
    Erase(GoalPeriod),
    EraseAll,
}

pub struct InvalidGoalPeriod;

pub enum DurationParseError {
    InvalidFormat,
    InvalidHourNumber,
    InvalidMinuteNumber,
    EmptyDuration,
}

//==============================================================================
//
//                              Type Conversions
//
//==============================================================================

impl FromStr for GoalPeriod {
    type Err = InvalidGoalPeriod;

    fn from_str(string: &str) -> Result<Self, InvalidGoalPeriod> {
        Ok(match string {
            "month" => GoalPeriod::Month,
            "week" => GoalPeriod::Week,
            "day" => GoalPeriod::Day,
            "sunday" => GoalPeriod::Weekday(Weekday::Sun),
            "monday" => GoalPeriod::Weekday(Weekday::Mon),
            "tuesday" => GoalPeriod::Weekday(Weekday::Tue),
            "wednesday" => GoalPeriod::Weekday(Weekday::Wed),
            "thursday" => GoalPeriod::Weekday(Weekday::Thu),
            "friday" => GoalPeriod::Weekday(Weekday::Fri),
            "saturday" => GoalPeriod::Weekday(Weekday::Sat),
            _ => return Err(InvalidGoalPeriod),
        })
    }
}

impl Display for InvalidGoalPeriod {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), FormatError> {
        writeln!(f, "invalid goal period")?;
        write!(f, "valid period values: month, week, day, sunday, monday, tuesday, wednesday, thursday, friday, saturday.")
    }
}

pub fn parse_duration(input: &str) -> Result<Duration, DurationParseError> {
    lazy_static! {
        static ref DURATION_REGEX: Regex = Regex::new(r"^(?:(\d+)h)? *(?:(\d+)m)?$").unwrap();
    }

    DURATION_REGEX
        .captures(input)
        .ok_or(DurationParseError::InvalidFormat)
        .and_then(|cap| {
            let hours = cap
                .get(1)
                .map(|h| {
                    h.as_str()
                        .parse::<i64>()
                        .map(Duration::hours)
                        .map_err(|_| DurationParseError::InvalidHourNumber)
                })
                .transpose()?;

            let minutes = cap
                .get(2)
                .map(|m| {
                    m.as_str()
                        .parse::<i64>()
                        .map(Duration::minutes)
                        .map_err(|_| DurationParseError::InvalidMinuteNumber)
                })
                .transpose()?;

            match (hours, minutes) {
                (None, None) => Err(DurationParseError::EmptyDuration),
                _ => {
                    Ok(hours.unwrap_or_else(Duration::zero)
                        + minutes.unwrap_or_else(Duration::zero))
                }
            }
        })
}
