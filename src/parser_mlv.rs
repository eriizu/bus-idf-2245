use crate::operating_flags::OperatingFlags;
use csv::StringRecord;

pub fn time_of_year_from_record(rec: &StringRecord) -> Vec<OperatingFlags> {
    rec.iter()
        .skip(1)
        .map(|txt| match txt {
            "A" => OperatingFlags::NOT_HOLIDAYS | OperatingFlags::HOLIDAYS,
            "SC" => OperatingFlags::NOT_HOLIDAYS,
            "V" => OperatingFlags::HOLIDAYS,
            _ => OperatingFlags::NEVER,
        })
        .collect()
}

pub fn day_of_operation_from_record(rec: &StringRecord) -> Vec<OperatingFlags> {
    rec.iter()
        .skip(1)
        .map(|txt| match txt {
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
        })
        .collect()
}
