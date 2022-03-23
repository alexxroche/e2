use std::env;
use std::sync::atomic::Ordering;

use crate::cfg::Cnf;
use crate::fsio::path_exists;
use crate::lib::{CONF, DEBUG};
use crate::log::err;
use crate::pre_process::{pre_process_cfg, ProcedCfg};

mod cfg; // configuration interactions
mod fsio; // File System IO
mod last_gasp; // Signal Handling
mod lib; // globals that we no longer use
pub mod log;
mod pre_process; // generate the full tile set from the XML (unless already available via serde)
mod solve; // where the other magic happens
pub mod st; //the search tree module // mostly colour output
mod util; // helper functions

fn main() {
    // "global" of the config data that we are about to ingest
    //let mut cfg: HashMap<String,String> = HashMap::new();
    let mut cfg: Cnf = cfg::Cnf::new();

    // lets see if the user wants us to use a non-default config file

    let mut c = false;
    let mut conf_file: String = CONF.to_string(); //set the default
                                                  //let args: Vec<String> = env::args().collect();
                                                  //for arg=0;arg<args.len();arg++ { // this way we get an incidental index ..
    for argument in env::args() {
        if c {
            //#lib::log(&format!("Using config file: {}", argument).to_string());
            // D> log(&format!("Using config file: {}", argument).to_string());
            c = false;
            conf_file = argument.clone();
        }
        if argument == "-c" {
            c = true;
        }
        if argument == "-d" {
            //DEBUG += 1;
            DEBUG.fetch_add(1, Ordering::SeqCst);
        }
    }
    /*
    use std::process;
    // DEBUG exit
    process::exit(0);
    //process::exit(0x0100);
    */

    if path_exists(&conf_file) {
        // D> log(&format!("[i] {} found!", &conf_file));
        cfg = cfg::read_conf(&conf_file, cfg);
    } else if path_exists(&("etc/".to_string() + &conf_file)) {
        // D> log(&format!("[w] etc/{} found!", &conf_file));
        cfg = cfg::read_conf(&("etc/".to_string() + &conf_file), cfg);
    } else {
        //println!("[e:missing config file] write conf.ini to {}", conf_file);
        err(&format!(
            "[e:missing config file] write conf.ini to {}",
            conf_file
        ));
    }

    let ProcedCfg(c, board, path, perm, trih, tiles) = pre_process_cfg(cfg);
    let _ = solve::puzzle(c, board, path, perm, trih, tiles);
}
