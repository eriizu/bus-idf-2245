use bus_20240330::*;

fn print_is_inside(start: &time::Date, end: &time::Date, x: &time::Date) {
    print!("{} is ", x);
    if !(start < x && x < end) {
        print!("not ");
    }
    println!("inside {} and {}", start, end);
}

fn main() {
    println!("explore time");
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
