#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

pub(crate) mod error {
    pub(crate) use sys::error::Error;
}

#[cfg(feature = "std")]
pub fn all_glory_to_the_hypnotoad() -> Result<(), Box<dyn crate::error::Error>> {
    println!("Hello from 'comm' with feature 'std' enabled!");

    sys::all_glory_to_the_hypnotoad()?;
    base::all_glory_to_the_hypnotoad()?;

    Ok(())
}

#[cfg(not(feature = "std"))]
pub fn all_glory_to_the_hypnotoad() -> Result<(), Box<dyn crate::error::Error>> {
    // println!("Hello from 'comm' with feature 'std' disabled!");

    sys::all_glory_to_the_hypnotoad()?;
    base::all_glory_to_the_hypnotoad()?;

    Ok(())
}
