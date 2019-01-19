# Description

Timelog is a command line tool for tracking the work time.

# Commands

## timelog enter [flags]
Register the time the user arrived at her workplace.

## timelog exit [flags]
Register the time the user left her workplace.

## timelog new &lt;mnemonic&gt; [code]
Create a new task (the user is asked to type a name for it).

## timelog edit &lt;mnemonic&gt; [code]
Change the code and the name of an existing task.

## timelog delete &lt;mnemonic&gt;
Delete an existing task.

## timelog start &lt;mnemonic&gt; [flags]
Register the time the user started working on a task.

## timelog stop [flags]
Register the time the user stopped working on the current task.

## timelog commit &lt;mnemonic&gt;
Indicate that all the time worked on a task was logged into an external tool.

## timelog resolve [mnemonic]
Prompt the user to provide an estimated time for all pending entries of a task.
If a mnemonic is not provided, only the pending entries related to the general
work will be resolved.

See `--forgot (-f)`.

## timelog goal &lt;type&gt;=&lt;goal&gt; [mnemonic]
Set a time goal. If a mnemonic is provided, the goal is valid for the
corresponding task. If not, the goal is valid for the work time in general.

&lt;type&gt;:
* month
* week
* day (overrides all days of the week)
* sunday, monday, tuesday, wednesday, thursday, friday, saturday (overrides that
day)

&lt;goal&gt;: &lt;hours&gt;h&lt;minutes&gt;m

## timelog goals [mnemonic]
Display all the goals for a task, or for the work in general if a mnemonic is
not provided.

## timelog status
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

\* Only displayed if it's possible for the weekly goal to be accomplished along
the current day.

# Flags

## --forgot (-f)
Used to indicate that an entry should have been registered some time before, but
the interval is uncertain. The registry will be set to the current time, but it
will also be marked as pending. At a proper moment, the user must provide an
estimated time for that entry using the command `timelog resolve`.

Aplicable commands:
* timelog enter
* timelog exit
* timelog start
* timelog stop