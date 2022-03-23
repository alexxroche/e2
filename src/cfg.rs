use crate::util::Tile;
use ini::Ini;
use std::fs;
use std::io::prelude::*;
//use crate::log::*;
use serde::{Deserialize, Serialize};
use std::str; // &[u8] -> &str
use sxd_document::parser;

//#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cnf {
    pub x: u16,
    pub y: u16,
    pub pieces: Vec<Tile>,
    pub solutions_dir: String,
    pub filename: String,
    pub blake_str: String,
    pub uuid: String,
}

use std::default::Default;
impl Default for Cnf {
    fn default() -> Self {
        //Self::new() // WOULD this work?
        Cnf {
            x: 0,
            y: 0,
            pieces: vec![],
            solutions_dir: "".to_string(),
            filename: "".to_string(),
            blake_str: "".to_string(),
            uuid: "".to_string(),
        }
    }
}

impl Cnf {
    pub fn new() -> Self {
        Cnf { ..Cnf::default() }
    }
}

/*
pub fn foreach(&self) -> (&u16, &u16, &Vec<Tile>, &String, &String, &String, &String) {
    (
        &self.x,
        &self.y,
        &self.pieces,
        &self.solutions_dir,
        &self.filename,
        &self.blake_str,
        &self.uuid,
    )
}
*/

pub fn read_conf(config_file: &str, mut cfg: Cnf) -> Cnf {
    /* INI should be in its own ini.rs  NTS */

    let conf = Ini::load_from_file(config_file).unwrap();

    // NTS here we will check if the tiles have been parsed
    // and the hashes calcualted, (and if so we can skip the xml
    //   and just deserialise the data from the serde (etc?) directory
    //  )

    let size_section = conf.section(Some("size")).unwrap();
    let x = size_section.get("x").unwrap();
    let y = size_section.get("y").unwrap();
    let self_section = conf.section(None::<String>).unwrap();
    let solutions_dir = self_section.get("solutions_dir").unwrap();
    //cfg.insert("x".to_string(), x.to_string());
    //cfg.insert("y".to_string(), y.to_string());
    cfg.x = x.parse::<u16>().unwrap();
    cfg.y = y.parse::<u16>().unwrap();
    cfg.solutions_dir = solutions_dir.to_string();
    cfg.filename = config_file.to_string();

    //let x_y = cfg.x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap();
    let x8_y8: u16 = cfg.x * cfg.y;
    let x_y: usize = usize::from(x8_y8);
    let mut pieces = Vec::with_capacity(x_y);

    //let ex = cfg.x;
    //println!("The puzzle is {:?} x {}", ex.unwrap().parse::<u32>().unwrap(), y);
    // D> log(&format!("The puzzle is {:?} x {}", ex, y));

    let general_conf = conf.section(None::<String>).unwrap();
    let xml_dir = general_conf.get("xml_dir").unwrap();
    let sheets = conf.section(Some("xml")).unwrap();

    /* XML  NTS: move this into its own xml.rs */

    for (_key, value) in sheets.iter() {
        // D> log(&format!("Importing: {}/{}", xml_dir, value));
        /*
            NOTE refactor the XML import/parsing into its own function
            NOTE so that we can switch between XML parsers (and keep the functions small)
        */
        //let mut xml_file: String = xml_dir.clone().to_owned();
        let mut xml_file: String = (*xml_dir).to_string();
        xml_file.push('/');
        xml_file.push_str(&(*value));
        match fs::File::open(&xml_file) {
            Ok(mut file) => {
                let mut xml = String::new();

                //println!("[i] reading from XML: {}", &xml_file);
                // Read all the file content into a variable (ignoring the result of the operation).
                file.read_to_string(&mut xml)
                    .expect("[e] failed to read xml from file");
                //println!("[i] XML: {}", xml);
                let package = parser::parse(&xml).expect("failed to parse XML");
                let document = package.as_document();

                //  config

                let d = document.root().children()[0].element().expect("Missing e1");
                for r in d.children() {
                    // root
                    if let Some(t) = r.element() {
                        // tiles
                        if Some(t).is_some() {
                            //edges
                            // we have 256 tiles so u8 is too small
                            let t_id: u16 = t.attribute("id").unwrap().value().parse().unwrap();
                            //let mut sides: Vec<&str> = Vec::new(); // if we have more than 255 types of edge then change from u8 -> u16
                            // if we have more than 255 types of edge then change from u8 -> u16
                            let mut sides: [u8; 4] = [0; 4];
                            //let mut loop_offset: usize = 1;
                            // ~/XML/sxd_test/src/main.rs doesn't need this, so why do we? BUG
                            /* A. because we have comments in our XML files that the sxd_test didn't! */
                            //for (i, e) in t.children().iter().enumerate() {
                            let mut li: usize = 0; // loop_index
                            for e in t.children().iter() {
                                //sides[i] = e.element().unwrap().attribute("value").unwrap().value().parse().unwrap();
                                if let Some(thing) = e.element() {
                                    /* NOTE this could be simplified */
                                    //println!("[d] {} this is an {:?}", i, e.element());
                                    match thing.attribute("value").unwrap().value().parse().unwrap()
                                    {
                                        0 => (),
                                        1..=255 => {
                                            //sides[li-loop_offset] = thing.attribute("value").unwrap().value().parse().unwrap();
                                            sides[li] = thing
                                                .attribute("value")
                                                .unwrap()
                                                .value()
                                                .parse()
                                                .unwrap();
                                            li += 1;
                                        }
                                        _ => println!(
                                            "looks like a comment: {:?}",
                                            thing.attribute("value").unwrap()
                                        ),
                                    };
                                }
                            }
                            let p = Tile {
                                i: t_id - 1,
                                e: (sides),
                                r: 1,
                            };
                            pieces.push(p);
                        }
                    }
                }
            }
            // Error handling.
            Err(error) => {
                println!("Error opening file {}: {}", &xml_file, error);
            }
        }
    }
    cfg.pieces = pieces;
    cfg
}

/*
    Here we are going to update the config[section] with a value
    (notably the hashes section)
*/
pub fn update_cf(conf_filename: &str, section: Option<&str>, key: &str, value: &str) {
    let mut conf = Ini::load_from_file(conf_filename).unwrap();
    conf.with_section(section).set(key, value);
    conf.write_to_file(conf_filename).unwrap();
}

#[cfg(test)]
mod cfg_tests {
    //use super::*;
    use crate::cfg::Cnf;
    #[test]
    fn can_create_new_cnf() {
        let mut cfg: Cnf = crate::cfg::Cnf::new();
        cfg.y = 16;
        eprintln!("[t] {:?}", cfg);
        assert_eq!(cfg.x, (Cnf { ..Cnf::default() }).x);
        assert_ne!(cfg.y, (Cnf { ..Cnf::default() }).y);
    }
} //tests
