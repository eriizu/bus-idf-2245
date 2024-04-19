use crate::operating_flags::Runs;
use csv::StringRecord;

pub fn time_of_year_from_record(rec: &StringRecord) -> Vec<Runs> {
    time_of_year_from_str_iter(rec.iter())
}

pub fn time_of_year_from_str_iter<'a>(iter: impl Iterator<Item = &'a str>) -> Vec<Runs> {
    iter.skip(1).map(time_of_year_flag_from_str).collect()
}

fn time_of_year_flag_from_str(value: &str) -> Runs {
    match value {
        "A" => Runs::OUTSIDE_HOLIDAYS | Runs::HOLIDAYS,
        "SC" => Runs::OUTSIDE_HOLIDAYS,
        "V" => Runs::HOLIDAYS,
        _ => Runs::NEVER,
    }
}

pub fn operating_flags_from_iter<'a>(iter: impl Iterator<Item = &'a str>) -> Vec<Runs> {
    iter.map(time_of_year_and_day_flag_from_str).collect()
}

fn time_of_year_and_day_flag_from_str(value: &str) -> Runs {
    match value {
        "A" => Runs::ALL_YEAR,
        "SC" => Runs::OUTSIDE_HOLIDAYS,
        "V" => Runs::HOLIDAYS,
        "LàV" => Runs::WORKDAYS,
        "LMJV" => Runs::MONDAY | Runs::TUESDAY | Runs::THURSDAY | Runs::FRIDAY,
        "S" => Runs::SATURDAY,
        "D" => Runs::SUNDAY,
        "Me" => Runs::WEDNESDAY,
        _ => Runs::NEVER,
    }
}

fn days_flag_from_str(value: &str) -> Runs {
    match value {
        "LàV" => Runs::WORKDAYS,
        "LMJV" => Runs::MONDAY | Runs::TUESDAY | Runs::THURSDAY | Runs::FRIDAY,
        "S" => Runs::SATURDAY,
        "D" => Runs::SUNDAY,
        "Me" => Runs::WEDNESDAY,
        _ => Runs::NEVER,
    }
}

pub fn day_of_operation_from_record(rec: &StringRecord) -> Vec<Runs> {
    day_of_operation_from_str_iter(rec.iter())
}

pub fn day_of_operation_from_str_iter<'a>(iter: impl Iterator<Item = &'a str>) -> Vec<Runs> {
    iter.skip(1).map(days_flag_from_str).collect()
}
