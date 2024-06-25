pub mod clock_time;
pub mod timetable;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize)]
struct Opt {
    depart_from: Option<String>,
    number_to_show: Option<usize>,

    #[arg(short, long, default_value_t = false)]
    long: bool,
}

pub fn run() {
    let opt = Opt::parse();
    let tt = timetable::parse::parse_files().expect("parsing internal timetable");
    let Some(depart_from) = get_departure_stop(
        &opt,
        tt.stop_names.iter().map(|item| item.as_str()).collect(),
    ) else {
        eprintln!("failed to ask or match stop name");
        return;
    };
    let take = opt.number_to_show.unwrap_or(3);
    let datetime = time::OffsetDateTime::now_local().expect("getting current date and time");
    // INFO: for testing, test on specific date using:
    // let datetime = time::macros::datetime!(2024 - 05 - 06 08:00);
    let today = datetime.date();
    let now = datetime.time();
    print_next_buses_times(tt, depart_from.as_str(), today, now, take, opt.long);
}

fn print_next_buses_times(
    tt: timetable::TimeTable,
    start_stop_name: &str,
    day: time::Date,
    clock_time: time::Time,
    take: usize,
    long: bool,
) {
    let start_stop_id = tt
        .get_stop_id(start_stop_name)
        .expect("geting stop by id (should already be checked)");
    println!("departures from \"{}\":", start_stop_name);
    let iter = tt
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
            let time = clock_time - time::Duration::minutes(10);
            timetable::runs::runs_on_date(&day, journey.operates) && stop.time > time
        })
        .take(take);

    let mut processed: usize = 0;
    if !long {
        iter.for_each(|(_, stop)| {
            processed += 1;
            if stop.time < clock_time {
                print!("{:02}:{:02} (due), ", stop.time.hour(), stop.time.minute());
            } else {
                print!("{:02}:{:02}, ", stop.time.hour(), stop.time.minute());
            }
        });
        if processed > 0 {
            println!("...");
        }
    } else {
        iter.for_each(|(journey, stop)| {
            processed += 1;
            if stop.time < clock_time {
                println!("(should have already departed)");
            }
            journey.pretty_print_from_stop_id(&tt.stop_names, start_stop_id);
            println!("\n");
        });
    }
    if processed == 0 {
        println!("none :(");
    }
}

fn get_departure_stop<'a>(opt: &Opt, stops: Vec<&'a str>) -> Option<String> {
    if let Some(depart_from) = &opt.depart_from {
        get_best_matching_stop_name(depart_from, stops)
    } else {
        ask_for_deperture_stop(stops)
    }
}

fn ask_for_deperture_stop(stops: Vec<&str>) -> Option<String> {
    use inquire::{error::InquireError, Select};

    let ans: Result<&str, InquireError> = Select::new("Depart from?", stops).prompt();
    if let Ok(ans) = ans {
        Some(ans.to_owned())
    } else {
        None
    }
}

fn get_best_matching_stop_name<'a>(stop_name: &str, stops: Vec<&'a str>) -> Option<String> {
    use fuse_rust::Fuse;
    let fuse = Fuse::default();
    let results = fuse.search_text_in_iterable(stop_name, stops.iter());
    if let Some(best_result) =
        results
            .iter()
            .reduce(|acc, item| if item.score < acc.score { item } else { acc })
    {
        Some(stops[best_result.index].to_owned())
    } else {
        None
    }
}
