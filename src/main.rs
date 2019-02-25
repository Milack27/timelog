#![feature(try_from)]

#[cfg(windows)]
use ansi_term::enable_ansi_support;

use clap::clap_app;
use clap::crate_authors;
use clap::crate_description;
use clap::crate_version;

use std::convert::TryFrom;

use timelog::Command;
use timelog::CommandInput;
use timelog::ForgetableDateTimeInput;
use timelog::GoalArgInput;

fn main() {
    const MNEMONIC_DESCRIPTION: &str = "Primary reference to the task";
    const TASK_CODE_DESCRIPTION: &str = "Reference to the task used in an external tool";
    const FORGOT_DESCRIPTION: &str = "Marks date/time as uncertain";

    let matches = clap_app!(timelog =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@subcommand enter =>
            (about: "Registers the time the user arrived at the workplace")
            (@arg datetime: "Date/time the user arrived")
            (@arg forgot: --forgot -f FORGOT_DESCRIPTION)
        )
        (@subcommand exit =>
            (about: "Registers the time the user left the workplace")
            (@arg datetime: "Date/time the user left")
            (@arg forgot: --forgot -f FORGOT_DESCRIPTION)
        )
        (@subcommand create =>
            (visible_alias: "new")
            (about: "Creates a new task")
            (@arg mnemonic: +required MNEMONIC_DESCRIPTION)
            (@arg code: TASK_CODE_DESCRIPTION)
        )
        (@subcommand edit =>
            (about: "Changes the code and name of a task")
            (@arg mnemonic: +required MNEMONIC_DESCRIPTION)
            (@arg code: TASK_CODE_DESCRIPTION)
        )
        (@subcommand delete =>
            (visible_alias: "del")
            (about: "Removes a task")
            (@arg mnemonic: +required MNEMONIC_DESCRIPTION)
        )
        (@subcommand start =>
            (about: "Registers the time the user started working on a task")
            (@arg mnemonic: +required MNEMONIC_DESCRIPTION)
            (@arg datetime: "Date/time the user started working")
            (@arg forgot: --forgot -f FORGOT_DESCRIPTION)
        )
        (@subcommand stop =>
            (about: "Registers the time the user stopped working on the current task")
            (@arg mnemonic: MNEMONIC_DESCRIPTION)
            (@arg datetime: "Date/time the user stopped working")
            (@arg forgot: --forgot -f FORGOT_DESCRIPTION)
        )
        (@subcommand commit =>
            (about: "Marks a time period worked on a task as logged in an external tool")
            (@arg mnemonic: +required MNEMONIC_DESCRIPTION)
            (@arg datetime: "Date/time until which all time has been logged")
        )
        (@subcommand open =>
            (about: "Opens the log file of a task")
            (@arg mnemonic: MNEMONIC_DESCRIPTION)
        )
        (@subcommand resolve =>
            (about: "Allows the user to provide a better estimate of date/time of the entries marked as forgot")
            (@arg mnemonic: MNEMONIC_DESCRIPTION)
        )
        (@subcommand goal =>
            (about: "Sets a time goal for a provided task or for the work in general")
            (@arg period: +required +takes_value --period -p "Period of the goal (month, week, day, or a day of the week)")
            (@group goal +required =>
                (@arg time: +takes_value --time -t "Expected worked time (e.g. 8h 48m)")
                (@arg erase: --erase -e "Erase the time goal for the given period")
            )
            (@arg mnemonic: MNEMONIC_DESCRIPTION)
        )
        (@subcommand goals =>
            (about: "Displays the time goals for a provided task or for the work in general")
            (@arg mnemonic: MNEMONIC_DESCRIPTION)
        )
        (@subcommand status =>
            (about: "Displays general information about the current status of the user's work")
            (@arg mnemonic: MNEMONIC_DESCRIPTION)
        )
    ).get_matches();

    const REQUIRED_FIELD_EXPECTED: &str = "Required field not found!";

    let command_input = match matches.subcommand() {
        ("enter", Some(submatches)) => CommandInput::Enter {
            datetime: ForgetableDateTimeInput::from(submatches),
        },
        ("exit", Some(submatches)) => CommandInput::Exit {
            datetime: ForgetableDateTimeInput::from(submatches),
        },
        ("create", Some(submatches)) => CommandInput::Create {
            mnemonic: submatches.value_of("mnemonic").expect(REQUIRED_FIELD_EXPECTED),
            code: submatches.value_of("code"),
        },
        ("edit", Some(submatches)) => CommandInput::Edit {
            mnemonic: submatches.value_of("mnemonic").expect(REQUIRED_FIELD_EXPECTED),
            code: submatches.value_of("code"),
        },
        ("delete", Some(submatches)) => CommandInput::Delete {
            mnemonic: submatches.value_of("mnemonic").expect(REQUIRED_FIELD_EXPECTED),
        },
        ("start", Some(submatches)) => CommandInput::Start {
            mnemonic: submatches.value_of("mnemonic").expect(REQUIRED_FIELD_EXPECTED),
            datetime: ForgetableDateTimeInput::from(submatches),
        },
        ("stop", Some(submatches)) => CommandInput::Stop {
            mnemonic: submatches.value_of("mnemonic"),
            datetime: ForgetableDateTimeInput::from(submatches),
        },
        ("commit", Some(submatches)) => CommandInput::Commit {
            mnemonic: submatches.value_of("mnemonic").expect(REQUIRED_FIELD_EXPECTED),
            datetime: submatches.value_of("datetime"),
        },
        ("open", Some(submatches)) => CommandInput::Open {
            mnemonic: submatches.value_of("mnemonic"),
        },
        ("resolve", Some(submatches)) => CommandInput::Resolve {
            mnemonic: submatches.value_of("mnemonic"),
        },
        ("goal", Some(submatches)) => CommandInput::Goal {
            period: submatches.value_of("period").expect(REQUIRED_FIELD_EXPECTED),
            goal_arg: GoalArgInput::from(submatches),
            mnemonic: submatches.value_of("mnemonic"),
        },
        ("goals", Some(submatches)) => CommandInput::Goals {
            mnemonic: submatches.value_of("mnemonic"),
        },
        ("status", Some(submatches)) => CommandInput::Status {
            mnemonic: submatches.value_of("mnemonic"),
        },
        _ => return,
    };

    #[cfg(windows)]
    enable_ansi_support().ok();

    let command = match Command::try_from(command_input) {
        Ok(command) => command,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    if let Err(error) = command.execute() {
        println!("{}", error);
    }
}
