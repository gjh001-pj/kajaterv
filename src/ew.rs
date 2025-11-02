use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

use crate::terv::Terv;

#[derive(Debug, Clone)]
pub enum EW<T> {
    Owned (String),
    Child {
        from: String,
        ews: EWs<T>,
    },
}

impl<T> EW<T> {
    pub const BEHUZAS: &str = "    ";

    pub fn n_behuzas(n: usize) -> String {
        let mut res = String::new();
        for _ in 0..n {
            res += EW::<T>::BEHUZAS;
        }
        res
    }

    pub fn count(&self) -> usize {
        match self {
            Self::Owned(_) => 1,
            Self::Child { from: _, ews, } => {
                ews.count()
            }
        }
    }

    pub fn child_if(from: String, ews: EWs<T>) -> Option<EW<T>> {
        ews.to_child(from)
    }

    pub fn to_string_alap(&self, behuzas: usize) -> String {
        match &self {
            Self::Owned(string) => {
                format!("{}- {}\n", EW::<T>::n_behuzas(behuzas), string)
            },
            Self::Child { from, ews } => {
                let mut res = format!("{}- {}\n", EW::<T>::n_behuzas(behuzas), from);
                for ew in ews.iter() {
                    res += &ew.to_string_alap(behuzas + 1);
                }
                res
            }
        }
    }
}

impl<T> From<String> for EW<T> {
    fn from(value: String) -> Self {
        Self::Owned(value)
    }
}

impl<T> From<&str> for EW<T> {
    fn from(value: &str) -> Self {
        Self::Owned(value.to_string())
    }
}

impl<T> std::fmt::Display for EW<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_alap(0))
    }
}


#[derive(Debug, Default, Clone)]
pub struct EWs<T> (pub Vec<EW<T>>, pub PhantomData<T>);

impl<T> EWs<T> {
    pub fn push_opt(&mut self, opt: Option<EW<T>>) {
        match opt {
            Some(ew) => {
                self.push(ew);
            },
            None => {},
        };
    }

    pub fn to_child(self, from: String) -> Option<EW<T>> {
        if self.len() > 0 {
            Some(EW::Child { from, ews: self })
        } else {
            None
        }
    }

    pub fn count(&self) -> usize {
        self.iter().map(|ew| ew.count()).sum()
    }

    pub fn new() -> Self {
        Self(Vec::new(), PhantomData)
    }
}

impl<T> std::fmt::Display for EWs<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ew in self.iter() {
            write!(f, "{}", ew);
        }
        Ok(())
    }
}

// implement default things

impl<T> From<Vec<EW<T>>> for EWs<T> {
    fn from(value: Vec<EW<T>>) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> Into<Vec<EW<T>>> for EWs<T> {
    fn into(self) -> Vec<EW<T>> {
        self.0
    }
}

impl<T> AsRef<Vec<EW<T>>> for EWs<T> {
    fn as_ref(&self) -> &Vec<EW<T>> {
        &self.0
    }
}

impl<T> AsMut<Vec<EW<T>>> for EWs<T> {
    fn as_mut(&mut self) -> &mut Vec<EW<T>> {
        &mut self.0
    }
}

impl<T> Deref for EWs<T> {
    type Target = Vec<EW<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for EWs<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct Err;
#[derive(Debug)]
pub struct Warn;

pub trait GetEWs {
    fn get_errors(&self, terv: &Terv) -> EWs<Err>;
    fn get_warnings(&self, terv: &Terv) -> EWs<Warn>;
    fn get_from(&self) -> String;
}

#[macro_export]
macro_rules! impl_getews_for_vec_wrapper {
    ($wrapper:ident, $inner_type:ty ) => {
        impl GetEWs for $wrapper {
            fn get_errors(&self, terv: &crate::terv::Terv) -> EWs<crate::ew::Err> {
                self.iter().filter_map(|item: &$inner_type| {
                    EW::child_if(item.get_from(), item.get_errors(&terv))
                }).collect::<Vec<_>>().into()
            }
            fn get_warnings(&self, terv: &crate::terv::Terv) -> EWs<crate::ew::Warn> {
                self.iter().filter_map(|item: &$inner_type| {
                    EW::child_if(item.get_from(), item.get_warnings(&terv))
                }).collect::<Vec<_>>().into()
            }
            fn get_from(&self) -> String {
                String::new()
            }
        }
    };
}


#[test]
fn test_ew_tree() {
    let ew1: EW<Err> = EW::Child { 
        from: "a1".to_string(), 
        ews: EWs(
            vec![
                EW::Owned ("b1".to_string()),
                EW::Child { 
                    from: "b2".to_string(), 
                    ews: EWs(
                        vec![
                            EW::Child { 
                                from: "c11".to_string(), 
                                ews: EWs(
                                    vec![
                                        EW::Owned("d1".to_string())
                                    ], PhantomData
                                ) 
                            },
                            EW::Owned("c21".to_owned()),
                        ], PhantomData
                    )
                },
                EW::Owned("b3".to_string()),
                EW::Child { 
                    from: "b4".to_string(), 
                    ews: EWs(
                        vec![
                            EW::Owned("c12".to_owned()),
                            EW::Owned("c21".to_owned()),
                        ], PhantomData
                    )
                },
            ], PhantomData
        ) 
    };

    println!("{:#?}\n{}", ew1, ew1);

    //panic!();
}

/*
a1
    b1
    b2
        c11
            d1
        c21
    b3
    b4
        c12
        c22




*/