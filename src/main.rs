use std::{error::Error, process};

mod operating_flags;
use operating_flags::OperatingFlags;
mod parser_mlv;

mod clock_time;
use clock_time::ClockTime;

mod timetable;
use timetable::{Journey, Stop, TimeTable};

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
    let flags = parser_mlv::operating_flags_from_iter(cols);
    let tt = TimeTable::new_from_flags(flags.iter().map(|flag_ref| *flag_ref));
    Ok(tt)
}

fn reader_from_stdin() -> csv::Reader<std::io::Stdin> {
    csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(std::io::stdin())
}

fn reader_from_bytes_included() -> csv::Reader<&'static [u8]> {
    let bytes = include_bytes!("../timetable_bus_2245.csv");
    csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(bytes)
}

fn example() -> Result<(), Box<dyn Error>> {
    // TODO: read from injected file as a string
    // https://doc.rust-lang.org/std/macro.include_str.html
    // https://docs.rs/csv/latest/csv/struct.ReaderBuilder.html#method.new
    // INFO: the goal is to be able to ship this as one binary
    let mut reader = reader_from_bytes_included();

    let mut records = reader.records();
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
            parse_and_add_operating_flags(cols, &mut timetable);
        } else {
            extract_and_add_stop_names(first_col, &mut timetable, cols);
        }
    }
    // timetable
    //     .journeys
    //     .iter()
    //     .for_each(|elem| println!("{:?}", elem));
    timetable.pretty_print();
    Ok(())
}

fn extract_and_add_stop_names(
    stop_name: &str,
    timetable: &mut TimeTable,
    cols: csv::StringRecordIter,
) {
    let stop_name_idx = timetable.add_or_get_stop_id(stop_name);
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

fn parse_and_add_operating_flags(cols: csv::StringRecordIter, timetable: &mut TimeTable) {
    let flags = parser_mlv::operating_flags_from_iter(cols);
    flags
        .iter()
        .zip(timetable.journeys.iter_mut())
        .for_each(|(flags, journey)| {
            journey.oparates |= *flags;
        });
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
