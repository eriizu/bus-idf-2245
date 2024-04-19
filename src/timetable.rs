use crate::clock_time::ClockTime;
use crate::operating_flags::Runs;

#[derive(Debug)]
pub struct Stop {
    pub time: ClockTime,
    pub stop_idx: usize,
}

#[derive(Debug)]
pub struct Journey {
    pub oparates: Runs,
    pub stops: Vec<Stop>,
}

impl Journey {
    pub fn default() -> Self {
        Journey::new_from_flags(Runs::NEVER)
    }
    pub fn new_from_flags(flags: Runs) -> Self {
        Self {
            oparates: flags,
            stops: vec![],
        }
    }
}

pub struct TimeTable {
    pub journeys: Vec<Journey>,
    pub stop_names: Vec<String>,
}

impl TimeTable {
    pub fn new_from_flags(flags_iter: impl Iterator<Item = Runs>) -> Self {
        let mut journeys = vec![];
        flags_iter.for_each(|flags| {
            journeys.push(Journey::new_from_flags(flags));
        });
        Self {
            journeys,
            stop_names: vec![],
        }
    }

    pub fn pretty_print(&self) {
        self.journeys.iter().for_each(|journey| {
            println!("\njourney {}", journey.oparates);
            journey.stops.iter().for_each(|stop| {
                println!(
                    "{:02}:{:02} {:02} {}",
                    stop.time.get_hours(),
                    stop.time.get_minutes(),
                    stop.stop_idx,
                    self.stop_names[stop.stop_idx]
                )
            })
        });
    }

    pub fn get_stop_id(&self, stop_name: &str) -> Option<usize> {
        self.stop_names
            .iter()
            .enumerate()
            .find(|(_, elem)| elem.as_str() == stop_name)
            .map(|(idx, _)| idx)
    }

    pub fn add_stop(&mut self, stop_name: &str) -> usize {
        self.stop_names.push(stop_name.to_owned());
        self.stop_names.len() - 1
    }

    pub fn add_or_get_stop_id(&mut self, stop_name: &str) -> usize {
        match self.get_stop_id(stop_name) {
            Some(idx) => idx,
            None => self.add_stop(stop_name),
        }
    }
}
