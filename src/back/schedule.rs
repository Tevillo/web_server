use std::collections::HashSet;

/// Sched holds a name and holds activites of person during the 7 day week
struct Sched {
    /// Arr of Options
    ///
    /// None represents no activities on the day
    /// Some<Vec<Activity>> holds the various activites designated to the day
    ///
    /// In this Sunday represents 0 and Saturday 6
    week: [Option<Vec<Activity>>; 7],
    /// Holds name of who the Sched belongs too
    name: String,
}

impl Sched {
    /// Create empty Sched. Needs String to assign name
    fn new(name: String) -> Self {
        Sched {
            week: [None, None, None, None, None, None, None],
            name,
        }
    }
    /// Add one activity to schedule
    /// Repeats for every day that is provided
    ///
    /// # Example
    ///
    /// let foo = Sched::new("foo");
    /// let bar = Activity::new(0b0010101, 1200,1400, "Bar".to_string());
    ///
    /// // Add activity to sched on Sunday Tuesday Thursday
    /// foo.add_activity(bar)
    fn add_activity(&mut self, act: Activity) {
        for day in act.days.iter() {
            if self.week[*day].is_some() {
                self.week[*day].as_mut().unwrap().push(act.clone());
            } else {
                self.week[*day] = Some(vec![act.clone()]);
            }
        }
    }
    /// Checks schedule to see if a new activity would conflict with existing schedule
    ///
    /// If there is no conflict then returns None
    /// If there is a conflict then returns Some<HashSet<&Activity>> which is a set of all conflicts
    fn conflicts(&self, act: Activity) -> Option<HashSet<&Activity>> {
        let mut conflicts = HashSet::new();
        for day in act.days.into_iter() {
            if let Some(load) = self.week[day].as_ref() {
                for x in load {
                    if x.start <= act.start && x.end >= act.end {
                        conflicts.insert(x);
                    }
                }
            }
        }
        match conflicts.is_empty() {
            true => None,
            false => Some(conflicts),
        }
    }
}

/// Start and end is not smart as it is just literal military time
/// This will allow for end - start != the amount of time between the two
/// also can allow for time that does not make sense like 2500 and 1290
///
/// Title: name of activity
/// Days: days of the week the activity is on
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Activity {
    start: usize,
    end: usize,
    title: String,
    days: Vec<usize>,
}

impl Activity {
    fn new(mut init_day: u8, start: usize, end: usize, title: String) -> Self {
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

    #[test]
    fn empty() {
        let s = Sched::new("Sample".to_string());
        let empty = Activity::new(0, 0, 0, String::from("Empty"));
        assert!(s.conflicts(empty).is_none());
    }

    #[test]
    fn unavailable() {
        let mut s = Sched::new("Sample".to_string());
        let x = Activity::new(0b1, 0000, 1200, String::from("Foo"));
        s.add_activity(x);
        let y = Activity::new(0b1, 0600, 1400, String::from("Bar"));
        let res = y.clone();
        let con = s.conflicts(y);
        assert!(con.is_some());
        assert!(con.unwrap().contains(&res));
    }
}
