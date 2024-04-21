pub mod parse;
pub mod runs;
use crate::clock_time::ClockTime;
use runs::Runs;

#[derive(Debug)]
pub struct Stop {
    pub time: time::Time,
    pub stop_idx: usize,
}

/// One bus journey, with all it's stops and bitflags indicating when does
/// it run.
#[derive(Debug)]
pub struct Journey {
    pub oparates: Runs,
    pub stops: Vec<Stop>,
}

impl Journey {
    /// Initialise an empty journey that never runs
    pub fn default() -> Self {
        Journey::new_from_flags(Runs::NEVER)
    }

    /// Initialise a journey that runs on given flags but doesn't have
    /// any stops yet.
    pub fn new_from_flags(flags: Runs) -> Self {
        Self {
            oparates: flags,
            stops: vec![],
        }
    }

    /// "pretty" print the contents of a journey, moslty for debugging.
    pub fn pretty_print(&self, stop_names: &Vec<String>) {
        // println!("\njourney {}", self.oparates);
        self.stops.iter().for_each(|stop| {
            println!(
                "{:02}:{:02} {:02} {}",
                stop.time.hour(),
                stop.time.minute(),
                stop.stop_idx,
                stop_names[stop.stop_idx]
            )
        })
    }
}

/// Journeys and stops
pub struct TimeTable {
    pub journeys: Vec<Journey>,
    pub stop_names: Vec<String>,
    pub complete_journeys: usize,
}

impl TimeTable {
    /// Initialise an empty timetable.
    pub fn new() -> Self {
        Self {
            journeys: vec![],
            stop_names: vec![],
            complete_journeys: 0,
        }
    }

    /// Call pretty_print on all journeys
    pub fn pretty_print(&self) {
        self.journeys.iter().for_each(|journey| {
            journey.pretty_print(&self.stop_names);
        });
    }

    /// Get the position (inside the stop_names vector) of a stop, by name
    /// Used during parsing.
    fn get_stop_id(&self, stop_name: &str) -> Option<usize> {
        self.stop_names
            .iter()
            .enumerate()
            .find(|(_, elem)| elem.as_str() == stop_name)
            .map(|(idx, _)| idx)
    }

    /// Add a stop name to the vector.
    /// Used during parsing.
    fn add_stop(&mut self, stop_name: &str) -> usize {
        self.stop_names.push(stop_name.to_owned());
        self.stop_names.len() - 1
    }

    /// Try and get a stop's id, by name, or add it to vector and return its id.
    /// Used during parsing.
    fn add_or_get_stop_id(&mut self, stop_name: &str) -> usize {
        match self.get_stop_id(stop_name) {
            Some(idx) => idx,
            None => self.add_stop(stop_name),
        }
    }

    /// Inject a complete line of stop times to the existing journeys.
    /// Always call "Timetable::mark_complete" when done injesting lines for
    /// the journeys.
    pub fn injest_stops<'a>(
        &mut self,
        stop_name: &str,
        cols_time_strs: impl Iterator<Item = &'a str>,
    ) {
        static FORMAT: &'static [time::format_description::BorrowedFormatItem<'static>] =
            time::macros::format_description!("[hour]:[minute]");
        let stop_name_idx = self.add_or_get_stop_id(stop_name);
        cols_time_strs
            .zip(self.journeys.iter_mut().skip(self.complete_journeys))
            .for_each(|(col, journey)| {
                if let Ok(time) = time::Time::parse(col, FORMAT) {
                    journey.stops.push(Stop {
                        time,
                        stop_idx: stop_name_idx,
                    });
                }
            });
    }

    /// Parse running flags and add them to the existing journeys.
    /// If a flag was already present, for instance "runs all year", and we
    /// are adding "runs on Tuesdays": the two will be combined.
    /// Skips journeys marked as complete.
    /// Using during parsing.
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

    /// Mark journeys as complete. Next time we injests new flags or stops,
    /// complete journeys will be untouched. Run "injest_flags_new_journeys"
    /// to create new journeys from an iterator of flags.
    pub fn mark_complete(&mut self) {
        self.complete_journeys = self.journeys.len();
    }

    /// Adds new journeys to the timetable, with the run flags given.
    pub fn injest_flags_new_journeys(&mut self, flags: impl Iterator<Item = Runs>) {
        flags
            .map(|flag| Journey::new_from_flags(flag))
            .for_each(|journey| self.journeys.push(journey));
    }
}
