# Files

## history.log

Contains entries of the following commands:

- Enter (date/time, forgotten?)
- Exit (date/time, forgotten?)
- Start (task, date/time, forgotten?)
- Stop (task, date/time, forgotten?)
- Commit (task)

The entries are encoded in a human-readable text format and in chronological order.

Example:
```html
enter 2019-02-22 9:30
start login-layout 2019-02-22 9:45
stop login-layout 2019-02-22 11:00
exit 2019-02-22 11:17
enter 2019-02-22 12:34
start login-layout 2019-02-22 12:40
stop login-layout 2019-02-22 15:22
commit login-layout                  <!--All work sessions above on login-layout have been logged into the external tool -->
start login-logic 2019-02-22 16:57?  <!--The question mark means the time is inaccurate (user forgot to log it before). When the user resolves a forgotten entry, the question mark is removed and the time is replaced by an estimate provided by the user.-->
stop login-logic 2019-02-22 18:14
exit 2019-02-22 18:15
```

## data.json

Contains data about work in general and the tasks:

- General work
- Tasks
  - Mnemonic
  - Active?
  - Title
  - Code
- Both
  - Time goals

Example:
```json
{
    "tasks": {
        "login-layout": {
            "active": false,
            "title": "Create the login screen interface",
            "code": "PROJ-001",
        },
        "login-logic": {
            "active": true,
            "title": "Integrate the login screen",
            "code": "PROJ-002",
            "goals": {
                "total": "12h",
                "day": "2h",
            },
        },
    },
    "general": {
        "goals": {
            "week": "44h",
            "thursday": "7h",
            "friday": "7h",
            // The time goals of the remaining days are supposed to be inferred from the information available.
        },
    },
}
```

# File handling

Because both history.log and data.json files are likely to become very large over time, timelog operations should avoid reading those files from the start to the end, but read them the opposite way. That will have a positive effect on performance if the data that's more likely to be accessed is stored close to the end of the file. That's one reason why history.log is written in chronological order: the most recent entries are the most likely ones to be accessed.

The data.json file should also store data that's more likely to be accessed close to the end of the file. That could be achieved using the following approach:

1. The "general" object is limited in size and is very often required, so it should always be the last object inside the root.
2. Active tasks are more likely to be accessed than inactive ones, so the former should be stored after the later.
3. Recently inactivated tasks are more likely to be accessed than the ones that have been inactivated earlier. So every time a task is inactivated, it should be moved to the last position before the active tasks.
4. It's assumed that the number of active tasks at any given moment will always be very limited, so there's no need to sort them. However, an optional approach is to bring an active task to the bottom every time it's used.

Timelog commands should also avoid loading those entire files into memory and back to the hard drive. Instead, only the least amount of data should be loaded into memory and then appended back to the file. For certain commands, it's even possible to append data to the history.log file without buffering anything. In general, the following approach is recommended:

1. Load the file into memory only from the point that needs to be changed until the end of the file
2. Make the needed changes in the buffer
3. Truncate the file removing the part that is already buffered
4. Append the buffer back to the file