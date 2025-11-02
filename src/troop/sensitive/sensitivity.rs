use std::str::FromStr;
use std::fmt::Display;



use crate::create_vec_wrapper;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sensitivity {
    G,
    TF,
    L,
}

impl FromStr for Sensitivity {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "G" | "g" => Ok(Self::G),
            "TF" | "tf" => Ok(Self::TF),
            "L" | "l" => Ok(Self::L),
            _ => Err(()),
        }
    }
}

impl Display for Sensitivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::G => "G",
            Self::TF => "TF",
            Self::L => "L",
            #[allow(unreachable_patterns)]
            _ => todo!(),
        })
    }
}

// impl Sensitivity {
//     // pub fn vec_from_str(s: &str) -> Vec<Sensitivity> {
//     //     let mut res = s.split([' ', ',']).filter_map(|s| {
//     //         if let Ok(sensitivity) = s.parse::<Sensitivity>() {
//     //             Some(sensitivity)
//     //         } else {
//     //             None
//     //         }
//     //     }).collect::<Vec<_>>();
//     //     res.sort_by_key(|sens| sens.to_string());
//     //     res.dedup();
//     //     res
//     // }
//
//     // pub fn print_vec<'a, T>(vec: T) -> String
//     // where
//     //     T: IntoIterator<Item = &'a Sensitivity>
//     // {
//     //     vec.into_iter().map(|sensitivity| sensitivity.to_string()).collect::<Vec<_>>().join(", ")
//     // }
// }

create_vec_wrapper!(Sensitivities, Sensitivity);

impl ToString for Sensitivities {
    fn to_string(&self) -> String {
        self.iter().map(|sensitivity| sensitivity.to_string()).collect::<Vec<_>>().join(", ")
    }
}

impl From<&str> for Sensitivities {
    fn from(value: &str) -> Self {
        let mut res = value.split([' ', ',']).filter_map(|s| {
            if let Ok(sensitivity) = s.parse::<Sensitivity>() {
                Some(sensitivity)
            } else {
                None
            }
        }).collect::<Vec<_>>();
        res.sort_by_key(|sens| sens.to_string());
        res.dedup();
        res.into()
    }
}

impl From<&String> for Sensitivities {
    fn from(value: &String) -> Self {
        Sensitivities::from(value.as_str())
    }
}