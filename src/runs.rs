use std::fmt::Display;

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Runs: u16 {
        const NEVER = 0;
        const OUTSIDE_HOLIDAYS = 0b00000001;
        const HOLIDAYS =  0b1 << 1;
        const MONDAY =    0b1 << 2;
        const TUESDAY =   0b1 << 3;
        const WEDNESDAY = 0b1 << 4;
        const THURSDAY =  0b1 << 5;
        const FRIDAY =    0b1 << 6;
        const SATURDAY =  0b1 << 7;
        const SUNDAY =    0b1 << 8;

        const ALL_YEAR = 0b11;
        const WORKDAYS = 0b11111 << 2;
        const WEEKENDS = 0b11 << 7;
    }
}

impl Runs {
    pub fn from_str(value: &str) -> Self {
        match value {
            "A" => Runs::ALL_YEAR,
            "SC" => Runs::OUTSIDE_HOLIDAYS,
            "V" => Runs::HOLIDAYS,
            "LàV" => Runs::WORKDAYS,
            "LMJV" => Runs::MONDAY | Runs::TUESDAY | Runs::THURSDAY | Runs::FRIDAY,
            "S" => Runs::SATURDAY,
            "D" => Runs::SUNDAY,
            "Me" => Runs::WEDNESDAY,
            _ => Runs::NEVER,
        }
    }
}

impl Display for Runs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Self::NEVER {
            return write!(f, "Never");
        }
        let time_of_year = match *self {
            a if a.contains(Self::ALL_YEAR) => "All year",
            a if a.contains(Self::OUTSIDE_HOLIDAYS) => "Outside Holidays",
            a if a.contains(Self::HOLIDAYS) => "Holidays",
            _ => "",
        };
        let mut temp = String::new();
        let days = match *self {
            a if a.contains(Self::WORKDAYS) => "Workdays",
            a if a.contains(Self::WEEKENDS) => "Weekends",
            a => {
                if a.contains(Runs::MONDAY) {
                    temp += "Monday ";
                }
                if a.contains(Runs::TUESDAY) {
                    temp += "Tuesday ";
                }
                if a.contains(Runs::WEDNESDAY) {
                    temp += "Wednesday ";
                }
                if a.contains(Runs::THURSDAY) {
                    temp += "Thursday ";
                }
                if a.contains(Runs::FRIDAY) {
                    temp += "Friday ";
                }
                if a.contains(Runs::SATURDAY) {
                    temp += "Saturday ";
                }
                if a.contains(Runs::SUNDAY) {
                    temp += "Sunday";
                }
                temp.as_str()
            }
        };
        write!(f, "{} {}", time_of_year, days)
    }
}

struct DateRange {
    start: time::Date,
    end: time::Date,
    // flag: OperatingFlags,
}

const HOLIDAY_RANGES: [DateRange; 2] = [
    DateRange {
        start: time::macros::date!(2024 - 04 - 06),
        end: time::macros::date!(2024 - 04 - 22),
    },
    DateRange {
        start: time::macros::date!(2024 - 06 - 06),
        end: time::macros::date!(2024 - 09 - 02),
    },
];

const BANK_HOLIDAYS: [time::Date; 6] = [
    time::macros::date!(2024 - 05 - 01),
    time::macros::date!(2024 - 05 - 08),
    time::macros::date!(2024 - 05 - 09),
    time::macros::date!(2024 - 05 - 20),
    time::macros::date!(2024 - 07 - 14),
    time::macros::date!(2024 - 08 - 15),
];

pub fn runs_on_date(date: &time::Date, flags: Runs) -> bool {
    if !flags.contains(Runs::ALL_YEAR) {
        let is_holiday = HOLIDAY_RANGES
            .iter()
            .find(|range| range.start < *date && *date < range.end)
            .is_some();
        if is_holiday != flags.contains(Runs::HOLIDAYS) {
            return false;
        }
    }
    let is_bank = BANK_HOLIDAYS
        .iter()
        .find(|bank_holiday| date == *bank_holiday)
        .is_some();
    if is_bank && flags.contains(Runs::SUNDAY) {
        return true;
    }
    return match date.weekday() {
        time::Weekday::Monday if flags.contains(Runs::MONDAY) => true,
        time::Weekday::Tuesday if flags.contains(Runs::TUESDAY) => true,
        time::Weekday::Wednesday if flags.contains(Runs::WEDNESDAY) => true,
        time::Weekday::Thursday if flags.contains(Runs::THURSDAY) => true,
        time::Weekday::Friday if flags.contains(Runs::FRIDAY) => true,
        time::Weekday::Saturday if flags.contains(Runs::SATURDAY) => true,
        time::Weekday::Sunday if flags.contains(Runs::SUNDAY) => true,
        _ => false,
    };
}

pub fn runs_on(weekday: &time::Weekday, is_bank: bool, is_holiday: bool, flags: Runs) -> bool {
    if !flags.contains(Runs::ALL_YEAR) {
        return (is_holiday && flags.contains(Runs::HOLIDAYS))
            || (!is_holiday && flags.contains(Runs::OUTSIDE_HOLIDAYS));
    } else {
        if is_bank && flags.contains(Runs::SUNDAY) {
            return true;
        }
        return match weekday {
            time::Weekday::Monday if flags.contains(Runs::MONDAY) => true,
            time::Weekday::Tuesday if flags.contains(Runs::TUESDAY) => true,
            time::Weekday::Wednesday if flags.contains(Runs::WEDNESDAY) => true,
            time::Weekday::Thursday if flags.contains(Runs::THURSDAY) => true,
            time::Weekday::Friday if flags.contains(Runs::FRIDAY) => true,
            time::Weekday::Saturday if flags.contains(Runs::SATURDAY) => true,
            time::Weekday::Sunday if flags.contains(Runs::SUNDAY) => true,
            _ => false,
        };
    }
}
