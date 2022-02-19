use std::{fs::File, error::Error};


extern crate libc;

pub fn main() -> Result<(), Box<dyn Error>>{
    let file = File::create("signals.rs")?;
    Ok(())
}