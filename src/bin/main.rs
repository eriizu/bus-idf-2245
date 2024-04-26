use std::error::Error;

use bus_20240330::timetable::{parse::parse_files, runs::runs_on_date, Journey};

fn mk_filter_for_stop(
    stop_id: usize,
    today: time::Date,
    now: time::Time,
) -> Option<Box<dyn Fn(&Journey) -> bool>> {
    let time = now - time::Duration::minutes(10);
    let fun = move |journey: &Journey| {
        if let Some(stop) = journey.stops.iter().find(|stop| stop.stop_idx == stop_id) {
            runs_on_date(&today, journey.oparates) && stop.time > time
        } else {
            false
        }
    };

    return Some(Box::new(fun));
}

fn main() -> Result<(), Box<dyn Error>> {
    let timetable = match parse_files() {
        Ok(tt) => tt,
        Err(err) => return Err(err),
    };

    use inquire::{error::InquireError, Select};

    let ans: Result<&str, InquireError> = Select::new(
        "Depart from?",
        timetable
            .stop_names
            .iter()
            .map(|name| name.as_str())
            .collect(),
    )
    .prompt();
    let start_stop = ans.unwrap();
    let start_stop_id = timetable.get_stop_id(start_stop).unwrap();
    let now_date = time::OffsetDateTime::now_local().unwrap();
    let now_date = time::macros::datetime!(2024 - 04 - 24 17:26);
    let today = now_date.date();
    let now_time = now_date.time();
    // let filter = mk_filter_for_stop(start_stop_id, today, now_time).unwrap();
    timetable
        .journeys
        .iter()
        .filter_map(|journey| {
            if let Some(stop) = journey
                .stops
                .iter()
                .find(|stop| stop.stop_idx == start_stop_id)
            {
                Some((journey, stop))
            } else {
                None
            }
        })
        .filter(|(journey, stop)| {
            let time = now_time - time::Duration::minutes(10);
            runs_on_date(&today, journey.oparates) && stop.time > time
        })
        .take(3)
        .for_each(|(journey, stop)| {
            if stop.time < now_time {
                println!("(should have already departed)");
            }
            journey.pretty_print_from_stop_id(&timetable.stop_names, start_stop_id);
            println!("\n");
        });
    Ok(())
}
