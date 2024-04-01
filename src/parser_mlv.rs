use crate::operating_flags::OperatingFlags;
use csv::StringRecord;

pub fn time_of_year_from_record(rec: &StringRecord) -> Vec<OperatingFlags> {
    time_of_year_from_str_iter(rec.iter())
}

pub fn time_of_year_from_str_iter<'a>(iter: impl Iterator<Item = &'a str>) -> Vec<OperatingFlags> {
    iter.skip(1)
        .map(|txt| time_of_year_flag_from_str(txt))
        .collect()
}

fn time_of_year_flag_from_str(value: &str) -> OperatingFlags {
    match value {
        "A" => OperatingFlags::OUTSIDE_HOLIDAYS | OperatingFlags::HOLIDAYS,
        "SC" => OperatingFlags::OUTSIDE_HOLIDAYS,
        "V" => OperatingFlags::HOLIDAYS,
        _ => OperatingFlags::NEVER,
    }
}

pub fn operating_flags_from_iter<'a>(iter: impl Iterator<Item = &'a str>) -> Vec<OperatingFlags> {
    iter.map(time_of_year_and_day_flag_from_str).collect()
}

fn time_of_year_and_day_flag_from_str(value: &str) -> OperatingFlags {
    match value {
        "A" => OperatingFlags::OUTSIDE_HOLIDAYS | OperatingFlags::HOLIDAYS,
        "SC" => OperatingFlags::OUTSIDE_HOLIDAYS,
        "V" => OperatingFlags::HOLIDAYS,
        "LàV" => {
            OperatingFlags::MONDAY
                | OperatingFlags::TUESDAY
                | OperatingFlags::WEDNESDAY
                | OperatingFlags::THURSDAY
                | OperatingFlags::FRIDAY
        }
        "LMJV" => {
            OperatingFlags::MONDAY
                | OperatingFlags::TUESDAY
                | OperatingFlags::THURSDAY
                | OperatingFlags::FRIDAY
        }
        "S" => OperatingFlags::SATURDAY,
        "D" => OperatingFlags::SUNDAY,
        "Me" => OperatingFlags::WEDNESDAY,
        _ => OperatingFlags::NEVER,
    }
}

fn days_flag_from_str(value: &str) -> OperatingFlags {
    match value {
        "LàV" => {
            OperatingFlags::MONDAY
                | OperatingFlags::TUESDAY
                | OperatingFlags::WEDNESDAY
                | OperatingFlags::THURSDAY
                | OperatingFlags::FRIDAY
        }
        "LMJV" => {
            OperatingFlags::MONDAY
                | OperatingFlags::TUESDAY
                | OperatingFlags::THURSDAY
                | OperatingFlags::FRIDAY
        }
        "S" => OperatingFlags::SATURDAY,
        "D" => OperatingFlags::SUNDAY,
        "Me" => OperatingFlags::WEDNESDAY,
        _ => OperatingFlags::NEVER,
    }
}

pub fn day_of_operation_from_record(rec: &StringRecord) -> Vec<OperatingFlags> {
    day_of_operation_from_str_iter(rec.iter())
}

pub fn day_of_operation_from_str_iter<'a>(
    iter: impl Iterator<Item = &'a str>,
) -> Vec<OperatingFlags> {
    iter.skip(1).map(|txt| days_flag_from_str(txt)).collect()
}
