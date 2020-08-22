use curl::easy::Easy;

#[derive(Error, Debug)]
pub enum AAError {
    #[error("Error Message: {0}")]
    Message(String),
    //TODO: add more error types
}

use anyhow::Result;
use thiserror::Error;

pub fn mk_err<T>(msg: String) -> Result<T> {
    Err(AAError::Message(msg))
}

pub fn mk_ok<T>(t: T) -> Result<T> {
    Ok(t)
}

pub fn get_string(url: &str) -> Result<String> {
    String::from_utf8(get_vec(url)?)
}

pub fn get_vec(url: &str) -> Result<Vec<u8>> {
 
    // get the html for GUI
    let mut data = Vec::new();
    let mut handle = Easy::new();
 
    handle.url(url)?;

    {    
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;

        transfer.perform()?;
    }

    Ok(data)
}