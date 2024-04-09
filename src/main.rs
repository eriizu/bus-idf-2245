use std::{error::Error, process};

mod operating_flags;
use operating_flags::OperatingFlags;
mod parser_mlv;

mod clock_time;
use clock_time::ClockTime;

#[derive(Debug)]
struct Stop {
    time: ClockTime,
    stop_idx: usize,
}

#[derive(Debug)]
struct Journey {
    oparates: OperatingFlags,
    stops: Vec<Stop>,
}

impl Journey {
    pub fn new() -> Self {
        Self {
            oparates: OperatingFlags::NEVER,
            stops: vec![],
        }
    }
}

struct TimeTable {
    journeys: Vec<Journey>,
    stop_names: Vec<String>,
}

impl TimeTable {
    fn pretty_print(&self) {
        self.journeys.iter().for_each(|journey| {
            println!("\njourney {}", journey.oparates);
            journey.stops.iter().for_each(|stop| {
                println!(
                    "{:02}:{:02} {} {}",
                    stop.time.get_hours(),
                    stop.time.get_minutes(),
                    stop.stop_idx,
                    self.stop_names[stop.stop_idx]
                )
            })
        });
    }
}

fn example_journey_operating_flags() -> Result<(), Box<dyn Error>> {
    OperatingFlags::print_all();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(std::io::stdin());
    let record = rdr.records().next().unwrap().unwrap();
    let time_of_year: Vec<OperatingFlags> = parser_mlv::time_of_year_from_record(&record);

    let record = rdr.records().next().unwrap().unwrap();
    let days: Vec<OperatingFlags> = parser_mlv::day_of_operation_from_record(&record);

    time_of_year
        .iter()
        .zip(days.iter())
        .map(|(a, b)| *a | *b)
        .enumerate()
        .for_each(|(idx, val)| println!("{:2}: {:09b}", idx, val));
    Ok(())
}

fn get_initial_timetable_from_first_line<'a>(
    cols: impl Iterator<Item = &'a str>,
) -> Result<TimeTable, Box<dyn Error>> {
    let mut timetable = TimeTable {
        journeys: vec![],
        stop_names: vec![],
    };

    let flags = parser_mlv::operating_flags_from_iter(cols);
    flags.iter().for_each(|flags| {
        timetable.journeys.push(Journey {
            oparates: *flags,
            stops: vec![],
        });
    });
    Ok(timetable)
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(std::io::stdin());

    let mut records = rdr.records();
    let mut timetable = match records.next() {
        Some(Ok(record)) => {
            let cols = record.iter();
            get_initial_timetable_from_first_line(cols)?
        }
        Some(Err(err)) => return Err(Box::new(err)),
        _ => {
            eprintln!("no lines to process");
            return Ok(());
        }
    };

    for result in records {
        let record = result?;
        let mut cols = record.iter();
        let Some(first_col) = cols.next() else {
            eprintln!("ignoring line, no columns");
            continue;
        };
        if first_col.is_empty() {
            let flags = parser_mlv::operating_flags_from_iter(cols);
            flags
                .iter()
                .zip(timetable.journeys.iter_mut())
                .for_each(|(flags, journey)| {
                    journey.oparates |= *flags;
                });
        } else {
            let stop_name = first_col;

            let stop_name_idx = match timetable
                .stop_names
                .iter()
                .enumerate()
                .find(|(_, elem)| elem.as_str() == stop_name)
                .map(|(a, _)| a)
            {
                Some(idx) => idx,
                None => {
                    timetable.stop_names.push(stop_name.to_owned());
                    timetable.stop_names.len() - 1
                }
            };

            timetable.stop_names.push(stop_name.to_owned());
            // TODO: add stop names
            cols.zip(timetable.journeys.iter_mut())
                .for_each(|(col, journey)| {
                    if let Some(time) = ClockTime::from_str(col) {
                        journey.stops.push(Stop {
                            time,
                            stop_idx: stop_name_idx,
                        });
                    }
                });
        }
    }
    // timetable
    //     .journeys
    //     .iter()
    //     .for_each(|elem| println!("{:?}", elem));
    timetable.pretty_print();
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
