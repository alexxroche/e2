use std::fs::{self, File, OpenOptions}; //self: mkdir, path_exists; File: read; OpenOptions: write
use std::io::{self, Read, Write};

pub fn write(file: &str, data: &str) -> Result<String, io::Error> {
    let mut fh = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file)?;
    match fh.write_all(data.as_bytes()) {
        Ok(_) => Ok("Ok".to_string()),
        Err(e) => Err(e),
    }
}

pub fn append(file: &str, data: &str) -> Result<String, io::Error> {
    let mut fh = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file)?;
    match fh.write_all(data.as_bytes()) {
        Ok(_) => Ok("Ok".to_string()),
        Err(e) => Err(e),
    }
}

pub fn read(f: &str) -> Result<String, io::Error> {
    let f = File::open(f);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn mkdir(path: &str) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}
// check if conf.ini exists
pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
