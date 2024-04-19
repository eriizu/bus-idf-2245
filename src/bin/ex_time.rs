use bus_20240330::{operating_flags::Runs, *};

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

fn get_time_of_year(date: &time::Date) -> Runs {
    let is_holiday = HOLIDAY_RANGES
        .iter()
        .find(|range| range.start < *date && *date < range.end)
        .is_some();
    if is_holiday {
        return Runs::HOLIDAYS;
    }
    return Runs::OUTSIDE_HOLIDAYS;
}

fn get_operating_flag_for(date: &time::Date) -> Runs {
    let is_bank = BANK_HOLIDAYS
        .iter()
        .find(|bank_holiday| date == *bank_holiday)
        .is_some();
    if is_bank {
        return Runs::SUNDAY;
    }
    match date.weekday() {
        time::Weekday::Monday => Runs::MONDAY,
        time::Weekday::Tuesday => Runs::TUESDAY,
        time::Weekday::Wednesday => Runs::WEDNESDAY,
        time::Weekday::Thursday => Runs::THURSDAY,
        time::Weekday::Friday => Runs::FRIDAY,
        time::Weekday::Saturday => Runs::FRIDAY,
        time::Weekday::Sunday => Runs::SUNDAY,
    }
}

fn runs_on_date(date: &time::Date, flags: Runs) -> bool {
    if !flags.contains(Runs::ALL_YEAR) {
        let is_holiday = HOLIDAY_RANGES
            .iter()
            .find(|range| range.start < *date && *date < range.end)
            .is_some();
        return (is_holiday && flags.contains(Runs::HOLIDAYS))
            || (!is_holiday && flags.contains(Runs::OUTSIDE_HOLIDAYS));
    } else {
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
}

fn runs_on(weekday: &time::Weekday, is_bank: bool, is_holiday: bool, flags: Runs) -> bool {
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

fn print_is_inside(start: &time::Date, end: &time::Date, x: &time::Date) {
    print!("{} is ", x);
    if !(start < x && x < end) {
        print!("not ");
    }
    println!("inside {} and {}", start, end);
}

fn main() {
    println!("explore time {}", HOLIDAY_RANGES[0].start);
    let flag = Runs::WORKDAYS;
    let start = time::Date::from_calendar_date(2024, time::Month::April, 15).unwrap();
    let end = time::Date::from_calendar_date(2024, time::Month::April, 28).unwrap();
    println!("start {}", start);
    println!("end {}", end);
    let inside = time::Date::from_calendar_date(2024, time::Month::April, 22).unwrap();
    let outside = time::Date::from_calendar_date(2024, time::Month::April, 29).unwrap();
    print_is_inside(&start, &end, &outside);
    print_is_inside(&start, &end, &inside);
}
