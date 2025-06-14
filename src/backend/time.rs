use std::ops::{Deref, DerefMut, Add, Sub};
use std::fmt::{self, Display};


const MIN: i32 = 1;
const HOUR: i32 = 60 * MIN;
const DAY: i32 = 24 * HOUR;

#[derive(PartialEq, Debug, Clone, Copy, Default, Eq, Hash, Ord, PartialOrd)]
pub struct Time {
    pub mins: i32,
}

impl Time {
    pub fn new() -> Self {
        Self {
            mins: 0
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        let space: Vec<&str> = s.split(". ").collect();
        let day: i32 = match space.first().unwrap().parse() {
            Err(_) => return Err("1".to_string()),
            Ok(v) => v,
        }; 
        let hour;
        let min ;
        if space.len() > 1 {
            let dots: Vec<&str> = space[1].split(":").collect();
            if dots.len() >= 2 {
                hour = dots[0].parse().unwrap();
                min = dots[1].parse().unwrap();
            } else {
                hour = 0;
                min = 0;
            }
        } else {
            hour = 0;
            min = 0;
        }

        Ok(Time {
            mins: day * DAY + hour * HOUR + min * MIN
        })
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
        let day = self.mins / DAY;
        let hour = (self.mins - day * DAY) / HOUR;
        let min = (self.mins - day * DAY - hour * HOUR) / MIN;
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

