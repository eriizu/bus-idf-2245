use bus_20240330::timetable::runs::Runs;

struct DateRange {
    start: time::Date,
    end: time::Date,
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

fn print_is_inside(start: &time::Date, end: &time::Date, x: &time::Date) {
    print!("{} is ", x);
    if !(start < x && x < end) {
        print!("not ");
    }
    println!("inside {} and {}", start, end);
}

fn main() {
    // let now = time::OffsetDateTime::now_utc();
    let now = time::OffsetDateTime::now_local().unwrap();

    println!("{now}");
    let today = now.date();
    println!("{today}");
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
