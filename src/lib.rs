pub mod clock_time;
pub mod timetable;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize)]
struct Opt {
    depart_from: Option<String>,
    number_to_show: Option<usize>,
}

use fuse_rust::{Fuse, SearchResult};

fn get_departure_stop<'a>(opt: &Opt, stops: Vec<&'a str>) -> Option<String> {
    dbg!(&stops);
    if let Some(depart_from) = &opt.depart_from {
        let fuse = Fuse::default();
        let results = fuse.search_text_in_iterable(depart_from, stops.iter());
        // return None;
        let max = results
            .iter()
            .reduce(|acc, item| if item.score < acc.score { item } else { acc });
        if let Some(max) = max {
            println!("{} max: {}", depart_from, stops[max.index]);
            Some(stops[max.index].to_owned())
        } else {
            None
        }
    } else {
        use inquire::{error::InquireError, Select};

        let ans: Result<&str, InquireError> = Select::new("Depart from?", stops).prompt();
        if let Ok(ans) = ans {
            Some(ans.to_owned())
        } else {
            None
        }
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();
    let tt = timetable::parse::parse_files()?;
    let Some(depart_from) = get_departure_stop(
        &opt,
        tt.stop_names.iter().map(|item| item.as_str()).collect(),
    ) else {
        // TODO: actually return an error
        return Ok(());
    };
    let take = opt.number_to_show.unwrap_or(3);
    // let datetime = time::OffsetDateTime::now_local().unwrap();
    let datetime = time::macros::datetime!(2024 - 05 - 06 08:00);
    let today = datetime.date();
    let now = datetime.time();
    print_next_buses(tt, depart_from.as_str(), today, now, take);
    Ok(())
}

fn print_next_buses(
    tt: timetable::TimeTable,
    start_stop_name: &str,
    day: time::Date,
    clock_time: time::Time,
    take: usize,
) {
    let start_stop_id = tt.get_stop_id(start_stop_name).unwrap();
    tt.journeys
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
            let time = clock_time - time::Duration::minutes(10);
            timetable::runs::runs_on_date(&day, journey.oparates) && stop.time > time
        })
        .take(take)
        .for_each(|(journey, stop)| {
            if stop.time < clock_time {
                println!("(should have already departed)");
            }
            journey.pretty_print_from_stop_id(&tt.stop_names, start_stop_id);
            println!("\n");
        });
}
