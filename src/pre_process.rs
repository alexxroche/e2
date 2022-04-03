use crate::cfg::{update_cf, Cnf};
use crate::fsio::{self, path_exists};
use crate::util::Tile;
use std::collections::HashMap;
use uuid::Uuid;

// rotate_tile_anti-clockwise
fn rotate_tile(t: Tile) -> Tile {
    let mut e: [u8; 4] = t.e;
    // hold
    let h = e[0];
    e[0] = e[1];
    e[1] = e[2];
    e[2] = e[3];
    e[3] = h;
    let mut r = t.r;
    r += 1;
    if r == 5 {
        r = 1;
    } //poor mans modulo
    Tile { i: t.i, e, r }
}

/// expects a vector of pieces
/// -> vector of tiles (all rotations)
fn gen_tiles(pieces: &[Tile]) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = vec![];
    for p in pieces {
        let p1 = *p;
        tiles.push(p1);

        let p2 = rotate_tile(p1);
        if p2.e != p1.e {
            tiles.push(p2);
        }
        let p3 = rotate_tile(p2);
        if p3.e != p2.e && p3.e != p1.e {
            tiles.push(p3);
        }
        let p4 = rotate_tile(p3);
        if p4.e != p3.e && p4.e != p2.e && p4.e != p1.e {
            tiles.push(p4);
        }
    }
    tiles
}

/*
        This is meant to take each piece and convert the four rotations
        into an entry in a HashMap to reduce the search space to ONLY the tiles that match
*/

//                                  <edges => vec![Tile_ids,rotation]>
fn gen_perm(ps: &[Tile]) -> HashMap<[u8; 4], Vec<(u16, u8)>> {
    //let perm: HashMap<[u8;4], Vec<(u16,u8)>> = HashMap { [0,1,2,3] => vec![(5,1),(19,2)], [1,1,0,0] => vec![(1,1),(3,2),(7,3),(9,4)] };
    let mut perm: HashMap<[u8; 4], Vec<(u16, u8)>> = HashMap::new();

    for p in ps.iter() {
        // iterate the PeaceS as P
        let [pt, pr, pb, pl] = p.e;
        let border_count: u8 = (&p.e)
            .iter()
            .map(|z| if z == &1_u8 { z } else { &0_u8 })
            .sum();
        //log(format!("Piece {:?} has {:?} borders: {:?}", p.i, border_count, p.e));
        // [t,r,b,l] a0
        match perm.get_mut(&p.e) {
            None => {
                let _ = perm.insert(p.e, vec![(p.i, p.r)]);
            }
            Some(existing) => existing.push((p.i, p.r)),
        };

        // Note: there MUST be a methodical way to iterate the array being used as the key
        // Note: (rather than doing this longhand)

        // [t,r,b,0] m1
        let pa1: [u8; 4] = [pt, pr, pb, 0];
        match perm.get_mut(&pa1) {
            None => {
                let _ = perm.insert(pa1, vec![(p.i, p.r)]);
            }
            Some(existing) => existing.push((p.i, p.r)),
        };
        // [t,r,0,l] m2
        let pa2: [u8; 4] = [pt, pr, 0, pl];
        match perm.get_mut(&pa2) {
            None => {
                let _ = perm.insert(pa2, vec![(p.i, p.r)]);
            }
            Some(existing) => existing.push((p.i, p.r)),
        };
        // [t,r,0,0] m3
        let pa3: [u8; 4] = [pt, pr, 0, 0];
        match perm.get_mut(&pa3) {
            None => {
                let _ = perm.insert(pa3, vec![(p.i, p.r)]);
            }
            Some(existing) => existing.push((p.i, p.r)),
        };

        // [t,0,b,l] m4
        let pa4: [u8; 4] = [pt, 0, pb, pl];
        match perm.get_mut(&pa4) {
            None => {
                let _ = perm.insert(pa4, vec![(p.i, p.r)]);
            }
            Some(existing) => existing.push((p.i, p.r)),
        };
        // exclude corners
        if border_count != 2 {
            // [t,0,b,0] m5
            let pa5: [u8; 4] = [pt, 0, pb, 0];
            match perm.get_mut(&pa5) {
                None => {
                    let _ = perm.insert(pa5, vec![(p.i, p.r)]);
                }
                Some(existing) => existing.push((p.i, p.r)),
            };
        }
        // [t,0,0,l] m6
        let pa6: [u8; 4] = [pt, 0, 0, pl];
        match perm.get_mut(&pa6) {
            None => {
                let _ = perm.insert(pa6, vec![(p.i, p.r)]);
            }
            Some(existing) => existing.push((p.i, p.r)),
        };
        // exclude corners
        if border_count != 2 {
            // [t,0,0,0] m7
            let pa7: [u8; 4] = [pt, 0, 0, 0];
            match perm.get_mut(&pa7) {
                None => {
                    let _ = perm.insert(pa7, vec![(p.i, p.r)]);
                }
                Some(existing) => existing.push((p.i, p.r)),
            };
        }

        // [0,r,b,l] m8
        let pb0: [u8; 4] = [0, pr, pb, pl];
        match perm.get_mut(&pb0) {
            None => {
                let _ = perm.insert(pb0, vec![(p.i, p.r)]);
            }
            Some(existing) => existing.push((p.i, p.r)),
        };
        // [0,r,b,0] m9
        let pb1: [u8; 4] = [0, pr, pb, 0];
        match perm.get_mut(&pb1) {
            None => {
                let _ = perm.insert(pb1, vec![(p.i, p.r)]);
            }
            Some(existing) => existing.push((p.i, p.r)),
        };
        // exclude corners
        if border_count != 2 {
            // [0,r,0,l] m10
            let pb2: [u8; 4] = [0, pr, 0, pl];
            match perm.get_mut(&pb2) {
                None => {
                    let _ = perm.insert(pb2, vec![(p.i, p.r)]);
                }
                Some(existing) => existing.push((p.i, p.r)),
            };
            // [0,r,0,0] m11
            let pb3: [u8; 4] = [0, pr, 0, 0];
            match perm.get_mut(&pb3) {
                None => {
                    let _ = perm.insert(pb3, vec![(p.i, p.r)]);
                }
                Some(existing) => existing.push((p.i, p.r)),
            };
        }
        // [0,0,b,l] m12
        let pb4: [u8; 4] = [0, 0, pb, pl];
        match perm.get_mut(&pb4) {
            None => {
                let _ = perm.insert(pb4, vec![(p.i, p.r)]);
            }
            Some(existing) => existing.push((p.i, p.r)),
        };
        // exclude corners
        if border_count != 2 {
            // [0,0,b,0] m13
            let pb5: [u8; 4] = [0, 0, pb, 0];
            match perm.get_mut(&pb5) {
                None => {
                    let _ = perm.insert(pb5, vec![(p.i, p.r)]);
                }
                Some(existing) => existing.push((p.i, p.r)),
            };
            // [0,0,0,l] m14
            let pb6: [u8; 4] = [0, 0, 0, pl];
            match perm.get_mut(&pb6) {
                None => {
                    let _ = perm.insert(pb6, vec![(p.i, p.r)]);
                }
                Some(existing) => existing.push((p.i, p.r)),
            };
            // [0,0,0,0] m15 <<<< mask_15 Not needed as this is $* (The union of all tiles)
        }
    }
    perm
}

use std::convert::TryInto;

/// x, y, direction enum[cw_spiral, acw_spiral, row_by_row, row_snake, col_by_col, col_snake, spiral_out, bottom_right_triangle, ...]
fn gen_path(sizex: usize, sizey: usize, spiral: bool) -> Vec<usize> {
    //println!("board dimentions: {} x {}", sizex, sizey);

    //let mut path: Vec<usize> = vec![0; (sizex * sizey).try_into().unwrap()];
    let area = sizex * sizey;
    let mut path: Vec<usize> = vec![0; area];

    let mut x: usize = 1; // move 1 square when moving x
    let mut y: usize = 1; // move 1 square when moving y
    let mut xadd: isize = 1; // currently we are moving x
    let mut yadd: isize = 0; // currently we are NOT moving y

    {
        // this "outlines" a plan of the board from which a spiral can be deduced
        let mut outline: Vec<Vec<u8>> = vec![vec![0; sizey + 2]; sizex + 2];

        // bottom and top sides
        let top_width = sizex + 1;
        //for a in 0..top_width {
        for a in outline.iter_mut().take(top_width) {
            a[0] = 1;
            a[sizey + 1] = 1;
        }

        // left and right sides
        let side_height = sizey + 1;
        for a in 0..side_height {
            outline[0][a] = 1;
            outline[sizex + 1][a] = 1;
        }

        // fill the search path
        //for aa in 0..area {
        for (aa, pi) in path.iter_mut().enumerate().take(area) {
            //path[aa] = ((y - 1) * sizex + x) - 1;
            *pi = ((y - 1) * sizex + x) - 1;
            outline[x][y] = 1;
            if !spiral {
                //path[aa]  = aa;
                *pi = aa;
            }
            let mut newxadd: isize = 0;
            let mut newyadd: isize = 0;

            let xi: isize = (x).try_into().unwrap();
            let yi: isize = (y).try_into().unwrap();
            let xxadd_u: usize = (xi + xadd).try_into().unwrap();
            let yyadd_u: usize = (yi + yadd).try_into().unwrap();
            // here we look ahead at the next position in the current direction
            if outline[xxadd_u][yyadd_u] == 1 {
                // and change if needed
                if xadd == 1 {
                    newyadd = 1;
                }
                if yadd == 1 {
                    newxadd = -1;
                }
                if xadd == -1 {
                    newyadd = -1;
                }
                if yadd == -1 {
                    newxadd = 1;
                }
                xadd = newxadd;
                yadd = newyadd;
            }
            let xi: isize = (x).try_into().unwrap();
            let yi: isize = (y).try_into().unwrap();
            x = (xi + xadd).try_into().unwrap();
            y = (yi + yadd).try_into().unwrap();
        }
    }
    path
}

// this is used as a lookup for (tile_id, rotation) => [all,four,of the, edges]
fn gen_tiles_map(ts: &[Tile]) -> HashMap<(u16, u8), [u8; 4]> {
    let mut tiles_map: HashMap<(u16, u8), [u8; 4]> = HashMap::new();

    for t in ts.iter() {
        // iterate the tiles (we could probably overload gen_tiles for this!!!)
        let [pt, pr, pb, pl] = t.e;
        tiles_map.insert((t.i, t.r), [pt, pr, pb, pl]);
    }
    tiles_map
}

pub struct ProcedCfg(
    pub Cnf,
    pub Vec<(u16, u8)>,
    pub Vec<usize>,
    pub HashMap<[u8; 4], Vec<(u16, u8)>>,
    pub Vec<u16>,
    pub HashMap<(u16, u8), [u8; 4]>,
);

/*
    c WAS a HashMap of the config, from which we can solve the puzzle
    now it is a struct
*/
/// (Cnf) -> proced_cfg(
///    Cnf: Cnf,
///    board: Vec<(u16,u8)>,
///    path: Vec<usize>,
///    perm: HashMap,
///    trih_vec: Vec<u16>
///    )
pub fn pre_process_cfg(mut c: Cnf) -> ProcedCfg {
    let x_y: usize = (c.x * c.y).into();
    {
        let c_string = format!("{:?}", c);
        let c_bytes = c_string.into_bytes();
        let c_hash = blake3::hash(&c_bytes);
        let mut var_dir: &str = &c.solutions_dir;
        match var_dir.len() {
            0 => var_dir = "./var",
            //1..=65536 => info("[i] var_dir looks good"),
            //1..=65536 => info(&"[i] var_dir looks good".to_string()),
            1..=65536 => {} //info(&"[i] var_dir looks good".to_string()),
            _ => panic!("[e] missing solutions_dir {:?}", var_dir),
        };

        // create the var dir if it is missing
        if !path_exists(var_dir) {
            if let Err(e) = fsio::mkdir(var_dir) {
                panic!("[e] unable to create var_dir: {}", e)
            };
        }

        let blake_str = &c_hash.to_hex()[..].to_string();
        c.blake_str = blake_str.clone();

        // Update conf_file with hash value
        let this_section: Option<&str> = Some("hashes");
        let this_key: String = "blake_str".to_string();

        // add the hash to the conf.ini (if it isn't already set)
        {
            update_cf(&c.filename, this_section, &this_key, blake_str);
        }

        let my_uuid = Uuid::parse_str(&blake_str[0..32]).unwrap();
        c.uuid = my_uuid.hyphenated().to_string();

        // add the uuid to the conf.ini (if it isn't already set)
        let this_key: String = "uuid".to_string();
        {
            update_cf(&c.filename, this_section, &this_key, &c.uuid);
        }

        let cfg_dir = format!("{}/{}", &var_dir, my_uuid.hyphenated());

        // create the dir to hold the serde files (for future faster parsing)
        if !path_exists(&cfg_dir) {
            if let Err(e) = fsio::mkdir(&cfg_dir) {
                panic!("[e] unable to create cfg_dir: {}", e)
            };
        }

        // if cfg_dir exists and we haven't written the serde Serilaised(cfg)
        // as a file, then do that.
        let serde_cfg = serde_json::to_string(&c).unwrap();
        let cfg_serde_filename = format!("{}/{}", &cfg_dir, "cfg.serde");
        let _ = fsio::write(&cfg_serde_filename, &serde_cfg);
    }

    // tiles are all of the possible rotations of the pieces
    let tiles = gen_tiles(&c.pieces);

    // generate the permutations (which are permenant) from the tiles
    let perm = gen_perm(&tiles);
    let tiles_map: HashMap<(u16, u8), [u8; 4]> = gen_tiles_map(&tiles);

    let board: Vec<(u16, u8)> = vec![(0, 0); x_y];
    // generate a path around the board. "true" := spiral in to the middle. "false" := row by row
    let path = gen_path(c.x.try_into().unwrap(), c.y.try_into().unwrap(), true);

    let mut trih_vec: Vec<u16> = vec![];
    for px in &c.pieces {
        trih_vec.push(px.i);
    }
    //let trih: [u16; trih_vec.len()] = trih_vec as Array; // for now we are using Vec rather than array
    //   we might use https://github.com/fizyk20/generic-array later or https://stackoverflow.com/a/29570662/1153645
    // println!("]");

    trih_vec.sort_unstable();
    ProcedCfg(c, board, path, perm, trih_vec, tiles_map)
}
