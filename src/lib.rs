// #![feature(weak_into_raw)]
#![feature(strict_provenance)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]

// extern crate tailcalled;
extern crate lasso;
extern crate serde;

pub mod lazy;
pub mod cons;
pub mod meta;
pub mod skew;
pub mod name;
pub mod sets;
// pub mod term;
