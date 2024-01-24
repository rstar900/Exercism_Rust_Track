use std::fmt;

// Need to derive Debug trait for printing using {:?} and PartialEq for == comparisons
#[derive(Debug, PartialEq)]
pub struct Clock{
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut  processed_hrs: i32 = hours;
        let mut processed_mins: i32 = minutes; 

        if processed_mins >= 0 {
            // Logic for adding minutes into hours if equal to or greater than 60
            while processed_mins >= 60 {
                processed_hrs += 1; 
                processed_mins -= 60;
            }
        } else {

            // get rid of negative minutes and make them positive
            while processed_mins < 0 {
                processed_hrs -= 1;
                processed_mins += 60;
            } 
        }

        if processed_hrs >= 0 {
            processed_hrs %= 24;
            
        } else {
            // subtract the absolute value of hours mod 24 from 24
            processed_hrs = (24 - ((-1 * processed_hrs) % 24)) % 24;
        }
        
        Clock {
            hours: processed_hrs,
            minutes: processed_mins
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

// Implementing Display trait for getting string out of our Clock and obviously to display it without using {:?} 
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Here {:padding(=0 as we want 08 instead of 8), alignment(> for right aligned), width(=2 for 2 digits)}
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
