use std::fmt;


// Using PartialEq to compare the clocks
#[derive(Debug, Eq, PartialEq)]

// Adding in fields: hours & minutes
pub struct Clock {
    hours: i32,
    minutes: i32,
}

// Creating constants 
const HOURS_IN_DAY: i32 = 24;
const MIN_IN_HOUR: i32 = 60;
const MIN_IN_DAY: i32 = HOURS_IN_DAY*MIN_IN_HOUR;



impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // {:02} = two leading zeroes
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // total minutes uses the associated function total minutes to ensure
        // we can find the time in the day regardless of if minutes or hours input
        // is more than exists in a day. This protects against oveflow.
        let total_minutes = Self::total_minutes(hours, minutes);
        let hours = total_minutes / MIN_IN_HOUR;
        let minutes = total_minutes % MIN_IN_HOUR;
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes+minutes)
    }

    fn total_minutes(hours: i32, minutes: i32) -> i32 {
        let total_minutes = (hours * MIN_IN_HOUR + minutes) % MIN_IN_DAY;
        if total_minutes >= 0 {
            total_minutes
        } else {
            total_minutes + MIN_IN_DAY
        }
    }
}

