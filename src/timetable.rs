pub mod parse;
pub mod runs;
use crate::clock_time::ClockTime;
use runs::Runs;

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

    pub fn pretty_print(&self, stop_names: &Vec<String>) {
        println!("\njourney {}", self.oparates);
        self.stops.iter().for_each(|stop| {
            println!(
                "{:02}:{:02} {:02} {}",
                stop.time.get_hours(),
                stop.time.get_minutes(),
                stop.stop_idx,
                stop_names[stop.stop_idx]
            )
        })
    }
}

pub struct TimeTable {
    pub journeys: Vec<Journey>,
    pub stop_names: Vec<String>,
    pub complete_journeys: usize,
}

impl TimeTable {
    pub fn new() -> Self {
        Self {
            journeys: vec![],
            stop_names: vec![],
            complete_journeys: 0,
        }
    }
    pub fn new_from_flags(flags_iter: impl Iterator<Item = Runs>) -> Self {
        let mut journeys = vec![];
        flags_iter.for_each(|flags| {
            journeys.push(Journey::new_from_flags(flags));
        });
        Self {
            journeys,
            stop_names: vec![],
            complete_journeys: 0,
        }
    }

    pub fn pretty_print(&self) {
        self.journeys.iter().for_each(|journey| {
            journey.pretty_print(&self.stop_names);
        });
    }

    fn get_stop_id(&self, stop_name: &str) -> Option<usize> {
        self.stop_names
            .iter()
            .enumerate()
            .find(|(_, elem)| elem.as_str() == stop_name)
            .map(|(idx, _)| idx)
    }

    fn add_stop(&mut self, stop_name: &str) -> usize {
        self.stop_names.push(stop_name.to_owned());
        self.stop_names.len() - 1
    }

    fn add_or_get_stop_id(&mut self, stop_name: &str) -> usize {
        match self.get_stop_id(stop_name) {
            Some(idx) => idx,
            None => self.add_stop(stop_name),
        }
    }

    pub fn injest_stops<'a>(
        &mut self,
        stop_name: &str,
        cols_time_strs: impl Iterator<Item = &'a str>,
    ) {
        let stop_name_idx = self.add_or_get_stop_id(stop_name);
        cols_time_strs
            .zip(self.journeys.iter_mut().skip(self.complete_journeys))
            .for_each(|(col, journey)| {
                if let Some(time) = ClockTime::from_str(col) {
                    journey.stops.push(Stop {
                        time,
                        stop_idx: stop_name_idx,
                    });
                }
            });
    }

    pub fn injest_parse_flags<'a>(
        self: &mut TimeTable,
        cols_operation_strs: impl Iterator<Item = &'a str>,
    ) {
        cols_operation_strs
            .map(Runs::from_str)
            .zip(self.journeys.iter_mut().skip(self.complete_journeys))
            .for_each(|(flags, journey)| {
                journey.oparates |= flags;
            });
    }

    pub fn mark_complete(&mut self) {
        self.complete_journeys = self.journeys.len();
    }

    pub fn injest_flags_new_journeys(&mut self, flags: impl Iterator<Item = Runs>) {
        flags
            .map(|flag| Journey::new_from_flags(flag))
            .for_each(|journey| self.journeys.push(journey));
    }
}
