use ansi_term::Color;

use chrono::prelude::*;

use core::str::FromStr;

use super::parse_duration;
use super::Command;
use super::DateTime;
use super::DurationParseError;
use super::ForgetableDateTime;
use super::GoalAction;
use super::GoalPeriod;
use super::InvalidGoalPeriod;

use std::convert::From;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Error as FormatError;
use std::fmt::Formatter;

//==============================================================================
//
//                                Input Types
//
//==============================================================================

/// Represent a command in the same format as invoked by the user, with possibly missing parameters.
/// For example, if a command requires a date/time parameter and the user doesn't provide it, the current date/time is used.
/// All the missing parameters can be resolved by converting the `CommandInput` into a `Command` object.
pub enum CommandInput<'a> {
    Enter {
        datetime: ForgetableDateTimeInput<'a>,
    },
    Exit {
        datetime: ForgetableDateTimeInput<'a>,
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
        datetime: ForgetableDateTimeInput<'a>,
    },
    Stop {
        mnemonic: Option<&'a str>,
        datetime: ForgetableDateTimeInput<'a>,
        commit: bool,
    },
    Commit {
        mnemonic: &'a str,
        datetime: Option<&'a str>,
    },
    Resolve {
        mnemonic: Option<&'a str>,
    },
    Goal {
        action: GoalActionInput<'a>,
        arg: Option<GoalArgInput<'a>>,
        mnemonic: Option<&'a str>,
    },
    Goals {
        mnemonic: Option<&'a str>,
    },
    Status {
        mnemonic: Option<&'a str>,
    },
}

pub struct ForgetableDateTimeInput<'a> {
    pub datetime: Option<&'a str>,
    pub forgotten: bool,
}

pub enum GoalActionInput<'a> {
    Set(&'a str),
    EraseAll,
}

pub enum GoalArgInput<'a> {
    Time(&'a str),
    Erase,
}

//==============================================================================
//
//                                Error Types
//
//==============================================================================

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
            CommandInput::Stop {
                mnemonic,
                datetime,
                commit,
            } => Command::Stop {
                mnemonic,
                datetime: ForgetableDateTime::try_from(datetime)?,
                commit,
            },
            CommandInput::Commit { mnemonic, datetime } => Command::Commit {
                mnemonic,
                datetime: parse_datetime_or_now(datetime)?,
            },
            CommandInput::Resolve { mnemonic } => Command::Resolve { mnemonic },
            CommandInput::Goal {
                action,
                arg,
                mnemonic,
            } => Command::Goal {
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
        write!(
            f,
            "{}",
            match self {
                DurationParseError::InvalidFormat => "invalid duration format",
                DurationParseError::InvalidHourNumber => "invalid hour number",
                DurationParseError::InvalidMinuteNumber => "invalid minute number",
                DurationParseError::EmptyDuration => "empty duration",
            }
        )
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

fn parse_goal_action<'a>(
    action: GoalActionInput<'a>,
    arg: Option<GoalArgInput<'a>>,
) -> Result<GoalAction, GoalActionParseError> {
    match (action, arg) {
        (GoalActionInput::EraseAll, None) => Ok(GoalAction::EraseAll),
        (GoalActionInput::EraseAll, Some(_)) => Err(GoalActionParseError::UnexpectedArg),
        (GoalActionInput::Set(_), None) => Err(GoalActionParseError::MissingArg),
        (GoalActionInput::Set(period), Some(GoalArgInput::Erase)) => {
            Ok(GoalAction::Erase(GoalPeriod::from_str(period)?))
        }
        (GoalActionInput::Set(period), Some(GoalArgInput::Time(time))) => Ok(GoalAction::Set(
            GoalPeriod::from_str(period)?,
            parse_duration(time)?,
        )),
    }
}
