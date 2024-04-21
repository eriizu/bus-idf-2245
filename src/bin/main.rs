use std::error::Error;

use bus_20240330::timetable::{parse::parse_files, runs::runs_on_date, Journey, TimeTable};

fn mk_filter_for_stop(
    timetable: &TimeTable,
    stop_name: &str,
    today: time::Date,
    now: time::Time,
) -> Option<Box<dyn Fn(&Journey) -> bool>> {
    let Some(stop_id) = timetable
        .stop_names
        .iter()
        .enumerate()
        .find_map(
            |(idx, stop)| {
                if *stop == stop_name {
                    Some(idx)
                } else {
                    None
                }
            },
        )
    else {
        return None;
    };
    let fun = move |journey: &Journey| {
        if let Some(stop) = journey.stops.iter().find(|stop| stop.stop_idx == stop_id) {
            runs_on_date(&today, journey.oparates) && stop.time > now
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

    // let now_date = time::OffsetDateTime::now_local().unwrap();
    let now_date = time::macros::datetime!(2024 - 04 - 22 14:50);
    let today = now_date.date();
    let now_time = now_date.time();
    println!("next bus:");
    let work_stop_id = timetable
        .stop_names
        .iter()
        .enumerate()
        .find_map(|(idx, stop)| {
            if *stop == "Parc du Bel-Air" {
                Some(idx)
            } else {
                None
            }
        })
        .unwrap();
    timetable
        .journeys
        .iter()
        .filter(|journey| {
            if let Some(stop) = journey
                .stops
                .iter()
                .find(|stop| stop.stop_idx == work_stop_id)
            {
                runs_on_date(&today, journey.oparates) && stop.time > now_time
            } else {
                false
            }
        })
        .for_each(|journey| {
            journey.pretty_print(&timetable.stop_names);
            println!("\n");
        });
    Ok(())
}
