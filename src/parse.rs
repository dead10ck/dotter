use std::fs::File;
use std::io::{Read, Write};

use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use toml;

pub fn load_file<T>(filename: &str) -> Result<T, String>
    where T: DeserializeOwned
{
    let mut buf = String::new();
    let mut f = File::open(filename)
        .or_else(|_| Err(format!("Error: Couldn't open {}", filename)))?;
    f.read_to_string(&mut buf)
        .or_else(|_| Err(format!("Error: Couldn't read {}", filename)))?;
    Ok(toml::from_str::<T>(&buf)
           .or_else(|_| Err(format!("Error: Couldn't parse {}", filename)))?)
}

pub fn save_file<T>(filename: &str, data: &T) -> Result<(), String>
    where T: Serialize
{
    let mut f = File::create(filename)
        .or_else(|_| Err(format!("Error: Couldn't open {}", filename)))?;
    let buf = toml::to_string(data)
        .or_else(|_| Err("Error: Couldn't serialize data."))?;
    f.write(buf.as_bytes())
        .or_else(|_| Err(format!("Error: Couldn't write to {}", filename)))?;
    Ok(())
}
