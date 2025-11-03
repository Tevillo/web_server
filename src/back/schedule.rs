use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use chrono::{Datelike, Timelike};
/// Schedule holds a name and holds activites of person during the 7 day week
#[derive(Debug, Clone, PartialEq, bincode::Encode, bincode::Decode)]
pub struct Schedule {
    /// Arr of Options
    ///
    /// None represents no activities on the day
    /// `Some<Vec<Activity>>` holds the various activites designated to the day
    ///
    /// In this Sunday represents 0 and Saturday 6
    pub week: [Option<Vec<Activity>>; 7],
    /// Holds name of who the Schedule belongs too
    pub name: String,
}

impl Schedule {
    /// Create empty Schedule. Needs String to assign name
    pub fn new(name: String) -> Self {
        Schedule {
            week: [None, None, None, None, None, None, None],
            name,
        }
    }
    /// Add one activity to Scheduleule
    /// Repeats for every day that is provided
    ///
    /// # Example
    /// ```
    /// let foo = Schedule::new("foo");
    /// let bar = Activity::new(0b0010101, 1200,1400, "Bar".to_string());
    ///
    /// // Add activity to Schedule on Sunday Tuesday Thursday
    /// foo.add_activity(bar)
    /// ```
    pub fn add_activity(&mut self, act: Activity) {
        for day in act.days.iter() {
            if self.week[*day].is_some() {
                self.week[*day].as_mut().unwrap().push(act.clone());
            } else {
                self.week[*day] = Some(vec![act.clone()]);
            }
        }
    }
    /// Checks Scheduleule to see if a new activity would conflict with existing Scheduleule
    ///
    /// If there is no conflict then returns None
    /// If there is a conflict then returns Some<HashSet<&Activity>> which is a set of all conflicts
    fn conflicts(&self, act: Activity) -> Option<HashSet<&Activity>> {
        let mut conflicts = HashSet::new();
        for day in act.days.iter() {
            if let Some(load) = self.week[*day].as_ref() {
                for existing_act in load {
                    if (act.start >= existing_act.start && act.start <= existing_act.end)
                        || (act.end >= existing_act.start && act.end <= existing_act.end)
                    {
                        conflicts.insert(existing_act);
                    }
                }
            }
        }
        match conflicts.is_empty() {
            true => None,
            false => Some(conflicts),
        }
    }
    pub fn is_busy(&self) -> bool {
        let current_time = chrono::offset::Local::now();
        let current_day = (current_time.date_naive().weekday().number_from_sunday() - 1) as usize; // Aligns to current system
        if self.week[current_day].is_none() {
            return false;
        }
        let hour_minute: usize = (current_time.hour() * 100 + current_time.minute()) as usize;
        let events = self.week[current_day].as_ref().unwrap();
        for event in events {
            if hour_minute >= event.start && hour_minute <= event.end {
                return true;
            }
        }
        false
    }
}

/// Start and end is not smart as it is just literal military time
/// This will allow for end - start != the amount of time between the two
/// also can allow for time that does not make sense like 2500 and 1290
///
/// Title: name of activity
/// Days: days of the week the activity is on
#[derive(Hash, Debug, Clone, PartialEq, Eq, bincode::Encode, bincode::Decode)]
pub struct Activity {
    title: String,
    start: usize,
    end: usize,
    days: Vec<usize>,
}

impl Activity {
    pub fn new(title: String, start: usize, end: usize, mut init_day: u8) -> Self {
        let mut days = Vec::with_capacity(7);
        let mut x = 0;
        while init_day > 0 {
            if init_day & 1 == 1 {
                days.push(x);
            }
            init_day >>= 1;
            x += 1;
        }
        Activity {
            start,
            end,
            title,
            days,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Activity {
        fn get_title(&self) -> &str {
            &self.title
        }
    }

    #[test]
    fn empty() {
        let s = Schedule::new("Sample".to_string());
        let empty = Activity::new("empty".to_string(), 0, 0, 0);
        assert!(s.conflicts(empty).is_none());
    }

    #[test]
    fn unavailable() {
        let mut s = Schedule::new("Sample".to_string());
        let x = Activity::new("Foo".to_string(), 0000, 1200, 0b1);
        s.add_activity(x.clone());
        let y = Activity::new("Bar".to_string(), 0600, 1400, 0b1);
        let con = s.conflicts(y);
        assert!(con.is_some());
        let res = con
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>()
            .get(0)
            .unwrap()
            .get_title();
        assert!(res == "Foo");
    }
}
