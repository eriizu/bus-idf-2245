#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClockTime(u16);

impl ClockTime {
    pub fn new(hours: u16, minutes: u16) -> Option<Self> {
        if hours >= 24 || minutes >= 60 {
            None
        } else {
            Some(Self(hours * 60 + minutes))
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        let split = value.split_once(':')?;
        let hours: u16 = split.0.parse().ok()?;
        let minutes: u16 = split.1.parse().ok()?;
        Self::new(hours, minutes)
    }

    pub fn get_hours(&self) -> u16 {
        self.0 / 60
    }

    pub fn get_minutes(&self) -> u16 {
        self.0 % 60
    }

    pub fn with_hours(&self, value: u16) -> Option<Self> {
        if value < 24 {
            Some(Self((self.0 % 60) + value * 60))
        } else {
            None
        }
    }

    pub fn with_minutes(&self, value: u16) -> Option<Self> {
        if value < 60 {
            Some(Self((self.0 / 60) * 60 + value))
        } else {
            None
        }
    }
}

// impl std::str::FromStr for ClockTime {
//     type Err = &'static str;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         todo!();
//         Err("a")
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normal() {
        let Some(time) = ClockTime::new(15, 56) else {
            panic!();
        };
        let minutes = time.get_minutes();
        let hours = time.get_hours();
        assert!(hours == 15);
        assert!(minutes == 56);
        let Some(time) = time.with_hours(8) else {
            panic!();
        };
        assert!(time.get_hours() == 8);
        assert!(time.get_minutes() == 56);
        let Some(time) = time.with_minutes(36) else {
            panic!();
        };
        assert!(time.get_hours() == 8);
        assert!(time.get_minutes() == 36);
    }

    #[test]
    fn invalid_time() {
        assert!(ClockTime::new(15, 60).is_none());
        assert!(ClockTime::new(15, 61).is_none());
        assert!(ClockTime::new(24, 59).is_none());
        assert!(ClockTime::new(25, 59).is_none());
    }

    #[test]
    fn invalid_time_with() {
        let Some(time) = ClockTime::new(15, 56) else {
            panic!();
        };
        assert!(time.with_hours(24).is_none());
        assert!(time.with_hours(25).is_none());
        assert!(time.with_hours(25).is_none());
        assert!(time.with_minutes(60).is_none());
        assert!(time.with_minutes(61).is_none());
        assert!(time.with_minutes(62).is_none());
    }

    #[test]
    fn from_str_valid() {
        let Some(time) = ClockTime::from_str("12:45") else {
            panic!();
        };
        assert!(time.get_hours() == 12);
        assert!(time.get_minutes() == 45);
        let Some(time) = ClockTime::from_str("00:00") else {
            panic!();
        };
        assert!(time.get_hours() == 00);
        assert!(time.get_minutes() == 00);
    }

    #[test]
    fn from_str_invalid() {
        assert_eq!(ClockTime::from_str("12:89"), None);
        assert_eq!(ClockTime::from_str("12/89"), None);
        assert_eq!(ClockTime::from_str("12/00"), None);
        assert_eq!(ClockTime::from_str("9000/30"), None);
    }
}
