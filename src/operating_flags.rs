use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct OperatingFlags: u16 {
        const NEVER = 0;
        const NOT_HOLIDAYS = 0b00000001;
        const HOLIDAYS = 0b1 << 1;
        const MONDAY = 0b1 << 2;
        const TUESDAY = 0b1 << 3;
        const WEDNESDAY = 0b1 << 4;
        const THURSDAY = 0b1 << 5;
        const FRIDAY = 0b1 << 6;
        const SATURDAY = 0b1 << 7;
        const SUNDAY = 0b1 << 8;
    }
}

impl OperatingFlags {
    pub fn print_all() {
        println!("{:09b}", OperatingFlags::NOT_HOLIDAYS);
        println!("{:09b}", OperatingFlags::HOLIDAYS);
        println!("{:09b}", OperatingFlags::MONDAY);
        println!("{:09b}", OperatingFlags::TUESDAY);
        println!("{:09b}", OperatingFlags::WEDNESDAY);
        println!("{:09b}", OperatingFlags::THURSDAY);
        println!("{:09b}", OperatingFlags::FRIDAY);
        println!("{:09b}", OperatingFlags::SATURDAY);
        println!("{:09b}", OperatingFlags::SUNDAY);
    }
}
