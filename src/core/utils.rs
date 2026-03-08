use std::{io::Write, path::Path};

use serde::Serialize;

use super::Error;

pub fn concat_unix_path(left: &str, right: &str) -> String {
    let mut str = String::with_capacity(left.len() + 1 + right.len());
    str.push_str(left);
    str.push_str("/");
    str.push_str(right);
    str
}

pub fn print_json_entry(key: &str, value: &str) {
    info!("{}: {},", serde_json::to_string(key).unwrap(), serde_json::to_string(value).unwrap());
}

pub fn write_json_file<T: Serialize, P: AsRef<Path>>(data: &T, path: P) -> Result<(), Error> {
    let file = std::fs::File::create(path)?;
    let mut writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, data)?;
    writer.flush()?;
    Ok(())
}