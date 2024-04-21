mod reader;
use super::{runs::Runs, TimeTable};
use reader::*;
use std::error::Error;

fn add_to_timetable(
    timetable: &mut TimeTable,
    mut records: impl Iterator<Item = csv::Result<csv::StringRecord>>,
) -> Result<(), Box<dyn Error>> {
    let record = match records.next() {
        Some(Ok(record)) => record,
        Some(Err(err)) => return Err(Box::new(err)),
        None => {
            panic!(
                "{}{}",
                "No more lines, this only happens on empty files,",
                "this program shoudn't be packages with an empty file."
            );
        }
    };

    let cols = record.iter();
    let flags = cols.skip(1).map(Runs::from_str);
    timetable.injest_flags_new_journeys(flags);

    for result in records {
        let record = result?;
        let mut cols = record.iter();
        let Some(first_col) = cols.next() else {
            eprintln!("ignoring line, no columns");
            continue;
        };
        if first_col.is_empty() {
            timetable.injest_parse_flags(cols);
        } else {
            timetable.injest_stops(first_col, cols);
        }
    }
    timetable.mark_complete();
    Ok(())
}

fn read_and_parse(timetable: &mut TimeTable, bytes: &'static [u8]) -> Result<(), Box<dyn Error>> {
    let mut reader = reader_from_bytes(bytes);

    let records = reader.records();
    add_to_timetable(timetable, records)
}

/// Parse the csv timetables integrated into this binary.
// TODO: factorise this funciton
pub fn parse_files() -> Result<TimeTable, Box<dyn Error>> {
    let mut timetable = TimeTable::new();
    read_and_parse(
        &mut timetable,
        include_bytes!("../../timetable_bus_2245w1.csv"),
    )?;
    read_and_parse(
        &mut timetable,
        include_bytes!("../../timetable_bus_2245w2.csv"),
    )?;
    read_and_parse(
        &mut timetable,
        include_bytes!("../../timetable_bus_2245we1.csv"),
    )?;
    read_and_parse(
        &mut timetable,
        include_bytes!("../../timetable_bus_2245we2.csv"),
    )?;
    Ok(timetable)
}
