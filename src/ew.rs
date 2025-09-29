use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

use crate::terv::Terv;



pub enum EW<T> {
    Owned (String),
    Child {
        from: String,
        ews: EWs<T>,
    },
}

impl<T> EW<T> {
    pub fn count(&self) -> usize {
        match self {
            Self::Owned(_) => 1,
            Self::Child { from: _, ews, } => {
                ews.count()
            }
        }
    }

    pub fn child_if(from: String, ews: EWs<T>) -> Option<EW<T>> {
        if ews.len() > 0 {
            Some(Self::Child { from, ews, })
        } else {
            None
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



pub struct EWs<T> (pub Vec<EW<T>>, pub PhantomData<T>);

impl<T> EWs<T> {
    pub fn count(&self) -> usize {
        self.iter().map(|ew| ew.count()).sum()
    }
}

impl<T> EWs<T> {
    pub fn new() -> Self {
        Self(Vec::new(), PhantomData)
    }
}

impl<T> From<Vec<EW<T>>> for EWs<T> {
    fn from(value: Vec<EW<T>>) -> Self {
        Self(value, PhantomData)
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

pub struct Err;
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
