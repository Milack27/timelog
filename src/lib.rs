#![feature(try_from)]

use chrono::prelude::*;
use chrono::Duration;

use clap::ArgMatches;

use core::str::FromStr;

use lazy_static::lazy_static;

use regex::Regex;

use std::convert::From;
use std::convert::TryFrom;

type DateTime = chrono::DateTime<Local>;

/// Represents a command containing all the needed parameters to be executed.
pub enum Command<'a> {
    Enter { datetime: ForgetableDateTime },
    Exit { datetime: ForgetableDateTime },
    Create { mnemonic: &'a str, code: Option<&'a str> },
    Edit { mnemonic: &'a str, code: Option<&'a str> },
    Delete { mnemonic: &'a str },
    Start { mnemonic: &'a str, datetime: ForgetableDateTime },
    Stop { mnemonic: Option<&'a str>, datetime: ForgetableDateTime },
    Commit { mnemonic: &'a str, datetime: DateTime },
    Open { mnemonic: Option<&'a str> },
    Resolve { mnemonic: Option<&'a str> },
    Goal { period: GoalPeriod, goal_arg: GoalArg, mnemonic: Option<&'a str> },
    Goals { mnemonic: Option<&'a str> },
    Status { mnemonic: Option<&'a str> },
}

/// Represent a command in the same format as invoked by the user, with possibly missing parameters.
/// For example, if a command requires a date/time parameter and the user doesn't provide it, the current date/time is used.
/// All the missing parameters can be resolved by converting the `CommandIput` into a `Command` object.
pub enum CommandInput<'a> {
    Enter { datetime: ForgetableDateTimeInput<'a> },
    Exit { datetime: ForgetableDateTimeInput<'a> },
    Create { mnemonic: &'a str, code: Option<&'a str> },
    Edit { mnemonic: &'a str, code: Option<&'a str> },
    Delete { mnemonic: &'a str },
    Start { mnemonic: &'a str, datetime: ForgetableDateTimeInput<'a> },
    Stop { mnemonic: Option<&'a str>, datetime: ForgetableDateTimeInput<'a> },
    Commit { mnemonic: &'a str, datetime: Option<&'a str> },
    Open { mnemonic: Option<&'a str> },
    Resolve { mnemonic: Option<&'a str> },
    Goal { period: &'a str, goal_arg: GoalArgInput<'a>, mnemonic: Option<&'a str> },
    Goals { mnemonic: Option<&'a str> },
    Status { mnemonic: Option<&'a str> },
}

pub struct ForgetableDateTime {
    datetime: DateTime,
    forgot: bool,
}

pub struct ForgetableDateTimeInput<'a> {
    datetime: Option<&'a str>,
    forgot: bool,
}

pub enum GoalPeriod {
    Month,
    Week,
    Day,
    Weekday(Weekday),
}

pub enum GoalArg {
    Set(Duration),
    Erase,
}

pub enum GoalArgInput<'a> {
    Set(&'a str),
    Erase,
}

pub enum DurationParseError {
    InvalidFormat,
    InvalidHourNumber,
    InvalidMinuteNumber,
    EmptyDuration,
}

pub struct InvalidGoalPeriod;

pub enum CommandParseError {
    DateTimeParseError(chrono::format::ParseError),
    DurationParseError(DurationParseError),
    GoalPeriodParseError(InvalidGoalPeriod),
}

impl<'a> Command<'a> {
    pub fn execute(&self) -> Result<(), ()> {
        unimplemented!()
    }
}

impl<'a> CommandInput<'a> {
    pub fn execute(&self) -> Result<(), ()> {
        unimplemented!()
    }
}

impl<'a> TryFrom<CommandInput<'a>> for Command<'a> {
    type Error = CommandParseError;

    fn try_from(input: CommandInput<'a>) -> Result<Command, Self::Error> {
        Ok(match input {
            CommandInput::Enter { datetime } => Command::Enter {
                datetime: ForgetableDateTime::try_from(datetime)?,
            },
            CommandInput::Exit { datetime } => Command::Exit {
                datetime: ForgetableDateTime::try_from(datetime)?,
            },
            CommandInput::Create { mnemonic, code } => Command::Create { mnemonic, code },
            CommandInput::Edit { mnemonic, code } => Command::Edit { mnemonic, code },
            CommandInput::Delete { mnemonic } => Command::Delete { mnemonic },
            CommandInput::Start { mnemonic, datetime } => Command::Start {
                mnemonic,
                datetime: ForgetableDateTime::try_from(datetime)?,
            },
            CommandInput::Stop { mnemonic, datetime } => Command::Stop {
                mnemonic,
                datetime: ForgetableDateTime::try_from(datetime)?,
            },
            CommandInput::Commit { mnemonic, datetime } => Command::Commit {
                mnemonic,
                datetime: parse_datetime_or_now(datetime)?,
            },
            CommandInput::Open { mnemonic } => Command::Open { mnemonic },
            CommandInput::Resolve { mnemonic } => Command::Resolve { mnemonic },
            CommandInput::Goal { period, goal_arg, mnemonic } => Command::Goal {
                period: GoalPeriod::from_str(period)?,
                goal_arg: GoalArg::try_from(goal_arg)?,
                mnemonic,
            },
            CommandInput::Goals { mnemonic } => Command::Goals { mnemonic },
            CommandInput::Status { mnemonic } => Command::Status { mnemonic },
        })
    }
}

impl<'a> TryFrom<ForgetableDateTimeInput<'a>> for ForgetableDateTime {
    type Error = chrono::format::ParseError;

    fn try_from(input: ForgetableDateTimeInput<'a>) -> Result<ForgetableDateTime, Self::Error> {
        Ok(ForgetableDateTime {
            datetime: parse_datetime_or_now(input.datetime)?,
            forgot: input.forgot,
        })
    }
}

impl<'a> TryFrom<GoalArgInput<'a>> for GoalArg {
    type Error = DurationParseError;

    fn try_from(input: GoalArgInput<'a>) -> Result<GoalArg, Self::Error> {
        Ok(match input {
            GoalArgInput::Set(duration) => GoalArg::Set(parse_duration(duration)?),
            GoalArgInput::Erase => GoalArg::Erase,
        })
    }
}

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

impl From<chrono::format::ParseError> for CommandParseError {
    fn from(error: chrono::format::ParseError) -> CommandParseError {
        CommandParseError::DateTimeParseError(error)
    }
}

impl From<DurationParseError> for CommandParseError {
    fn from(error: DurationParseError) -> CommandParseError {
        CommandParseError::DurationParseError(error)
    }
}

impl From<InvalidGoalPeriod> for CommandParseError {
    fn from(error: InvalidGoalPeriod) -> CommandParseError {
        CommandParseError::GoalPeriodParseError(error)
    }
}

impl<'a> From<&'a ArgMatches<'a>> for ForgetableDateTimeInput<'a> {
    fn from(matches: &'a ArgMatches<'a>) -> ForgetableDateTimeInput<'a> {
        ForgetableDateTimeInput {
            datetime: matches.value_of("datetime"),
            forgot: matches.is_present("forgot"),
        }
    }
}

impl<'a> From<&'a ArgMatches<'a>> for GoalArgInput<'a> {
    fn from(matches: &'a ArgMatches<'a>) -> GoalArgInput<'a> {
        if matches.is_present("erase") {
            GoalArgInput::Erase
        } else {
            GoalArgInput::Set(
                matches
                    .value_of("time")
                    .expect("Required field not found!")
            )
        }
    }
}

fn parse_datetime_or_now(input: Option<&str>) -> Result<DateTime, chrono::format::ParseError> {
    input
        .map(|s| DateTime::from_str(s))
        .unwrap_or_else(|| Ok(Local::now()))
}

fn parse_duration(input: &str) -> Result<Duration, DurationParseError> {
    lazy_static! {
        static ref DURATION_REGEX: Regex = Regex::new(r"^(?:(\d+)h)? *(?:(\d+)m)?$").unwrap();
    }

    DURATION_REGEX
        .captures(input)
        .ok_or(DurationParseError::InvalidFormat)
        .and_then(|cap| {
            let hours = cap.get(1)
                .map(|h| h.as_str()
                    .parse::<i64>()
                    .map(Duration::hours)
                    .map_err(|_| DurationParseError::InvalidHourNumber)
                )
                .transpose()?;
            
            let minutes = cap.get(2)
                .map(|m| m.as_str()
                    .parse::<i64>()
                    .map(Duration::minutes)
                    .map_err(|_| DurationParseError::InvalidMinuteNumber)
                )
                .transpose()?;

            match (hours, minutes) {
                (None, None) => Err(DurationParseError::EmptyDuration),
                _ => Ok(
                    hours.unwrap_or_else(Duration::zero) +
                    minutes.unwrap_or_else(Duration::zero)
                ),
            }
        })
}