use std::ops::{Deref, DerefMut, Add, Sub};
use std::fmt::{self, Display};
use std::str::FromStr;

const MINS_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;

const MIN: i32 = 1;
const HOUR: i32 = MINS_IN_HOUR * MIN;
const DAY: i32 = HOURS_IN_DAY * HOUR;

#[derive(PartialEq, Debug, Clone, Copy, Default, Eq, Hash, Ord, PartialOrd)]
pub struct Time {
    pub mins: i32,
}

impl Time {
    pub fn new(day: i32, hour: i32, min: i32) -> Self {
        Self {
            mins: day * DAY + hour * HOUR + min * MIN,
        }
    }
}

impl FromStr for Time {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let space: Vec<_> = s.split(". ").collect();
        let day: i32 = match space.first().unwrap_or(&"0").parse() {
            Err(_) => return Err("Invalid day format! Correct form: 'day. (...)'".to_string()),
            Ok(v) => v,
        }; 
        let (hour, min) = if space.len() > 1 {
            let dots: Vec<&str> = space[1].split(":").collect();
            if dots.len() != 2 {
                return Err("Invalid hour-min format! Correct format: '(...). hours:days'".to_string());
            }
            let hour: i32 = match dots[0].parse() {
                Err(_) => return Err("Invalid hour format".to_string()),
                Ok(v) => v,
            };

            let min: i32 = match dots[1].parse() {
                Err(_) => return Err("Invalid minute format".to_string()),
                Ok(v) => v,
            };

            (hour, min)
        } else {
            (0, 0)
        };

        if hour < 0 || hour >= 24 {
            return Err("Hour must be between 0 and 23".to_string());
        }

        if min < 0 || min >= 60 {
            return Err("Minute must be between 0 and 59".to_string());
        }

        Ok(Time::new(day, hour, min))
    }
}

// impl ToString for Time {
//     fn to_string(&self) -> String {
//         let day = self.mins / DAY;
//         let hour = (self.mins - day * DAY) / HOUR;
//         let min = (self.mins - day * DAY - hour * HOUR) / MIN;
//         format!("{}. {}:{}", day, hour, min)
//     }
// }

impl Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //let sign = if self.mins < 0 { "-" } else { "" };
        //let mins = self.mins;

        let mut day = self.mins / DAY;
        let mut hour = (self.mins - day * DAY) / HOUR;
        if hour < 0 {
            day -= 1;
            hour += HOURS_IN_DAY;
        }
        let mut min = (self.mins - day * DAY - hour * HOUR) / MIN;
        if min < 0 {
            hour -= 1;
            min += MINS_IN_HOUR;
        }
        write!(f, "{}. {}:{:02}", day, hour, min)
    }
}

impl Add for Time {
    type Output = Time;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            mins: self.mins + rhs.mins
        }
    }
}

impl Sub for Time {
    type Output = Time;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            mins: self.mins - rhs.mins
        }
    }
}

impl Deref for Time {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.mins
    }
}

impl DerefMut for Time {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mins
    }
}


#[test]
fn test_time_parse() {
    let a = "-2. 8:45".parse::<Time>().unwrap();
    println!("{a} = {a:?}");
    let a = "-2. 13:00".parse::<Time>().unwrap();
    println!("{a} = {a:?}");
    let a = "-2. 19:00".parse::<Time>().unwrap();
    println!("{a} = {a:?}");
    let a = "-1. 8:45".parse::<Time>().unwrap();
    println!("{a} = {a:?}");
    let a = "-1. 13:00".parse::<Time>().unwrap();
    println!("{a} = {a:?}");
    let a = "-1. 19:00".parse::<Time>().unwrap();
    println!("{a} = {a:?}");
    let a = "0. 8:45".parse::<Time>().unwrap();
    println!("{a} = {a:?}");
    let a = "0. 13:00".parse::<Time>().unwrap();
    println!("{a} = {a:?}");
    let a = "0. 19:00".parse::<Time>().unwrap();
    println!("{a} = {a:?}");
    //panic!("");
}
