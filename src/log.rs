use chrono::{DateTime, Utc};

//pub const DEBUG: u8 = 0; // to set verbosity levels for log messages
//pub const ORG: &str = "\x1b[0;33m"; // orange
//pub const VIO: &str = "\x1b[0;35m"; // violet
//pub const LGT: &str = "\x1b[1;36m"; // log time
//pub const LME: &str = "\x1b[1;38m"; // lime
pub const NIL: &str = "\x1b[0m"; // reset/remove colour
pub const RED: &str = "\x1b[0;31m"; // red
pub const ERR: &str = "\x1b[1;31m"; // error
pub const INF: &str = "\x1b[1;32m"; // info
pub const WRN: &str = "\x1b[1;33m"; // HLT/warn
pub const LOG: &str = "\x1b[1;34m"; // log
pub const MAG: &str = "\x1b[1;35m"; // magenta
pub const LGA: &str = "\x1b[1;36m"; // Log Aqua
pub const CYN: &str = "\x1b[1;36m"; // cyan

//pub fn log(msg: &str) {
pub fn log<T: std::fmt::Display>(msg: T) {
    let dt: DateTime<Utc> = Utc::now();
    #[rustfmt::skip]
    println!("{}[{}{}{}]{} {}{}",MAG,CYN,dt.format("%Y-%m-%d_%H:%M:%S_%Z").to_string(),MAG,LOG,msg,NIL);
}

pub fn panic<T: std::fmt::Display>(msg: T) {
    eprintln!("{}[{}p{}]{} {}{}", INF, LGA, INF, WRN, msg, NIL);
    std::process::exit(2);
}
pub fn warn<T: std::fmt::Display>(msg: T) {
    eprintln!("{}[{}w{}]{} {}{}", INF, LGA, INF, WRN, msg, NIL);
}
pub fn info<T: std::fmt::Display>(msg: T) {
    eprintln!("{}[{}i{}]{} {}{}", WRN, INF, WRN, CYN, msg, NIL);
}
pub fn info_n<T: std::fmt::Display>(msg: T) {
    eprint!("{}[{}i{}]{} {}{}", WRN, INF, WRN, CYN, msg, NIL);
}
pub fn err<T: std::fmt::Display>(msg: T) {
    eprintln!("{}[{}e{}]{} {}{}", RED, ERR, RED, LOG, msg, NIL);
}
pub fn pass<T: std::fmt::Display>(msg: T) {
    println!("{}[{}pass{}]{} {}{}", WRN, INF, WRN, LOG, msg, NIL);
}
pub fn fail<T: std::fmt::Display>(msg: T) {
    eprintln!("{}[{}fail{}]{} {}{}", RED, ERR, RED, CYN, msg, NIL);
}

/*
/// Error Print Line
pub fn epl<T: std::fmt::Display>(err: T, msg: T) {
    // error_println!
    #[rustfmt::skip]
    eprintln!("{}[{}err{}]{} {}: {}{}{}",RED,ERR,RED,NIL,msg,WRN,err,NIL);
}
*/

pub fn step() {
    info_n("[press Enter to continue] ");
    let _ = std::io::stdin().read_line(&mut String::new());
}
