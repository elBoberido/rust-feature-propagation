#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]

pub(crate) mod error {
    pub(crate) use sys::error::Error;
}

fn main() -> Result<(), Box<dyn crate::error::Error>> {
    #[cfg(feature = "std")]
    println!("Hello from 'app' with feature 'std' enabled in crates!");
    #[cfg(not(feature = "std"))]
    println!("Hello from 'app' with feature 'std' disabled in crates!");

    sys::all_glory_to_the_hypnotoad()?;
    base::all_glory_to_the_hypnotoad()?;
    comm::all_glory_to_the_hypnotoad()?;

    Ok(())
}
