use std::error::Error;

use bus_20240330::{parse::parse_files, runs::runs_on_date};

fn main() -> Result<(), Box<dyn Error>> {
    let timetable = match parse_files() {
        Ok(tt) => tt,
        Err(err) => return Err(err),
    };

    // timetable.pretty_print();
    // let now = time::OffsetDateTime::now_local().unwrap();
    // let today = now.date();
    let today = time::macros::date!(2024 - 04 - 21);
    println!("{today}");
    timetable
        .journeys
        .iter()
        .filter(|journey| {
            let res = runs_on_date(&today, journey.oparates);
            // println!("{}: {}", res, journey.oparates);
            res
        })
        .for_each(|journey| {
            journey.pretty_print(&timetable.stop_names);
            println!("\n##########\n");
        });
    Ok(())
}
