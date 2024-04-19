use bus_20240330::*;

struct DateRange {
    start: time::Date,
    end: time::Date,
    // flag: operating_flags::OperatingFlags,
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

fn get_time_of_year(date: &time::Date) -> operating_flags::OperatingFlags {
    let is_holiday = HOLIDAY_RANGES
        .iter()
        .find(|range| range.start < *date && *date < range.end)
        .is_some();
    if is_holiday {
        return operating_flags::OperatingFlags::HOLIDAYS;
    }
    return operating_flags::OperatingFlags::OUTSIDE_HOLIDAYS;
}

fn get_operating_flag_for(date: &time::Date) -> operating_flags::OperatingFlags {
    let is_bank = BANK_HOLIDAYS
        .iter()
        .find(|bank_holiday| date == *bank_holiday)
        .is_some();
    if is_bank {
        return operating_flags::OperatingFlags::SUNDAY;
    }
    match date.weekday() {
        time::Weekday::Monday => operating_flags::OperatingFlags::MONDAY,
        time::Weekday::Tuesday => operating_flags::OperatingFlags::TUESDAY,
        time::Weekday::Wednesday => operating_flags::OperatingFlags::WEDNESDAY,
        time::Weekday::Thursday => operating_flags::OperatingFlags::THURSDAY,
        time::Weekday::Friday => operating_flags::OperatingFlags::FRIDAY,
        time::Weekday::Saturday => operating_flags::OperatingFlags::FRIDAY,
        time::Weekday::Sunday => operating_flags::OperatingFlags::SUNDAY,
    }
}

fn print_is_inside(start: &time::Date, end: &time::Date, x: &time::Date) {
    print!("{} is ", x);
    if !(start < x && x < end) {
        print!("not ");
    }
    println!("inside {} and {}", start, end);
}

fn main() {
    println!("explore time {}", HOLIDAY_RANGES[0].start);
    let flag = operating_flags::OperatingFlags::WORKDAYS;
    let start = time::Date::from_calendar_date(2024, time::Month::April, 15).unwrap();
    let end = time::Date::from_calendar_date(2024, time::Month::April, 28).unwrap();
    println!("start {}", start);
    println!("end {}", end);
    let inside = time::Date::from_calendar_date(2024, time::Month::April, 22).unwrap();
    let outside = time::Date::from_calendar_date(2024, time::Month::April, 29).unwrap();
    print_is_inside(&start, &end, &outside);
    print_is_inside(&start, &end, &inside);
}
