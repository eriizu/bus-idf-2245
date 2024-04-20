pub fn reader_from_stdin() -> csv::Reader<std::io::Stdin> {
    csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(std::io::stdin())
}

pub fn reader_from_bytes(bytes: &'static [u8]) -> csv::Reader<&'static [u8]> {
    csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(bytes)
}

pub fn reader_from_bytes_included() -> csv::Reader<&'static [u8]> {
    let bytes = include_bytes!("../timetable_bus_2245w1.csv");
    csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(bytes)
}

pub fn reader_from_bytes_included1() -> csv::Reader<&'static [u8]> {
    let bytes = include_bytes!("../timetable_bus_2245w2.csv");
    csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(bytes)
}
