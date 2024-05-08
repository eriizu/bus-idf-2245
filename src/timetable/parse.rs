mod reader;
use super::{runs::Runs, TimeTable};
use reader::*;
use std::error::Error;

fn add_to_timetable(
    timetable: &mut TimeTable,
    records: impl Iterator<Item = csv::Result<csv::StringRecord>>,
) -> Result<(), Box<dyn Error>> {
    let mut iter = records.filter_map(|record_result| match record_result {
        Ok(record) => Some(record),
        _ => None,
    });
    iter.by_ref().take(1).for_each(|first_line| {
        let cols = first_line.iter();
        let flags = cols.skip(1).map(Runs::from_str);
        timetable.journeys_new_from_flags(flags);
    });
    iter.for_each(|line| {
        let mut cols = line.iter();
        let Some(stop_name) = cols.next() else {
            eprintln!("ignoring line, no columns");
            return;
        };
        if stop_name.is_empty() {
            let flags = cols.map(Runs::from_str);
            timetable.journeys_add_flags(flags);
        } else {
            timetable.journeys_add_stops(stop_name, cols);
        }
    });
    timetable.mark_complete();
    Ok(())
}

fn read_and_parse(timetable: &mut TimeTable, bytes: &'static [u8]) -> Result<(), Box<dyn Error>> {
    let mut reader = reader_from_bytes(bytes);

    let records = reader.records();
    add_to_timetable(timetable, records)
}

/// Parse the csv timetables integrated into this binary.
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
