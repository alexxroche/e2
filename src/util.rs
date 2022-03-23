use crate::cfg::Cnf;
use crate::solve::store_st;
use crate::st::St;

use serde::{Deserialize, Serialize}; // so serde can do her magic with the Tile struct
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Tile {
    pub i: u16,     //id
    pub e: [u8; 4], //array of edges
    // pub edges: (u8, u8, u8, u8), //tuple of edges
    pub r: u8, //rotation [1..4]
}

// Define what to do with each signal
// part of last_gasp
pub fn process_signal(st: &St, c: &Cnf) {
    use crate::last_gasp::{clear_sig, final_breath, get_sig};
    match get_sig() {
        1 | 15 => {
            store_st(st, c);
            // clear the signal
            clear_sig();
        }
        sig => final_breath(sig, st, c),
    };
}
// last_gasp END

pub fn intersection_of_trih_and_vec(trih: &[u16], tf: &[(u16, u8)]) -> Vec<(u16, u8)> {
    //let mut intersection: Vec<T> = vec![];  // we can generalise this later for fun
    let mut intersection: Vec<(u16, u8)> = vec![];
    if trih.len() < tf.len() {
        for i in tf.iter() {
            for j in trih.iter() {
                if i.0 == *j {
                    intersection.push(*i);
                    break;
                }
            }
        }
    } else {
        // child is longer
        for i in trih.iter() {
            for j in tf.iter() {
                if *i == j.0 {
                    intersection.push(*j);
                    break;
                }
            }
        }
    }
    intersection
}

#[cfg(test)]
mod util_tests {
    //use super::*;  //cargo test lib::tests // <<<<< YOU should probably just use this
    //use super::{intersection_of_trih_and_vec, lib_tests}; //cargo test lib::tests
    use super::intersection_of_trih_and_vec; //cargo test lib::tests
                                             //use super::intersection_of_trih_and_vec; //cargo test

    #[test]
    fn intersection_of_trih_and_vec_works() {
        let trih: Vec<u16> = vec![0, 1, 2]; //trih
        let tf: Vec<(u16, u8)> = vec![(2, 3), (17, 1), (0, 1), (19, 11)]; //tiles found
        let tr_int = intersection_of_trih_and_vec(&trih, &tf);
        assert_eq!(tr_int, vec![(2, 3), (0, 1)]);
    }
    #[test]
    fn intersection_of_trih_and_vec_detects_failure() {
        let trih: Vec<u16> = vec![0, 1, 2]; //trih
        let tf: Vec<(u16, u8)> = vec![(2, 3), (17, 1), (0, 1), (19, 11)]; //tiles found
        let tr_int = intersection_of_trih_and_vec(&trih, &tf);
        assert_ne!(tr_int, vec![(2, 3), (2, 1)]);
    }
} //tests
