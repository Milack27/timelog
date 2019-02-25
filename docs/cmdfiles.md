# Commands

## `enter` and `start` (`mnemonic`, `forgettable datetime`)

- Assure that there's no open session
- Add new open session at the end of the log

## `exit` and `stop` (`mnemonic`, `forgettable datetime`)

- Find and close an open session at the end of the log

# `commit` (`mnemonic`, `datetime`)

- Find the last commit in the log
- From the last commit, read the log until ??

# `resolve` (`mnemonic`)

## Create { mnemonic: &'a str, code: Option<&'a str> }
## Edit { mnemonic: &'a str, code: Option<&'a str> }
## Delete { mnemonic: &'a str }
## Open { mnemonic: Option<&'a str> }
## Resolve { mnemonic: Option<&'a str> }
## Goal { period: GoalPeriod, goal_arg: GoalArg, mnemonic: Option<&'a str> }
## Goals { mnemonic: Option<&'a str> }
## Status { mnemonic: Option<&'a str> }


# Files

## Whole log

enter 2019-02-12 12:34
exit 2019-02-12 18:43
start mytask 2019-02-12 21:40?
stop mytask 2019-02-13 00:10
commit mytask 

## Sessions

2019-02-12 12:34 -- 18:43
2019-02-12 21:40? -- 02-13 00:10
commit
2019-02-13 09:09 -- 11:18?
2019-02-13 12:35 -- 