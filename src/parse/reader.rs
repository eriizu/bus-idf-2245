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
