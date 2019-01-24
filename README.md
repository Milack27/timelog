# Description

Timelog is a command line tool for tracking the work time.

# Commands

## `timelog enter [date/time] [flags]`
Register the time the user arrived at her workplace.

## `timelog exit [date/time] [flags]`
Register the time the user left her workplace.

## `timelog new <mnemonic> [code]`
Create a new task (the user is asked to type a name for it).

## `timelog edit <mnemonic> [code]`
Change the code and the name of an existing task.

## `timelog delete <mnemonic>`
Delete an existing task.

## `timelog start <mnemonic> [date/time] [flags]`
Register the time the user started working on a task.

## `timelog stop [date/time] [flags]`
Register the time the user stopped working on the current task.

## `timelog commit <mnemonic> [date/time]`
Indicate that all the time worked on a task until the given date/time was logged into an external tool.

## `timelog open [mnemonic]`
Open the log file of a task, or the log file of the work in general if no mnemonic is provided.

## `timelog resolve [mnemonic]`
Prompt the user to provide an estimated time for all pending entries of a task. If a mnemonic is not provided, only the pending entries related to the general work will be resolved.

See `--forgot (-f)`.

## `timelog goal <type>=<time span> [mnemonic]`
Set a time goal. If a mnemonic is provided, the goal is valid for the corresponding task. If not, the goal is valid for the work time in general.

`<type>`:
* `month`
* `week`
* `day` (overrides all days of the week)
* `sunday`, `monday`, `tuesday`, `wednesday`, `thursday`, `friday`, `saturday` (overrides that day)

## `timelog goals [mnemonic]`
Display all the goals for a task, or for the work in general if a mnemonic is not provided.

## `timelog status`
Display current status:

* Not working
* Working
  * Current work session
    * Start time
    * Duration
  * Today
    * Worked time
    * Remaining time for daily goal
    * End time for daily goal
  * Week
    * Worked time
    * Remaining time for weekly goal*
    * End time for weekly goal*
  * No active task
  * Working on task
    * Name
    * Mnemomic
    * Code
    * Total worked time
    * Total unlogged time
* Pending times

\* Only displayed if it's possible for the weekly goal to be accomplished along the current day.

# Flags

## `--forgot (-f)`
Used to indicate that an entry should have been registered some time before, but the interval is uncertain. The registry will be set to the current time, but it will also be marked as pending. At a proper moment, the user must provide an estimated time for that entry using the command `timelog resolve`.

Aplicable commands:
* `timelog enter`
* `timelog exit`
* `timelog start`
* `timelog stop`

# Constraints

* Task mnemonics are restricted to letters (case insensitive), numbers and dashes (-).
* Time spans must be formatted as follows: `2h` or `2h 32m` or `2h32m`
* Dates must be formatted as follows: `25` or `25/01` or `25/01/2018`
  * When the month and/or year is omitted, the date is implicitly the most recent fit until the current time
* Times must be formatted as follows: `14h` or `14:15` or `14:15:54`
  * When the second and/or minute is omitted, the date is implicitly the most recent fit until the current time
* Date/time must be formatted as follows: `<date> <time>` or `<date>-<time>`
  * `<date>` and `<time>` follow the rules above.