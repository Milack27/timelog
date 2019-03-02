#![feature(try_from)]

use ansi_term::Color;

use chrono::prelude::*;
use chrono::Duration;

use core::str::FromStr;

use lazy_static::lazy_static;

use regex::Regex;

use std::convert::From;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Error as FormatError;
use std::fmt::Formatter;

//==============================================================================
//
//                              Type Definitions
//
//==============================================================================

type DateTime = chrono::DateTime<Local>;

/// Represents a command containing all the needed parameters to be executed.
pub enum Command<'a> {
    Enter { datetime: ForgetableDateTime },
    Exit { datetime: ForgetableDateTime },
    Create { mnemonic: &'a str, code: Option<&'a str> },
    Edit { mnemonic: &'a str, code: Option<&'a str> },
    Delete { mnemonic: &'a str },
    Start { mnemonic: &'a str, datetime: ForgetableDateTime },
    Stop { mnemonic: Option<&'a str>, datetime: ForgetableDateTime, commit: bool },
    Commit { mnemonic: &'a str, datetime: DateTime },
    Resolve { mnemonic: Option<&'a str> },
    Goal { action: GoalAction, mnemonic: Option<&'a str> },
    Goals { mnemonic: Option<&'a str> },
    Status { mnemonic: Option<&'a str> },
}

/// Represent a command in the same format as invoked by the user, with possibly missing parameters.
/// For example, if a command requires a date/time parameter and the user doesn't provide it, the current date/time is used.
/// All the missing parameters can be resolved by converting the `CommandInput` into a `Command` object.
pub enum CommandInput<'a> {
    Enter { datetime: ForgetableDateTimeInput<'a> },
    Exit { datetime: ForgetableDateTimeInput<'a> },
    Create { mnemonic: &'a str, code: Option<&'a str> },
    Edit { mnemonic: &'a str, code: Option<&'a str> },
    Delete { mnemonic: &'a str },
    Start { mnemonic: &'a str, datetime: ForgetableDateTimeInput<'a> },
    Stop { mnemonic: Option<&'a str>, datetime: ForgetableDateTimeInput<'a>, commit: bool },
    Commit { mnemonic: &'a str, datetime: Option<&'a str> },
    Resolve { mnemonic: Option<&'a str> },
    Goal { action: GoalActionInput<'a>, arg: Option<GoalArgInput<'a>>, mnemonic: Option<&'a str> },
    Goals { mnemonic: Option<&'a str> },
    Status { mnemonic: Option<&'a str> },
}

pub struct ForgetableDateTime {
    pub datetime: DateTime,
    pub forgotten: bool,
}

pub struct ForgetableDateTimeInput<'a> {
    pub datetime: Option<&'a str>,
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

pub enum GoalActionInput<'a> {
    Set(&'a str),
    EraseAll,
}

pub enum GoalArgInput<'a> {
    Time(&'a str),
    Erase,
}

pub enum DurationParseError {
    InvalidFormat,
    InvalidHourNumber,
    InvalidMinuteNumber,
    EmptyDuration,
}

pub struct InvalidGoalPeriod;

pub enum GoalActionParseError {
    UnexpectedArg,
    MissingArg,
    InvalidGoalPeriod(InvalidGoalPeriod),
    DurationParseError(DurationParseError),
}

pub enum CommandParseError {
    DateTimeParseError(chrono::format::ParseError),
    DurationParseError(DurationParseError),
    InvalidGoalPeriod(InvalidGoalPeriod),
    GoalActionParseError(GoalActionParseError),
}

//==============================================================================
//
//                  Conversions From Input to Data Structures
//
//==============================================================================

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
            CommandInput::Stop { mnemonic, datetime, commit } => Command::Stop {
                mnemonic,
                datetime: ForgetableDateTime::try_from(datetime)?,
                commit,
            },
            CommandInput::Commit { mnemonic, datetime } => Command::Commit {
                mnemonic,
                datetime: parse_datetime_or_now(datetime)?,
            },
            CommandInput::Resolve { mnemonic } => Command::Resolve { mnemonic },
            CommandInput::Goal { action, arg, mnemonic } => Command::Goal {
                action: parse_goal_action(action, arg)?,
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
            forgotten: input.forgotten,
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

//==============================================================================
//
//                       Conversions Between Error Types
//
//==============================================================================

impl From<InvalidGoalPeriod> for GoalActionParseError {
    fn from(error: InvalidGoalPeriod) -> GoalActionParseError {
        GoalActionParseError::InvalidGoalPeriod(error)
    }
}

impl From<DurationParseError> for GoalActionParseError {
    fn from(error: DurationParseError) -> GoalActionParseError {
        GoalActionParseError::DurationParseError(error)
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
        CommandParseError::InvalidGoalPeriod(error)
    }
}

impl From<GoalActionParseError> for CommandParseError {
    fn from(error: GoalActionParseError) -> CommandParseError {
        CommandParseError::GoalActionParseError(error)
    }
}

//==============================================================================
//
//                           Display Implementations
//
//==============================================================================

impl Display for DurationParseError {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), FormatError> {
        write!(f, "{}", match self {
            DurationParseError::InvalidFormat => "invalid duration format",
            DurationParseError::InvalidHourNumber => "invalid hour number",
            DurationParseError::InvalidMinuteNumber => "invalid minute number",
            DurationParseError::EmptyDuration => "empty duration",
        })
    }
}

impl Display for InvalidGoalPeriod {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), FormatError> {
        writeln!(f, "invalid goal period")?;
        write!(f, "valid period values: month, week, day, sunday, monday, tuesday, wednesday, thursday, friday, saturday.")
    }
}

impl Display for GoalActionParseError {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), FormatError> {
        match self {
            GoalActionParseError::UnexpectedArg => write!(f, "time/erase argument is not expected"),
            GoalActionParseError::MissingArg => write!(f, "time/erase argument is missing"),
            GoalActionParseError::InvalidGoalPeriod(error) => write!(f, "{}", error),
            GoalActionParseError::DurationParseError(error) => write!(f, "{}", error),
        }
    }
}

impl Display for CommandParseError {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), FormatError> {
        write!(f, "{}: ", Color::Red.paint("error").to_string())?;

        match self {
            CommandParseError::DateTimeParseError(error) => {
                writeln!(f, "could not parse the date/time argument.")?;
                write!(f, "cause: {}", error)
            }
            CommandParseError::DurationParseError(error) => {
                writeln!(f, "could not parse the duration argument.")?;
                write!(f, "cause: {}", error)
            }
            CommandParseError::InvalidGoalPeriod(error) => {
                writeln!(f, "could not parse the period argument.")?;
                write!(f, "cause: {}", error)
            }
            CommandParseError::GoalActionParseError(error) => {
                writeln!(f, "could not parse the goal action.")?;
                write!(f, "cause: {}", error)
            }
        }
    }
}

//==============================================================================
//
//                               Parsing Functions
//
//==============================================================================

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

fn parse_goal_action<'a>(
    action: GoalActionInput<'a>,
    arg: Option<GoalArgInput<'a>>
) -> Result<GoalAction, GoalActionParseError>
{
    match (action, arg) {
        (GoalActionInput::EraseAll, None) => {
            Ok(GoalAction::EraseAll)
        }
        (GoalActionInput::EraseAll, Some(_)) => {
            Err(GoalActionParseError::UnexpectedArg)
        }
        (GoalActionInput::Set(_), None) => {
            Err(GoalActionParseError::MissingArg)
        }
        (GoalActionInput::Set(period), Some(GoalArgInput::Erase)) => {
            Ok(GoalAction::Erase(GoalPeriod::from_str(period)?))
        }
        (GoalActionInput::Set(period), Some(GoalArgInput::Time(time))) => {
            Ok(GoalAction::Set(
                GoalPeriod::from_str(period)?,
                parse_duration(time)?,
            ))
        }
    }
}