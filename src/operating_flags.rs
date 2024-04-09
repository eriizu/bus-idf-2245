use std::fmt::Display;

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct OperatingFlags: u16 {
        const NEVER = 0;
        const OUTSIDE_HOLIDAYS = 0b00000001;
        const HOLIDAYS = 0b1 << 1;
        const MONDAY = 0b1 << 2;
        const TUESDAY = 0b1 << 3;
        const WEDNESDAY = 0b1 << 4;
        const THURSDAY = 0b1 << 5;
        const FRIDAY = 0b1 << 6;
        const SATURDAY = 0b1 << 7;
        const SUNDAY = 0b1 << 8;

        const ALL_YEAR = 0b11;
        const WORKDAYS = 0b11111 << 2;
        const WEEKENDS = 0b11 << 7;
    }
}

impl OperatingFlags {
    pub fn print_all() {
        println!("{:09b}", OperatingFlags::OUTSIDE_HOLIDAYS);
        println!("{:09b}", OperatingFlags::HOLIDAYS);
        println!("{:09b}", OperatingFlags::MONDAY);
        println!("{:09b}", OperatingFlags::TUESDAY);
        println!("{:09b}", OperatingFlags::WEDNESDAY);
        println!("{:09b}", OperatingFlags::THURSDAY);
        println!("{:09b}", OperatingFlags::FRIDAY);
        println!("{:09b}", OperatingFlags::SATURDAY);
        println!("{:09b}", OperatingFlags::SUNDAY);
    }
}

impl Display for OperatingFlags {
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
                if a.contains(OperatingFlags::MONDAY) {
                    temp += "Monday ";
                }
                if a.contains(OperatingFlags::TUESDAY) {
                    temp += "Tuesday ";
                }
                if a.contains(OperatingFlags::WEDNESDAY) {
                    temp += "Wednesday ";
                }
                if a.contains(OperatingFlags::THURSDAY) {
                    temp += "Thursday ";
                }
                if a.contains(OperatingFlags::FRIDAY) {
                    temp += "Friday ";
                }
                if a.contains(OperatingFlags::SATURDAY) {
                    temp += "Saturday ";
                }
                if a.contains(OperatingFlags::SUNDAY) {
                    temp += "Sunday";
                }
                temp.as_str()
            }
        };
        write!(f, "{} {}", time_of_year, days)
    }
}
