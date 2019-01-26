#[macro_use]
extern crate clap;

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
}
