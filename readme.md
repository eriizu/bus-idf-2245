# Bus IDF 2245

This is a simple binary for querying next buses from a specific stop on Ile-de-France's line 2245 (ex MLV line 27).
It includes the 2023-2024 timetable and all variations (school and bank holidays, and day of the week variations).
The timetable has been extracted by hand as there is no simple way to extract it using open data.

The parser can technically work with all timetables in the same format.
The program cannot differentiate different lines,
but it considers all routes as independent for a simple reason:
line 2245 is a loop, has 4 main routes, and some spurs on school days.
All working properly here.

This program is moslty a experiment of all its parts, and an easy way for my
students to see what next buses they can take after their classes,
in one command on their workstations.

# Usage

Arguments are optional, if not provided you will be asked for your departure
stop.

```sh
cargo run --bin bus -- [PARTIAL_STOP_NAME] [NUMBER_OF_RESULTS_TO_SHOW]
```

# Features I'd like to build

- [x] Only show next departures
- [x] Ask for depature stop, using inquire
- [ ] Ask for arival stop
- [ ] Plan possible return trips
- [x] switch ClockTime to time's crates implementation
