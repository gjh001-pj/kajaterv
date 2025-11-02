use std::str::FromStr;
use std::fmt::Display;



use crate::backend::time::Time;
use crate::create_vec_wrapper;

#[derive(Clone, Debug, PartialEq, Copy, PartialOrd, Eq, Ord)]
pub enum ActionType {
    Meg,
    El,
}

impl FromStr for ActionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "El" | "EL" | "el" => Ok(Self::El),
            "Meg" | "MEG" | "meg" => Ok(Self::Meg),
            _ => Err(()),
        }
    }
}

impl Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::El => "El",
            Self::Meg => "Meg",
            #[allow(unreachable_patterns)]
            _ => todo!(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Copy, Eq, PartialOrd, Ord)]
pub struct Action {
    pub time: Time,
    pub ty: ActionType,
}

impl Action {
    pub fn new(ty: ActionType, time: Time) -> Self {
        Self {
            ty, time,
        }
    }
}

create_vec_wrapper!(Actions, Action);

impl Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}: {}", first.ty, first.time)?;
        }
        for action in iter {
            write!(f, ", {}: {}", action.ty, action.time)?;
        }
        Ok(())
    }
}

impl From<&str> for Actions {
    fn from(value: &str) -> Self {
        let mut res = value.split(',').filter_map(|sub| {
            let mut iter = sub.splitn(2, ':');
            let action_type: ActionType;
            if let Some(act_str) = iter.next() {
                if let Ok(act) = act_str.trim().parse() {
                    action_type = act;
                } else { return None; }
            } else { return None; }

            if let Some(time_str) = iter.next() {
                if let Ok(time) = time_str.trim().parse() {
                    return Some(Action::new(action_type, time));
                } else { return None; }
            } else { return None; }
        }).collect::<Vec<_>>();
        res.sort();
        res.into()
    }
}

// impl Action {
//     pub fn vec_from_str(s: &str) -> Vec<(Time, Action)> {
//         s.split(',').filter_map(|sub| {
//             let mut iter = sub.splitn(2, ':');
//             let action: Action;
//             if let Some(act_str) = iter.next() {
//                 if let Ok(act) = act_str.trim().parse() {
//                     action = act;
//                 } else { return None; }
//             } else { return None; }
//
//             if let Some(time_str) = iter.next() {
//                 if let Ok(time) = time_str.trim().parse() {
//                     return Some((time, action));
//                 } else { return None; }
//             } else { return None; }
//         }).collect()
//     }
//
//     pub fn print_vec<'a, T>(vec: T) -> String
//     where
//         T: IntoIterator<Item = &'a (Time, Action)>
//     {
//         vec.into_iter().map(|(time, action)| format!("{}: {}", action, time)).collect::<Vec<_>>().join(", ")
//     }
// }