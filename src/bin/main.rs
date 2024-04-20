use std::error::Error;

use bus_20240330::{
    reader::{reader_from_bytes_included, reader_from_bytes_included1},
    runs::Runs,
    timetable::TimeTable,
    *,
};

fn timetable_from_records(
    mut records: impl Iterator<Item = csv::Result<csv::StringRecord>>,
) -> Result<TimeTable, Box<dyn Error>> {
    let record = match records.next() {
        Some(Ok(record)) => record,
        Some(Err(err)) => return Err(Box::new(err)),
        None => {
            panic!("No more lines, this only happens on empty files, this program shoudn't be packages with an empty file.");
        }
    };

    let cols = record.iter();
    let flags = cols.skip(1).map(Runs::from_str);
    let mut timetable = TimeTable::new_from_flags(flags);

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
    Ok(timetable)
}

fn add_to_timetable(
    timetable: &mut TimeTable,
    mut records: impl Iterator<Item = csv::Result<csv::StringRecord>>,
) -> Result<(), Box<dyn Error>> {
    let record = match records.next() {
        Some(Ok(record)) => record,
        Some(Err(err)) => return Err(Box::new(err)),
        None => {
            panic!("No more lines, this only happens on empty files, this program shoudn't be packages with an empty file.");
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = reader_from_bytes_included();

    let records = reader.records();
    let mut timetable = match timetable_from_records(records) {
        Ok(timetable) => timetable,
        Err(err) => return Err(err),
    };

    let mut reader = reader_from_bytes_included1();
    let records = reader.records();
    match add_to_timetable(&mut timetable, records) {
        Err(err) => return Err(err),
        _ => {}
    };

    timetable.pretty_print();
    println!("\n##########\n");
    println!("\n##########\n");
    let now = time::OffsetDateTime::now_local().unwrap();
    let today = now.date();
    println!("{today}");
    timetable
        .journeys
        .iter()
        .filter(|journey| {
            let res = runs::runs_on_date(&today, journey.oparates);
            println!("{}: {}", res, journey.oparates);
            res
        })
        .for_each(|journey| {
            journey.pretty_print(&timetable.stop_names);
            println!("\n##########\n");
        });
    timetable
        .journeys
        .last()
        .unwrap()
        .pretty_print(&timetable.stop_names);
    Ok(())
}
