#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

pub mod error {
    #[cfg(feature = "std")]
    pub(crate) mod reexports {
        pub use std::error::Error;
    }

    #[cfg(not(feature = "std"))]
    pub(crate) mod reexports {
        pub use core::error::Error;
    }

    pub use reexports::*;
}

#[cfg(feature = "std")]
pub fn all_glory_to_the_hypnotoad() -> Result<(), Box<dyn crate::error::Error>> {
    println!("Hello from 'sys' with feature 'std' enabled!");

    Ok(())
}

#[cfg(not(feature = "std"))]
pub fn all_glory_to_the_hypnotoad() -> Result<(), Box<dyn crate::error::Error>> {
    // println!("Hello from 'sys' with feature 'std' disabled!");

    Ok(())
}
