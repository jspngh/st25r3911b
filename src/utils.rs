//! Module for utility functions
#![allow(unused_macros)]
#![allow(unused_imports)]

macro_rules! trace {
    ($t:literal $(,)? $($j:ident),*) => { };
}

macro_rules! debug {
    ($t:literal $(,)? $($j:ident),*) => { };
}

macro_rules! info {
    ($t:literal $(,)? $($j:ident),*) => { };
}


pub(crate) use trace;
pub(crate) use debug;
pub(crate) use info;
