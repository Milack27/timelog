# Commands

## `enter` and `start` (`mnemonic`, `forgettable datetime`)

- Find the last `start` or `stop` entry related to that task, or the last `enter` or `exit` before `forgettable datetime`
- If last entry is another `enter` or `start`, show it to the user and ask whether to proceed
- Add new entry at the appropriate place
- If the new entry is the last one:
  - Depending on the goals for that task or general work, calculate the time worked on the current day, week and month
  - For each of those periods, display the time left to achieve the goal if it's less than 12 hours

## `exit` and `stop` (`mnemonic`, `forgettable datetime`)

- Find the last `start` or `stop` entry related to that task, or the last `enter` or `exit` before `forgettable datetime`
- If last entry is another `exit` or `stop`, show it to the user and ask whether to proceed
- Add new entry at the appropriate place
- If closing an open session, display the total time worked on it
- If the new entry is the last one:
  - Depending on the goals for that task or general work, calculate the time worked on the current day, week and month
  - For each of those periods, display the time left to achieve the goal if it's less than 12 hours

## `commit` (`mnemonic`, `datetime`)

- Find the last `stop` and `commit` entries related to that task before `datetime`
- If the last `stop` is before the last `commit`, tell the user there's nothing to commit
- Otherwise, place a new `commit` entry immediately after the last `stop`
- Compute the total time worked between the last `commit` and the new one, and display it to the user
- Compute the total time worked after the new `commit`, if any, and display it to the user

## `resolve` (`mnemonic`)

## Create { mnemonic: &'a str, code: Option<&'a str> }
## Edit { mnemonic: &'a str, code: Option<&'a str> }
## Delete { mnemonic: &'a str }
## Open { mnemonic: Option<&'a str> }
## Resolve { mnemonic: Option<&'a str> }
## Goal { period: GoalPeriod, goal_arg: GoalArg, mnemonic: Option<&'a str> }
## Goals { mnemonic: Option<&'a str> }
## Status { mnemonic: Option<&'a str> }
