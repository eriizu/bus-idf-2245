use std::{error::Error, io, process};

use csv::StringRecord;

mod operating_flags;
use operating_flags::OperatingFlags;
mod parser_mlv;

struct Journey {
    oparates: OperatingFlags,
    stops: Vec<String>,
}

impl Journey {
    pub fn new() -> Self {
        Self {
            oparates: OperatingFlags::NEVER,
            stops: vec![],
        }
    }
}

fn example() -> Result<(), Box<dyn Error>> {
    OperatingFlags::print_all();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(io::stdin());
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
    // for result in rdr.records() {
    //     let record = result?;
    //     //println!("{:?}", record);
    //     record.iter().for_each(|elem| print!("{} ", elem));
    //     println!();
    // }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
