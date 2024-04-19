use std::{error::Error, process};

use csv::StringRecord;

use bus_20240330::{
    clock_time::ClockTime,
    timetable::{Journey, Stop, TimeTable},
    *,
};

fn get_initial_timetable_from_first_line<'a>(
    cols: impl Iterator<Item = &'a str>,
) -> Result<TimeTable, Box<dyn Error>> {
    let flags = parser_mlv::operating_flags_from_iter(cols);
    let tt = TimeTable::new_from_flags(flags.iter().copied());
    Ok(tt)
}

fn reader_from_stdin() -> csv::Reader<std::io::Stdin> {
    csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(std::io::stdin())
}

fn reader_from_bytes_included() -> csv::Reader<&'static [u8]> {
    let bytes = include_bytes!("../../timetable_bus_2245.csv");
    csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(bytes)
}

fn timetable_from_records(
    mut records: impl Iterator<Item = csv::Result<csv::StringRecord>>,
) -> Result<TimeTable, Box<dyn Error>> {
    let mut timetable = match records.next() {
        Some(Ok(record)) => {
            let cols = record.iter();
            get_initial_timetable_from_first_line(cols)?
        }
        Some(Err(err)) => return Err(Box::new(err)),
        _ => {
            panic!("???");
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
    Ok(timetable)
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut reader = reader_from_bytes_included();

    let mut records = reader.records();
    match timetable_from_records(records) {
        Ok(timetable) => {
            timetable.pretty_print();
            Ok(())
        }
        Err(err) => Err(err),
    }
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
