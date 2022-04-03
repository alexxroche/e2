use std::collections::HashMap;
use std::error::Error;
//extern crate signal_hook;
//use signal_hook::{iterator::Signals, SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use std::ptr::NonNull;

use crate::cfg::Cnf;
use crate::log::*;
use crate::st::St;

use std::convert::TryInto;

use crate::st::LhpResult;

// now
use chrono::{DateTime, Utc};

//unsafe impl core::marker::Send for St {}
// evil code just to make last_gasp work
struct Wrapper(NonNull<St>);
unsafe impl std::marker::Send for Wrapper {}

fn n_mod_m<T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy>(n: T, m: T) -> T {
    ((n % m) + m) % m
}

#[rustfmt::skip]
fn index_to_coords(x: &u16, i: &usize) -> (usize, usize) {
    //( n_mod_m(*i, *x as usize)+1, (*i / (*x as usize))+1 )  // one-indexed
    (n_mod_m(*i, *x as usize), (*i / (*x as usize))) // zero-indexed
}

/// we shift the coordinates (x+1,y+1) so that (0,0) can be None
/// we reverse this when we convert from coordinate -> board_index
fn neighbours(xpos: &usize, ypos: &usize, x: &u16, y: &u16) -> [(u16, u16); 4] {
    // if we change -> (usize,usize) we could remove xp_u16,yp_u16 and golf 56 char in this function

    // neighbour_top, neigh_r, n_b, nl  [we mark them all as being "off the board"]
    let [mut nt, mut nr, mut nb, mut nl] = [(0u16, 0u16), (0u16, 0u16), (0u16, 0u16), (0u16, 0u16)];
    let x_us: usize = (*x).try_into().unwrap();
    let y_us: usize = (*y).try_into().unwrap();
    let xp_u16: u16 = (*xpos).try_into().unwrap();
    let yp_u16: u16 = (*ypos).try_into().unwrap();

    match xpos {
        0 => nr = (xp_u16 + 2, yp_u16 + 1),
        xp if *xp >= 1 && *xp < x_us - 1 => {
            nl = (xp_u16, yp_u16 + 1);
            nr = (xp_u16 + 2, yp_u16 + 1);
        }
        xp if *xp == x_us - 1 => {
            nl = (xp_u16, yp_u16 + 1);
        }
        _ => {
            // any other value is invalid (so we clear out any mistakes)
            return [(0u16, 0u16), (0u16, 0u16), (0u16, 0u16), (0u16, 0u16)];
            // otherwise we can get "undefined behaviour" for (xpos > x, ypos <= y)
        }
    };
    match ypos {
        0 => nb = (xp_u16 + 1, yp_u16 + 2),
        yp if *yp >= 1 && *yp < y_us - 1 => {
            nt = (xp_u16 + 1, yp_u16);
            nb = (xp_u16 + 1, yp_u16 + 2);
        }
        yp if *yp == y_us - 1 => {
            nt = (xp_u16 + 1, yp_u16);
        }
        _ => {
            // any other value is invalid (so we clear out any mistakes)
            return [(0u16, 0u16), (0u16, 0u16), (0u16, 0u16), (0u16, 0u16)];
        }
    };

    [nt, nr, nb, nl]
}

/// returns the pieces [(tile_id,rotation)] of the
fn board_coordinate_to_board_array_index(x: &u16, coordinate: (u16, u16)) -> Option<usize> {
    //let (xpos,ypos) = coordinate;
    match coordinate {
        (0, 0) => None,
        // ypos-1 * x plus xpos-1 plus {correction for one-index}
        (xpos, ypos) if xpos >= 1 && ypos >= 1 => {
            Some((((ypos - 1) * x) + (xpos - 1) + 1).try_into().unwrap())
        }
        //(xpos, ypos) => Some(0_usize),
        _ => Some(0_usize),
    }
}

fn neighbour_edges(
    c: &Cnf,
    board: &[(u16, u8)],
    path_index: &usize,
    tiles: &HashMap<(u16, u8), [u8; 4]>,
) -> [u8; 4] {
    // we pre suppose that the tile has no known edges (N.B. 1==border of the enture board; 0==unknow)
    //let (te, re, be, le): (u8, u8, u8, u8) = (0, 0, 0, 0);

    // find the pos from index  N.B. xpos,ypos are zero-indexed!!
    let (xpos, ypos): (usize, usize) = index_to_coords(&c.x, path_index);

    let [nt, nr, nb, nl] = neighbours(&xpos, &ypos, &c.x, &c.y);

    // tt := top_tile rt := right_tile bt := bottom_tile lt := left_tile
    let tt = board_coordinate_to_board_array_index(&c.x, nt);
    let rt = board_coordinate_to_board_array_index(&c.x, nr);
    let bt = board_coordinate_to_board_array_index(&c.x, nb);
    let lt = board_coordinate_to_board_array_index(&c.x, nl);

    // top_edge
    let te = match tt {
        None => 1, //  we want a perimeter of the board (which we have as 1_u8 )
        Some(i) => {
            let piece = board[i - 1];
            match piece {
                (0, 0) => 0_u8,
                (_, _) => {
                    //match tiles.get(&(piece.0, piece.1)) { Some(e) => e[2], None => 0_u8 }
                    let [_top_e, _right_e, bottom_e, _left_e] = tiles
                        .get(&(piece.0, piece.1))
                        .unwrap_or(&[0_u8, 0_u8, 0_u8, 0_u8]);
                    // now we need to find the bottom edge of the piece above ours
                    *bottom_e // 0 means "wildcard"
                }
            }
        }
    };

    // right_edge
    let re = match rt {
        None => 1, //  we want a perimeter of the board (which we have as 1_u8 )
        Some(i) => {
            let piece = board[i - 1];
            match piece {
                (0, 0) => 0_u8,
                (_, _) => {
                    //match tiles.get(&(piece.0, piece.1)) { Some(e) => e[3], None => 0_u8 }
                    let [_top_e, _right_e, _bottom_e, left_e] = tiles
                        .get(&(piece.0, piece.1))
                        .unwrap_or(&[0_u8, 0_u8, 0_u8, 0_u8]);
                    // now we need to find the right edge of the piece to the left of ours
                    *left_e // 0 means "wildcard"
                }
            }
        }
    };

    // bottom_edge
    let be = match bt {
        None => 1, //  we want a perimeter of the board (which we have as 1_u8 )
        Some(i) => {
            let piece = board[i - 1];
            match piece {
                (0, 0) => 0_u8,
                (_, _) => {
                    //match tiles.get(&(piece.0, piece.1)) { Some(e) => e[0], None => 0_u8 }
                    let [top_e, _right_e, _bottom_e, _left_e] = tiles
                        .get(&(piece.0, piece.1))
                        .unwrap_or(&[0_u8, 0_u8, 0_u8, 0_u8]);
                    // now we need to find the right edge of the piece to the left of ours
                    *top_e // 0 means "wildcard"
                }
            }
        }
    };

    // left_edge
    let le = match lt {
        None => 1, //  we want a perimeter of the board (which we have as 1_u8 )
        Some(i) => {
            let piece = board[i - 1];
            match piece {
                (0, 0) => 0_u8,
                (_, _) => {
                    //match tiles.get(&(piece.0, piece.1)) { Some(e) => e[1], None => 0_u8 }
                    let [_top_e, right_e, _bottom_e, _left_e] = tiles
                        .get(&(piece.0, piece.1))
                        .unwrap_or(&[0_u8, 0_u8, 0_u8, 0_u8]);
                    // now we need to find the right edge of the piece to the left of ours
                    *right_e // 0 means "wildcard"
                }
            }
        }
    };

    [te, re, be, le] //top left corner
}

#[rustfmt::skip]
fn bp_order_to_absolute(nt: &[(u16, u8)], bp: &[usize], prih: &mut Vec<u16>,) -> Vec<(u16, u8)> {
    let mut next_board: Vec<(u16, u8)> = vec![(0, 0); bp.len()];

    // then we iter new_tiles and place them
    for (i, t) in nt.iter().enumerate() {
        next_board[bp[i as usize]] = *t;
        match *t {
            (pn, r) if r <= 4 => {
                //pieces_placed.push(pn),
                let index_within_prih = match prih.iter().position(|&r| r == pn) {
                    Some(i) => i,
                    _ => {
                        panic!("[p] no {} found within prih", pn);
                        //continue;
                    }
                };
                let mut _placed_from_hand =
                    prih.splice(index_within_prih..=index_within_prih, vec![]);
            }
            (pn, r) if r > 4 => {
                warn(format!(
                    "Tile {:?} is being inspected by a sister thread",
                    pn
                ));
            }
            _ => panic!("[p] {:?} isn't a valid tile", *t),
        }
    }
    next_board
}

// takes in the Vec from bp_order_to_absolute() and the board width and produces html
pub fn board_to_html(bp_order: &[(u16, u8)], x_size: u16, y_size: u16, file_name: Option<String>) {
    use crate::{fsio, path_exists};
    use std::fmt;

    let puzzle_string: String = format!("{}_{}_{}", x_size, y_size, x_size * y_size);
    let puzzle_dimentions: String = format!("{}{}{}", x_size, y_size, x_size * y_size);
    let puzzle: &str = &puzzle_string;
    let html_path = format!("./var/{}/", &puzzle_string);
    let html_index_file = format!("./var/{}/index.html", &puzzle_string);
    let mut file_path: String = match file_name {
        Some(fl_name) => fl_name,
        _ => fmt::format(format_args!(
            "./var/{}/{}0001.html",
            &puzzle_string, &puzzle_dimentions
        )),
    };

    // save time and RAM and disk IO by not over-writing
    while path_exists(&file_path) {
        let path_parts: Vec<&str> = file_path.split('/').collect::<Vec<&str>>();
        let file_parts: Vec<&str> = path_parts[path_parts.len() - 1]
            .split('.')
            .collect::<Vec<&str>>();
        let file: isize = file_parts[0].parse::<isize>().unwrap();
        let extension: &str = file_parts[1];
        let filename = format!("{}.{}", file + 1, extension);
        file_path = fmt::format(format_args!("{}{}", &html_path, filename));
    }

    let mut html: String = String::new();
    let mut html_body: String = String::new();

    let mut tile_count: usize = 0;
    for (i, t) in bp_order.iter().enumerate() {
        let rotation = if x_size >= 3 {
            match t.1 {
                1 => 3,
                2 => 4,
                3 => 1,
                4 => 2,
                _ => 0,
            }
        } else {
            t.1
        };
        if rotation >= 1 {
            tile_count += 1;
        }
        html_body.push_str(&format!(
            "<td><img src=\"tiles/{puzzle}-p{tid}-{r}.png\" alt=\"{puzzle}-p{tid}-{r}.png\" title=\"{tid}\" /></td>",
            puzzle = puzzle,
            tid = (t.0) + 1,
            r = rotation
        ));
        //if  ((i % x_size) + x_size) % x_size == 0 { println!("</tr><tr>"); }
        if n_mod_m(i + 1, x_size as usize) == 0 {
            html_body.push_str("</tr><tr>");
        }
    }

    html_body.push_str(&format!(
        "</tr>
</table>
<span>Soltion {} </span>
<!-- /tiles -->
</div>
<pre>{:?}</pre>
</div>
 <script src=\"../js/e2.js\" type=\"text/javascript\"></script>
</body></html>",
        tile_count, bp_order
    ));

    html.push_str(&format!(
        "<!doctype html><html><head><meta charset=\"utf-8\">
<title>A Solution {0}</title>
<!--link rel=\"canonical\" href=\"https://localhost/tiles.html\"-->
<link media=\"only screen and (min-width: 200px)\"
         href=\"../css/mobile.css\" type=\"text/css\" rel=\"stylesheet\" />
<link media=\"screen and (min-width: 600px)\" rel=\"stylesheet\"
         href=\"../css/main.css\">
<link rel=\"stylesheet\" href=\"../css/tiles.css\">
<script src=\"../js/mithril.min.js\"></script>

</head><body class=\"vsc-initialized weather snow body\">
<div class=\"center\" style=\"opacity: 100;\">
<div style=\"padding-left:31%\">

<!-- tiles -->
<table title=\"Solution {0}\">
<!--caption> Soltion {0} </caption-->
<tr>",
        tile_count
    ));
    html.push_str(&html_body);
    let _ = fsio::write(&file_path, &html);
    //info(format!("written HTML to {:?}", &file_path));

    use std::fs;
    use std::os::unix::fs as unix_fs;
    //let sl_bad = fs::symlink(&html_index_file, &file_path);
    let path_parts: &Vec<&str> = &file_path.split('/').collect::<Vec<&str>>();
    let file_name: &str = path_parts[path_parts.len() - 1];
    if path_exists(&html_index_file) {
        match fs::remove_file(&html_index_file) {
            Ok(_) => {}
            Err(e) => {
                warn(format!("unable to remove {:?} : {:?}", &html_index_file, e));
            }
        };
    }
    //match fs::symlink(&file_path, &html_index_file) {
    match unix_fs::symlink(&file_name, &html_index_file) {
        Ok(_) => (),
        Err(_e) => {}
    };
}

fn remove_corners_from_tiles(tls_tr_int: Vec<(u16, u8)>, corners: &[u16]) -> Vec<(u16, u8)> {
    //tls_tr_int.iter().filter(|(t,_)| !corners.contains(t)).collect::<Vec<(u16,u8)>>()
    let mut cleaned_tiles: Vec<(u16, u8)> = vec![];
    for tile in tls_tr_int.iter() {
        match tile {
            (t, _) if !corners.contains(t) => cleaned_tiles.push(*tile),
            _ => (),
        };
    }
    cleaned_tiles
}

// fn save_st to disk
pub fn store_st(s: &St, c: &Cnf) {
    use crate::{fsio, path_exists};

    let mut var_dir: &str = &c.solutions_dir;
    match var_dir.len() {
        l if l < 1 => var_dir = "var",
        _ => (),
    };
    let mut uuid: &str = &c.uuid;
    match uuid.len() {
        l if l < 1 => uuid = "uuid-missing",
        _ => (),
    };
    let cfg_dir = format!("{}/{}", &var_dir, &uuid);

    let st_json = match serde_json::to_string(&s) {
        Ok(j) => j,
        Err(e) => panic!("[p] unable to serialise St: {:?}", e),
    };

    if !path_exists(&cfg_dir) {
        let should_exist = fsio::mkdir(&cfg_dir);
        if !path_exists(&cfg_dir) {
            panic!("[e] var dir {:?}", should_exist);
        }
    }

    info("Saving St to disk");
    let st_serde_filename = format!("{}/{}", &cfg_dir, "St.serde");
    log(format!("storing St to disk: {:?}", &c.uuid));
    if path_exists(&cfg_dir) {
        // we could make a backup of any existing St.serde => $(now)_St.serde {if we hate having disk space, IO, speed}
        if let Err(e) = fsio::write(&st_serde_filename, &st_json) {
            err(format!("Failed to store St: {:#?}", e));
            panic!("You should fix this");
        };
    } else {
        warn(format!("var dir {:?} missing", &cfg_dir));
    }
}

//fn fetch_st(c: &Cnf) -> Option<St> {
fn fetch_st(c: &Cnf) -> St {
    use crate::{fsio, path_exists};

    let mut var_dir: &str = &c.solutions_dir;
    match var_dir.len() {
        l if l < 1 => var_dir = "var",
        _ => (),
    };
    let mut uuid: &str = &c.uuid;
    match uuid.len() {
        l if l < 1 => uuid = "uuid-missing",
        _ => (),
    };
    let cfg_dir = format!("{}/{}", &var_dir, &uuid);

    if !path_exists(&cfg_dir) {
        let _var_dir_created = fsio::mkdir(&cfg_dir);
        //println!("{:?}", var_dir_created);
    }

    let st_serde_filename = format!("{}/{}", &cfg_dir, "St.serde");
    if path_exists(&st_serde_filename) {
        match fsio::read(&st_serde_filename) {
            Err(e) => {
                err(format!("Failed to read St from disk: {:#?}", e));
                panic!("You should fix this");
            }
            Ok(json) => {
                use serde::Deserialize;
                use serde_json::Value;

                let mut deserializer = serde_json::Deserializer::from_str(&json);
                deserializer.disable_recursion_limit();
                let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
                let value = Value::deserialize(deserializer).unwrap();

                return serde_json::from_value(value).unwrap();
            }
        };
    } else {
        info(format!(
            "St.serde {:?} not found (Is this a first run?)",
            &cfg_dir
        ));
        step();
    }
    // generate a new St if we didn't find a valid one on disk
    St::new()
}

///  puzzle over the board
pub fn puzzle(
    c: Cnf,
    board: Vec<(u16, u8)>,
    board_path: Vec<usize>,
    perm: HashMap<[u8; 4], Vec<(u16, u8)>>,
    trih: Vec<u16>,
    tiles: HashMap<(u16, u8), [u8; 4]>,
) -> Result<(), Box<dyn Error>> {
    // bpi (board_path_index) (where along the board_path we are)
    //let mut bpi: usize = 0;
    //let mut previous_board = board.clone();
    // able to deduce this from board for a spiral, but if we are searching using a random walk, board_path_index is needed)
    use crate::st::{c_vec_from_tiles, graft, node_children, update_node};
    use crate::util::intersection_of_trih_and_vec;
    let mut s: St = fetch_st(&c);
    {
        let existing_st: String = format!("{:?}", &s);
        let st_debug = serde_json::to_string(&s).unwrap();
        if existing_st.len() >= 100 {
            log(format!(
                "imported St from disk [happy days]: {:?}",
                st_debug.len()
            ));
        }
    }

    // max depth down the tree... so far (also the "don't bother printing any solutions with fewer than this many placed tiles)
    //let mut most_found = 185;
    let mut most_found = 184;
    //let mut most_found = 178;

    // hook in the last_gasp function to capture
    //  and process any process Signals
    use crate::last_gasp::is_sig;
    let _ = crate::last_gasp::hook();

    let mut prih_cleaned = trih;
    // We may have tiles already placed on the board, so we have to remove those from the trih
    for placed_tile in &board {
        match *placed_tile {
            (_, 0) => continue, // This is a blank space on the board
            (ti, _) => {
                let index_within_prih = match prih_cleaned.iter().position(|&r| r == ti) {
                    Some(i) => i,
                    _ => {
                        warn(&format!("[w] no {} found within prih", ti));
                        continue;
                    }
                };
                #[rustfmt::skip]
                let mut placed_from_hand =
                    prih_cleaned.splice(index_within_prih..=index_within_prih, vec![]);
                pass(&format!(
                    "[i] we have already placed piece {}",
                    &placed_from_hand.next().unwrap()
                ));
            }
        }
    }

    let corners = match perm.get(&[1, 0, 0, 1]) {
        Some(ting) => {
            let mut gathered_tile_ids: Vec<u16> = vec![];
            for t in ting.iter() {
                let (tid, _) = t;
                gathered_tile_ids.push(*tid);
            }
            gathered_tile_ids
        }
        _ => panic!("We were unable to call the corners"),
    };

    // the core of this program: a loop{ St.walk_the_tree }
    loop {
        // check if a signal has been received
        if let true = is_sig() {
            crate::util::process_signal(&s, &c);
        }

        let mut prih = prih_cleaned.clone();

        // N.B. "new_tiles" is in board_path order sooo we MUST convert it before using it to create next_board

        let (new_tiles, stn_path) = match s.left_hand_path() {
            Ok(LhpResult(b, p)) => {
                //info(&"LHP found a path");
                (b, p)
            }
            Err(e) => match e {
                "Zero size SearchTree" => (vec![], vec![]), // to be expected: if the St is blank then the LHP is blank
                _ => {
                    // uh-oh! looks like an error
                    warn(format!("LHP FAILED!!!: {:?}", &e));
                    (vec![], vec![])
                }
            },
        };
        let bpi = new_tiles.len();

        // convert the tiles from the searchTree order {which is in board_path_order} into our absolute board array
        let next_board = bp_order_to_absolute(&new_tiles, &board_path, &mut prih);

        /* OK ! if bpi == board.len() then we have a solution!

            Joyous happy days!
            We can write that to disk (yes!)
            We can prune that branch from the tree (as long as it is marked properly so that no other thread tries it.)
            We can stop (optional)
            We can walk the tree and mark the first node that has no siblings as being a solution (_,55) "five-by-five" i.e. solved
                [ requires that the left_hand_path be restricted to (_,[1,2,3,4]) ] <<<<< todo!
        */
        if bpi == board.len() {
            warn(format!(
                "Joyous happy day! A solution has been found [row-by-row board order]: {:?}",
                next_board
            ));
            store_st(&s, &c);
            let dt: DateTime<Utc> = Utc::now();
            let now = dt.format("12%Y%m%d%H%M%S").to_string();
            // NTS this will fail if we we find more than one solution per second
            #[rustfmt::skip]
          board_to_html(&next_board, c.x, c.y, Some(format!("./var/{}_{}_{}/Solution_{}{}", &c.x, &c.y, { c.x * c.y }, now, ".html")));
            //board_to_html(&next_board, c.x, c.y, None);
            s.dump();
            //s.dumper();
            std::process::exit(0);
        }

        let ni = neighbour_edges(&c, &next_board, &board_path[bpi], &tiles);

        // use the neighbour edges to retrieve the possible tiles from the perm HashMap
        match perm.get(&ni) {
            Some(tls) => {
                // take into account prih (Pieces remainin the hand, i.e. the ones that can still be placed)
                let tiles_tr_intersection = intersection_of_trih_and_vec(&prih, tls);

                let border_count: u8 = (&ni)
                    .iter()
                    .map(|z| if z == &1_u8 { z } else { &0_u8 })
                    .sum();

                let tls_tr_int;
                if border_count == 1 {
                    tls_tr_int = remove_corners_from_tiles(tiles_tr_intersection, &corners);
                } else {
                    tls_tr_int = tiles_tr_intersection;
                }

                if tls_tr_int.is_empty() {
                    // then we have no candidate tiles for this branch and need to "prune" the parent Node
                    let mut st_path_to_parent = stn_path.clone();
                    st_path_to_parent.pop(); // take a step back up the tree

                    match bpi {
                        0 => {
                            panic!("bpi is zero but we are trying to remove a tile????");
                        }
                        _ => {
                            //match &next_board[board_path[bpi - 2]] {
                            match &next_board[board_path[bpi - 1]] {
                                (t, _) if t > &0 => {
                                    if bpi > most_found {
                                        store_st(&s, &c);
                                        // This will serialise the searchTree (serde) and store it in var/08135070-6c2f-d57d-cf45-92b6f5f6676f/st.serde
                                        // Q. is this the optimal place for store_st? Should we have it in other places as well, or instead
                                        let dt: DateTime<Utc> = Utc::now();
                                        let now = dt.format("19%Y%m%d%H%M").to_string();
                                        #[rustfmt::skip]
                                        board_to_html(&next_board, c.x, c.y, Some(format!("./var/{}_{}_{}/{}{}", &c.x, &c.y, { c.x * c.y }, now, ".html")));
                                        most_found = bpi;
                                    }
                                    #[rustfmt::skip]
                                    let _ = update_node(&mut s, st_path_to_parent, Some((261, 0)), None,);
                                }
                                _ => {
                                    #[rustfmt::skip]
                                    err(format!("here be DrAgons: {:?}", &next_board[board_path[bpi - 1]]));
                                    #[rustfmt::skip]
                                    err(format!("next_board: {:?}; bpi: {:?}", &next_board, &bpi));
                                    #[rustfmt::skip]
                                    err(format!("Grrr dragons HERE: {:?}", &next_board[board_path[bpi - 1]]));
                                }
                            };
                        }
                    };
                } else {
                    let ns: Vec<St> = c_vec_from_tiles(tls_tr_int); //new SearchTree
                    for new_child in ns {
                        let c_gs = stn_path.clone();
                        let _ = graft(&mut s, c_gs, new_child);
                    }
                }
            }
            _ => {
                let mut child_tally: u8 = 0;

                // AS a hail-mary lets update this node
                let this_node = stn_path.clone();
                //this_node.pop(); // take a step back up the tree
                match &next_board[board_path[bpi - 1]] {
                    // N.B. [bpi-1] because we have bpi == {the new location that we do not have}
                    (t, _) if t > &0 => {
                        if bpi > most_found {
                            store_st(&s, &c);
                            let hour_check: DateTime<Utc> = Utc::now();
                            let now = hour_check.format("94%Y%m%d%H%M").to_string();
                            #[rustfmt::skip]
                            board_to_html(&next_board, c.x, c.y, Some(format!("./var/{}_{}_{}/{:0>3}{}{}", &c.x, &c.y, { c.x * c.y }, &bpi, now, ".html")));
                            most_found = bpi;
                        } //endif

                        let _ = update_node(&mut s, this_node, Some((622, 0)), None);
                    }
                    _ => {
                        err(format!("SR HERE: {:?}", &next_board[board_path[bpi - 1]]));
                        err(format!("next_board: {:?}; bpi: {:?}", &next_board, &bpi));
                    }
                };
                // /AS a hail-mary

                match node_children(&s, stn_path.clone()) {
                    None => {
                        pass("This node has no siblings");
                        s.dump_to_file();
                    }
                    Some(vec_of_children) => {
                        for voc in vec_of_children {
                            match voc.v {
                                // matching rotation to one of the 4 valid rotations
                                //Some((_, rotation)) if rotation >= 1 && rotation <= 4 => {
                                // actually any rotation that isn't zero must NOT have its parent pruned
                                Some((_, rotation)) if rotation != 0 => child_tally += 1,
                                _ => {}
                            };
                        }
                    }
                };

                //let stn_path: Vec<usize> = stn_path.clone();
                //let r = update_node(&mut s, stn_path, Some((0,0)), None ); // update v: and remove c:
                let mut st_path_to_parent = stn_path.clone();
                st_path_to_parent.pop(); // take a step back up the tree
                match st_path_to_parent.len() {
                    0 => {
                        panic!("bpi is zero but we are trying to PRUNE a branch???");
                    }
                    // do we need a special case for root nodes? (or is that just an indicator that our match branches are incorrrectly constructed?
                    1 => {
                        warn("PRUNING root-corner tile!");
                        let _ = update_node(&mut s, stn_path, Some((0, 0)), None);
                        //loop_count += 1;
                        // update v: and remove c:
                    }
                    _ => {
                        // this match should be matching on the length of the c: of the parnet node
                        // so that we can cycle through the siblings of the current node and check that they are all (_,0) before we prune
                        //match &next_board[board_path[bpi - 1]] {
                        //match &next_board[board_path[catch]] {
                        match &next_board[board_path[bpi - 1]] {
                            (t, _r) if t > &0_u16 => {
                                if child_tally == 0 {
                                    let mut parents_siblings_udn = stn_path.clone();
                                    // Can't remember what UDN meant
                                    parents_siblings_udn.pop();
                                    let mut parents_siblings = stn_path.clone();
                                    parents_siblings.pop();
                                    match node_children(&s, parents_siblings) {
                                        None => {
                                            fail(format!("How can we be here with 'None' children of our parents_siblings? {}", &parents_siblings_udn.len()));
                                            let mut grand_parent_siblings = stn_path.clone();
                                            grand_parent_siblings.pop(); //parent generation
                                            grand_parent_siblings.pop(); //grandparent generation

                                            //grand_parent_siblings.pop(); //greatgrandparent generation // I /think/ we can remove this line
                                            let grand_parent_siblings_udn =
                                                grand_parent_siblings.clone();
                                            match node_children(&s, grand_parent_siblings) {
                                                None if !stn_path.is_empty() =>
                                                // is this to bypass the root node?
                                                {
                                                    if bpi > most_found {
                                                        store_st(&s, &c);
                                                        let dt: DateTime<Utc> = Utc::now();
                                                        let now =
                                                            dt.format("31%Y%m%d%H%M").to_string();
                                                        #[rustfmt::skip]
                                                            board_to_html(&next_board, c.x, c.y, Some(format!(
                                                              "./var/{}_{}_{}/{}{}", &c.x, &c.y, { c.x * c.y }, now, ".html")));
                                                        most_found = bpi;
                                                    };

                                                    /*
                                                    //#[cfg_attr(rustfmt, rustfmt::skip)]
                                                    let peek = node_children(
                                                        &s,
                                                        grand_parent_siblings_udn,
                                                    );
                                                    let peek_clone = peek.clone();
                                                    let _ = match peek_clone {
                                                        Some(new_st) => new_st[0].dumper(),
                                                        None => warn("Nothing to peek, dumping entire tree (sorry)"),
                                                    };
                                                    */

                                                    panic(format!("No way there are no parents when we are {:?} into the searchTree", stn_path.clone()))
                                                }
                                                Some(grand_parent_nodes) => {
                                                    info("We are going to prune the parent if we can find it: ".to_string());
                                                    for (ij, gpn) in
                                                        grand_parent_nodes.iter().enumerate()
                                                    {
                                                        gpn.dumper();
                                                        #[rustfmt::skip]
                                                            log(format!("{} Gramps: {:?}", ij, gpn));
                                                    }
                                                    //info(format!("{:?} you are wonding how to get the grand parent v(tile_id, _)", grand_parent_nodes[0].v.unwrap().0));
                                                    //let gpv = &grand_parent_nodes[0].v.unwrap().0;
                                                    //info(format!("{:?} you are wonding how to get the grand parent v(tile_id, _)", &gpv));
                                                    // this is almost certainly a bad idea (we are blindly trimming the grandparent's children?
                                                    //let r = update_node( &mut s, grand_parent_siblings_udn, Some((*gpv, 0)), None,);
                                                    #[rustfmt::skip]
                                                        let _ = update_node(&mut s, grand_parent_siblings_udn, Some((269, 0)), None,);
                                                }
                                                _ => {
                                                    // I think we can safely panic! at this point
                                                    #[rustfmt::skip]
                                                        err(format!( "no uncles or aunts or PARENT! {:?}, {} ", stn_path.clone(), stn_path.len()));
                                                }
                                            };

                                            #[rustfmt::skip]
                                                let _ = update_node( &mut s, parents_siblings_udn, Some((267, 0)), None,);
                                        }
                                        Some(_vec_of_aunts) => {
                                            // 99% sure we are missing this UNTESTED NTS NOTE N.B. TO BE CHECKED!!!
                                            // At this point we have a parent with no children left to check, so we can prune the parent
                                            #[rustfmt::skip]
                                                let _ = update_node(&mut s, parents_siblings_udn, Some((275, 0)), None,);
                                            // _udn := Update_Duplicate_Node ? "take a reciept of the same value to be consumed somewhere else"
                                        }
                                    };

                                    #[rustfmt::skip]
                                        // update v: and remove c:
                                        let _ = update_node(&mut s, st_path_to_parent, Some((268, 0)), None,);
                                };
                            }
                            _ => {
                                #[rustfmt::skip]
                                err(format!( "As you are here it means that [the grandparent tile is zero] <how could that even happen?>: {:?}", &next_board[board_path[bpi - 1]]));
                                err(format!("next_board: {:?}; bpi: {:?}", &next_board, &bpi));
                                #[rustfmt::skip]
                                err(format!("SREAL HERE: {:?}", &next_board[board_path[bpi - 1]]));
                            }
                        };
                    }
                };

                /*
                    1. do we need this?
                    2. does it need to have its own block?
                */
                {
                    //record the tile to be lifted (so that it can be put back "in hand")
                    let tile_for_hand = &next_board[board_path[bpi]];
                    match tile_for_hand {
                        // Q. will rust match
                        // (t, _) if t > &0 => ... with (4,1) AND (4, 1)  or just the latter?
                        // Q. do we need (t,_) if t > &0 => ... as well ?
                        (t, _) if t > &0 => prih.push(*t),
                        _ => {}
                    };
                }
            }
        };
    } // loop
}
