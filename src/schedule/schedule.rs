fn main() {
    let x = String::from("Fri Oct 03 18:22:28 GMT-0500 (Central Daylight Time)");
    let mut s = create_schedule(String::from("example"));
    println!("{:?}", s.is_available(x.clone()));
    s.add_time(4,1800,2000, String::from("Programming"));
    println!("{:?}", s.is_available(x));
}

struct Sched {
    times: Vec<Option<Vec<Time>>>,
    name:  String,
}

impl Sched {
    fn add_time(&mut self, day: usize, start: usize, end: usize, activity: String) {
        if self.times[day].is_some() {
            self.times[day].as_mut().unwrap().push( Time { start, end, activity});
        } else {
            self.times[day] = Some(vec!(Time{ start, end, activity }));
        }

    }
    fn is_available(&self, time: String) -> Option<&Time> {
        let simple = create_time(time);
        match self.times[simple.0].as_ref() {
            Some(day) => {
                for x in day {
                    if x.start <= simple.1 && x.end >= simple.1 { return Some(x); }
                }
            }
            None => return None,
        }
        None
    }
}

fn create_schedule(name: String) -> Sched {
    Sched {
        times: vec!(None, None, None, None, None),
        name,
    }
}
#[derive(Debug)]
struct Time {
    start: usize,
    end: usize,
    activity: String,
}

fn create_time(js_output: String) -> (usize,usize) {
    let vec_output = Vec::from(js_output);
    let date = match vec_output[2] {
        110 => 0,
        101 => 1,
        100 => 2,
        117 => 3,
        105 => 4,
        _   => 5,
    };
    let mut t = vec_output[11..16].to_vec();
    t.remove(2);
    (date, t.iter().fold(0, |acc, &elem| acc * 10 + elem as usize - 48))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let s = create_schedule(String::from("example"));
        let x = String::from("Fri Oct 03 18:22:28 GMT-0500 (Central Daylight Time)");
        assert!(s.is_available(x).is_none());
    }

    #[test]
    fn unavailable() {
        let mut s = create_schedule(String::from("example"));
        let x = String::from("Fri Oct 03 18:22:28 GMT-0500 (Central Daylight Time)");
        s.add_time(4,1800,2000,String::from("Programming"));
        assert!(!s.is_available(x).is_some());
    }
}

