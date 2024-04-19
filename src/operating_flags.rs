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
    pub fn todays() {}
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
