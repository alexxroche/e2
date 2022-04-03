/*
    Our search tree is huge but finite. It can have a maximim of 256 decendends, each with a maximum of 256 children.
*/
use crate::fsio;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
//use serde_json::Result;

//#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize,)]
/// SearchTree struct
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct St {
    pub v: Option<(u16, u8)>, //value
    pub c: Option<Vec<St>>,   // children
}

// return structs
pub struct LhpResult(pub Vec<(u16, u8)>, pub Vec<usize>);
struct IlhpR(Vec<(u16, u8)>, Vec<usize>, i8);

//use crate::log::{log, info, warn, err, step};
use crate::log::*;
use std::default::Default;
/* impl Default for St {
    fn default() -> Self {
        St { v: None, c: None }
    }
} */

impl St {
    /*
    fn show(&self) {
        &self;
    }
    */
    pub fn new() -> St {
        St { ..St::default() }
    }

    fn add_child(&mut self, child: Vec<St>) {
        match &mut self.c {
            None => self.c = Some(child),
            Some(cv) => {
                cv.extend(child);
            }
        };
    }

    // used for testing
    #[allow(dead_code)]
    fn add_st(&mut self, child: St) {
        match &mut self.c {
            None => {
                self.c = Some(vec![child]);
            }
            Some(cv) => {
                cv.push(child);
            }
        };
    }

    /*
    fn board_from_left_path(&self) -> Vec<(u16,u8)> {
        let mut ret: Vec<(u16,u8)> = vec![];
        ret = match &self.v {
            Some(v) => match v {
                    rv => [*rv].to_vec(),
                    //_ => [].to_vec(),
            },

    // YOU got stuck trying to  et cute with recursion

            None =>
                match &self.c {
                /*
                        Some(c) => {
                            for e in c.iter() {
                                     e.board_from_left_path();
                            }
                        },
                */
                        _ => [].to_vec(),
                    },
                 //[].to_vec(),
         };
        ret
    }
    */

    pub fn left_hand_path(&self) -> core::result::Result<LhpResult, &str> {
        let nb: Vec<(u16, u8)> = vec![];
        let np: Vec<usize> = vec![];

        /* // this is more compact...
        let (b, p, _result) = &self
            .iter_left_hand_path(nb, np, 0)
            .unwrap_or((vec![], vec![], 0));

        eprintln!("lhp result: {}", _result);
        */

        // ... but this is better for debug
        let eb: Vec<(u16, u8)> = vec![];
        let ep: Vec<usize> = vec![];
        let (b, p) = match &self.iter_left_hand_path(nb, np, 0) {
            Ok(IlhpR(b, p, _)) => {
                //log(&format!("b: {:?}, p: {:?}", &b, &p));
                ((*b).clone(), (*p).clone())
            }
            Err(e) => {
                warn(&format!("looks like LHP failed: {:?}", e));
                (eb, ep) // return an empty_board, empty_path
            }
        };
        match p.len() {
            //0 => Error::String("Zero size SearchTree"),
            //0 => Error::String,
            0 => Err("Zero size SearchTree"),
            _ => Ok(LhpResult(b.to_vec(), p.to_vec())),
        }
    }

    // called by pub fn left_hand_path
    //fn iter_left_hand_path(&self, &mut nb: Vec<(u16,u8)>, &mut np: Vec<usize>) -> Result<(Vec<(u16,u8)>, Vec<usize>), Error> {
    fn iter_left_hand_path(
        &self,
        nb: Vec<(u16, u8)>,
        np: Vec<usize>,
        rslt: i8,
    ) -> Result<IlhpR, String> {
        //  we need to actually look at &self and get the values (and path)

        let mut tnbr = vec![]; //the new board return
        let mut tnpr = vec![]; //the new path  return

        if rslt < 0 {
            // r := result
            return Ok(IlhpR(nb, np, rslt));
        }
        let mut result: i8 = rslt;

        match &self.c {
            None => {
                return Ok(IlhpR(nb, np, -3)); //return with "end of the branch" signal: -3
            }
            Some(children) => {
                match children {
                    child_list => {
                        for (i, ch) in (*child_list).iter().enumerate() {
                            match ch.v {
                                // if this node is marked for pruning we "continue" to her siblings
                                Some((_, 0)) => {
                                    continue; // to the next sibling
                                              // N.B. NTS: DO WE deal with the case where there are no other siblings?
                                }
                                Some((_, child_rotation)) if child_rotation > 4 => {
                                    log(format!(
                                        "marked branch := {:?}, nb:= {:?}, np:= {:?})",
                                        &ch.v.unwrap(),
                                        &nb,
                                        &np
                                    ));
                                    continue;
                                } //this node is being processed by another thread
                                _ => {} // result = -1; }
                            };
                            match ch.c {
                                None => {
                                    // if nb and np are mutable, then why can't we use them directly?
                                    tnbr = nb; // a clone of the BOARD that was passed into this function
                                    tnpr = np; // a clone of the PATH that was passed into this function
                                    match ch.v {
                                        Some((p, r)) if r > 0 && r <= 4 => {
                                            tnbr.push((p, r));
                                            tnpr.push(i);
                                        }
                                        _ => {
                                            err(&format!(
                                                "LHP_I unable to push ch.v: {:?} onto the board",
                                                ch.v
                                            ));
                                        }
                                    };
                                    return Ok(IlhpR(tnbr, tnpr, -1));
                                    //result = -1; break; //seeing if this avoids iterating other siblings
                                }
                                _ => {
                                    let mut nbr: Vec<(u16, u8)> = nb.clone();
                                    let mut npr: Vec<usize> = np.clone();

                                    match ch.v {
                                        Some((child_piece_id, child_r))
                                            if child_r > 0 && child_r <= 4 =>
                                        {
                                            nbr.push((child_piece_id, child_r))
                                        }
                                        _ => {
                                            err(&format!("LHP_I unable to push ch.v: {:?} despite there being c: Some()", ch.v));
                                            return Err("LHP_I unable to push ch.v: despite there being c: Some(!)".to_string());
                                            //return Err(format!("LHP_I unable to push ch.v: {:?} despite there being c: Some(!)", ch.v).to_string());
                                        }
                                    };
                                    npr.push(i); // np.push(index_of_child_within_the vector)

                                    // recurse down that child
                                    match (ch).iter_left_hand_path(nbr, npr, result) {
                                        Ok(IlhpR(dr_rb, dr_rp, dr_r)) => {
                                            //deep_recusion
                                            tnbr.extend(dr_rb);
                                            tnpr.extend(dr_rp);
                                            result = dr_r;
                                            //log(format!("ilhp: tnbr:= {:?}, tnpr:= {:?}, result:= {})",tnbr, tnpr, result));
                                            //result = 0;
                                            //break;
                                        }
                                        Err(e) => {
                                            err(format!("error: {:?}", e));
                                            panic!();
                                        }
                                    };
                                }
                            };
                            match result {
                                /*
                                    -1 := "we have reached the end of a branch and are walking back
                                    -2 := " .iter_left_hand_path() returned an error for this branch "
                                    -3 := " c: None a.k.a. {end of the branch}
                                    -4 := "all of the children are either marked for pruning or {occupied} so take a step back"
                                */
                                //-1 | -2 | -3 | -4 => break,
                                -1 | -3 => break,
                                -2 | -4 => continue,
                                //_ => break, //No! We MUST continue down this branch
                                _ => continue,
                            };
                        } // for
                    } //_ => { },
                };
            }
        };
        Ok(IlhpR(tnbr, tnpr, result))
    }

    /*
    fn replace_child(&mut self, child: Vec<St>) {
        //self.c = Some(child);
        self.add_child(child);
    }
    fn remove_children(&mut self) {
        self.c = None;
    }

    pub fn print_as_yaml(&self) {
        let s = serde_yaml::to_string(&self);
        println!("{}", s.unwrap_or("".to_string()));
    }

    pub fn print_as_json(&self) {
        let s = serde_json::to_string(&self);
        println!("{}", s.unwrap_or("".to_string()));
    }
    */

    pub fn dump_to_file(&self) {
        self.dump_to_file_loop(0_usize);
        let dt: DateTime<Utc> = Utc::now();
        let now = dt.format("55%Y%m%d%H").to_string();
        #[rustfmt::skip]
        let filename = format!("./var/{}{}", now, ".rs");

        // we now have separate files so do not need a page break
        //let txt = format!("#{}\n", "-".repeat(79));

        use crate::fsio::path_exists;
        let old_file = filename.clone();
        let mut new_file = filename.clone();
        let path_parts: Vec<&str> = filename.split('/').collect::<Vec<&str>>();
        let new_path: &String = &path_parts[0..(path_parts.len() - 1)].join("/");
        let file_parts: Vec<&str> = path_parts[path_parts.len() - 1]
            .split('.')
            .collect::<Vec<&str>>();
        let mut file: isize = (format!("{}0", file_parts[0])).parse::<isize>().unwrap();
        let extension: &str = file_parts[1];
        if path_exists(&new_file) {
            while path_exists(&new_file) {
                file += 1;
                new_file = format!("{}/{}.{}", &new_path, file, extension);
                //filename = format!("{}.{}", file + 1, extension);
            }
            warn(format!(
                "mv filename to prevent overwriting: mv [{:?}] [{:?}]",
                &filename, &new_file
            ));
            use std::fs;
            let mv_action = fs::rename(
                &old_file,
                &new_file, //format!("{:?}{}", &path_parts[0..(path_parts.len() - 2)], filename),
            );
            if let Err(e) = mv_action {
                err(format!("mv {:?} {:?} [{:?}]", &old_file, &new_file, e))
            };
        }
        //let good_write = fsio::append(&filename, &txt);
        // save time and RAM and disk IO by not over-writing
    }

    fn dump_to_file_loop(&self, depth: usize) {
        match &self.v {
            Some(v) => match depth {
                0 => {
                    let dt: DateTime<Utc> = Utc::now();
                    let now = dt.format("55%Y%m%d%H").to_string();
                    #[rustfmt::skip]
                       let filename = format!("./var/{}{}", now, ".rs");
                    let txt = format!("{}{:?}", "".repeat(depth), v);
                    let _good_write = fsio::append(&filename, &txt);
                    //log(format!("Write gud?: {:?}", good_write));
                }
                _ if depth >= 1 => {
                    let dt: DateTime<Utc> = Utc::now();
                    let now = dt.format("55%Y%m%d%H").to_string();
                    #[rustfmt::skip]
                       let filename = format!("./var/{}{}", now, ".rs");
                    let txt = format!("{}{:?}\n", "\t".repeat(depth - 1), v);
                    let _good_write = fsio::append(&filename, &txt);
                    //log(format!("Good write?: {:?}", good_write));
                }
                _ => (),
            },
            None => (),
        }
        match &self.c {
            Some(c) => {
                for e in c.iter() {
                    e.dump_to_file_loop(depth + 1);
                }
            }
            None => (),
        }
    }

    pub fn dumper(&self) {
        self.dump_loop(1_usize);
    }

    pub fn dump(&self) {
        self.dump_loop(0_usize);
    }

    fn dump_loop(&self, depth: usize) {
        match &self.v {
            Some(v) => match depth {
                0 => eprintln!("{}{:?}", "".repeat(depth), v),
                _ if depth >= 1 => eprintln!("{}{:?}", "\t".repeat(depth - 1), v),
                _ => (),
            },
            None => (),
        }
        match &self.c {
            Some(c) => {
                for e in c.iter() {
                    e.dump_loop(depth + 1);
                }
            }
            None => (),
        }
    }

    /*
        fn iter_path<I: Iterator<Item=St>>(tree: &mut I) {
            if let Some(has_children) = tree.next() {
                let mut c = has_children;
                St::iter_path(&mut c);
                tree.c = Some(c.c.unwrap().to_vec());
            }
        }

    */

    /*  // this searches the tree for nodes that have not birthed their children yet N.B. must skip nodes that are already pruned
        fn find_graft_sites(tree: &mut St){
        }
    */
}

// the tuple returned by the StIterator
#[derive(Debug)]
pub struct StItem(Vec<u16>, Vec<(u16, u8)>); // does this have to be a Vec or can one or both be an array?

impl Iterator for St {
    //type Item = (Vec<u16>, Vec<(u16, u8)>); //abstract that for use in the other Iterators
    type Item = StItem;
    fn next(&mut self) -> Option<Self::Item> {
        let mut board: Vec<(u16, u8)> = vec![]; //empty board; we skip the root node because it just exists to hold all of the first pieces
        let path: Vec<u16> = match &mut self.c {
            Some(children) => {
                // add the current value to the board (if there is one)
                if let Some(_value) = self.v {
                    board.push(self.v.unwrap())
                };
                // iterate children and call next(child) on each of them NTS to be written once we grok recursive struct [iter,&iter_mut,&iter]
                let iter_index = 0;
                let step_on_the_path = match &(*children)[iter_index] {
                    St {
                        v: Some(_v),
                        c: Some(_c),
                    } => iter_index,
                    //St {Some(v),Some(c) } => self.v.unwrap(),
                    _ => 0,
                };
                //self.c = Some(children[1..].to_vec());  // cannot assign to `self.c` because it is borrowed
                //self.v = *(children)[0].v;
                self.v = Some((children)[0].v.unwrap());
                vec![step_on_the_path.try_into().unwrap()]
            }
            None => vec![],
        }; // let path:
        Some(StItem(path, board))
    }
}

/*
#[derive(Debug)]
pub enum MyError<'a> {
    TupleType(&'a str, (u16, u8)),
    Option,
    String,
    //Usize,
}

#[derive(Debug)]
pub enum Unwind {
    Option,
    String,
}
*/

/*
pub fn path_to_child<'a>(s: &St, p: Vec<u16>) -> core::result::Result<&St, &'a str> {
    // if we can't p.iter() or we get vec![] then return St
    match p.first() {
        Some(vi) => {
            // we want the value_integer of St.c[vi]
            //let c_list: &Vec<u16> = &p[1..].to_vec();
            match &s.c {
                Some(st) => {
                    // if the path has more steps then we walk the path
                    match p.len() {
                        //pl if pl <= 1 => { Ok(st[*vi as usize].clone()) },
                        pl if pl <= 1 => Ok(&st[*vi as usize]),
                        _ => {
                            //let new_path = (&p[1..]).to_vec().unwrap_or(vec![]);
                            let new_path = (&p[1..]).to_vec();
                            //let new_path = &p[1..].to_vec();
                            //p = p[1..].to_vec();
                            //let new_path: vec = p[1..].copy();
                            //path_to_child(st[*vi as usize].clone(), &p) //
                            //path_to_child(&st[*vi as usize].clone(), new_path) //
                            path_to_child(&st[*vi as usize], new_path) //
                                                                       //path_to_child(st[*vi as usize].clone(), &p[1..].to_vec()) //
                                                                       //path_to_child(st[*vi as usize].clone(), &new_path.to_vec()) //
                        }
                    }
                }
                //Some(st) => path_to_child(&mut st[*vi as usize], &c_list), // Rust: "returns a value referencing data owned by the current function"
                None => Err("No n'th child"),
            }
            //Ok(&(&mut s.c.unwrap())[*vi as usize])
        }
        None => Ok(&s),
    }
}
*/

// Probably MUCH more efficient to specify p: Vec<u16> {or Vec<usize>} rather than messing about with .clone().try_into().unwrap()
//pub fn graft<'a>(s: &mut St, p: Vec<u16>, g: Option<Vec<St>> ) -> core::result::Result<&'a str, &'a str> {
pub fn graft<'a, T: std::clone::Clone + std::convert::TryInto<usize>>(
    s: &mut St,
    p: Vec<T>,
    g: St,
) -> core::result::Result<&'a str, &'a str>
where
    <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug,
{
    // check for None
    match s.c.as_mut() {
        None => {
            /* DEBUG SF (solution found)
            info(format!("add_child {:?} to {:?}", &g, &s));
            */
            // no need to even walking the tree
            s.add_child([g].to_vec());
            Ok("inserted new St in old")
        }
        _ => {
            // its on: lets walk

            let mut result = Ok("Grafted");
            match (*(s.c.as_mut().unwrap())).len() {
                //len if p.len() > 1 && len > p[0] as usize => {
                //len if p.len() > 1 && len > p[0].clone().try_into().unwrap() => { // "if the path_vec_p has more than one step" && "some gibbering"
                // then dive in here for a recursion
                _len if !p.is_empty() => {
                    //len if p.len() > 1 && len > usize::try_from(p[0]).unwrap() => {
                    let mut next_step = p.clone();
                    next_step.drain(0..1); // "shift"
                    let sc_index: usize = (p[0].clone()).try_into().unwrap();
                    result = graft(
                        //&mut ((*(s.c.as_mut().unwrap()))[p[0] as usize]),
                        &mut ((*(s.c.as_mut().unwrap()))[sc_index]),
                        //&mut ((*(s.c.as_mut().unwrap()))[ (p[0]).try_into().unwrap() ]),
                        next_step,
                        g,
                    );
                }
                _ => {
                    match (*(s.c.as_mut().unwrap())).len() {
                        0 => {
                            (*(s.c.as_mut().unwrap())).push(g); // add Child
                        }
                        _ => {
                            (*(s.c.as_mut().unwrap())).push(g); // add Child
                        }
                    }
                } // end _ => {
            };
            result
        }
    }
}

#[cfg(test)]
pub fn st_from_tiles(new_tiles: Vec<(u16, u8)>) -> St {
    let mut c: Vec<St> = vec![];
    for tile in new_tiles {
        c.push(St {
            v: Some(tile),
            c: None,
        });
    }
    St {
        v: None,
        c: Some(c),
    }
}

pub fn c_vec_from_tiles(new_tiles: Vec<(u16, u8)>) -> Vec<St> {
    let mut c: Vec<St> = vec![];
    for tile in new_tiles {
        c.push(St {
            v: Some(tile),
            c: None,
        });
    }
    c
}
/// update_node takes a mutable SearchTree and a path Vec<usize> {to locate the node of the tree}
/// and then replaces v: and c: with v,c
pub fn update_node<'a>(
    s: &mut St,
    mut p: Vec<usize>,
    v: Option<(u16, u8)>,
    c: Option<Vec<St>>,
) -> core::result::Result<&'a str, &'a str> {
    match s.c.as_mut() {
        // check for St{ v:_, c: None }
        None => {
            //s.v = v;
            // if the update request has the tile_id set to >= 260 it is code for "actually just update the rotation"
            let v_ret = s.v;
            let c_ret = s.c.clone();
            log(format!(
                "Update_Node: {:?}; {:?} => {:?}; {:?}",
                v_ret, c_ret, &v, &c
            ));
            match s.v {
                Some((tid, _)) => match v {
                    Some((tile_id, rotation)) if tile_id >= 260 => s.v = Some((tid, rotation)),
                    //Some((tile_id,rotation)) => s.v = v,
                    _ => s.v = v,
                },
                _ => s.v = v,
            };
            s.c = c;
            Ok("Update")
        }
        _ => {
            //let mut result = Err("failed to update node");
            match (*(s.c.as_mut().unwrap())).len() {
                //len if p.len() > 1 => { // slso WORKS for ./test/system.sh
                len if p.len() > 1 && len > p[0] => {
                    // WORKS for ./test/system.sh
                    //len if p.len() > 1 && len > p.len() => { // Failes ./test/system.sh
                    let p_index = p[0]; // "shift"
                    p.drain(0..1); // "shift"
                                   //result = update_node(&mut ((*(s.c.as_mut().unwrap()))[p[0]]), p, v, c);
                    update_node(&mut ((*(s.c.as_mut().unwrap()))[p_index]), p, v, c)
                    //update_node(&mut ((*(s.c.as_mut().unwrap()))[p[0]]), p, v, c)
                }
                _ => {
                    //(*(s.c.as_mut().unwrap()))[0_usize].c = c; // it happens that in our test puzzler 0_usize == p[0] in many cases
                    //(*(s.c.as_mut().unwrap()))[0_usize].v = v; // but I don't think that is what we actuall want.
                    (*(s.c.as_mut().unwrap()))[p[0]].c = c;
                    //(*(s.c.as_mut().unwrap()))[p[0]].v = v;
                    match (*(s.c.as_mut().unwrap()))[p[0]].v {
                        Some((tid, _)) => match v {
                            Some((tile_id, rotation)) if tile_id >= 260 => {
                                (*(s.c.as_mut().unwrap()))[p[0]].v = Some((tid, rotation))
                            }
                            _ => (*(s.c.as_mut().unwrap()))[p[0]].v = v,
                        },
                        _ => s.v = v,
                    };

                    //result = Ok("node updated?");
                    Ok("node updated?")
                }
            }
            //};
            //result
        }
    }
}

// list the node's children (as Option<St> )
//pub fn node_children<'a>(s: &St, mut p: Vec<usize>) -> Option<&Vec<St>> { // maybe this would be faster? (so we can avoid .clone() )
pub fn node_children(s: &St, mut p: Vec<usize>) -> Option<Vec<St>> {
    //let mut result: Option<&Vec<St>> = None; // NTS future optimisation?
    let mut result: Option<Vec<St>> = None;
    match s.c.as_ref() {
        // check for St{ v:_, c: None }
        None => result,
        _ => {
            match (*(s.c.as_ref().unwrap())).len() {
                //is this accidntally working or should it be: len if p.len() > 1 && len > p.len() ) ?
                //len if p.len() > 1 && len > p[0] => {
                _length if p.len() > 1 => {
                    let p_index = p[0]; // do we _actually_ want this?
                                        //seems ok
                                        // 20201016170922 but _is_ it?
                    p.drain(0..1); // "shift"
                    result = node_children(&((*(s.c.as_ref().unwrap()))[p_index]), p);
                    // is this correct?
                    //result = node_children(&((*(s.c.as_ref().unwrap()))[p[0]]), p); // or this?
                }
                _ => {
                    result = Some((*(s.c.as_ref().unwrap())).clone());
                    //result = (*(s.c.as_mut().unwrap()))[0_usize].c.clone();
                }
            };
            result
        }
    }
}

#[cfg(test)]
//pub fn mut_ref_to_child(s: &St, path: &Vec<usize>) -> &mut St {
pub fn mut_ref_to_child<'a>(s: &'a mut St, _path: &[usize]) -> &'a mut St {
    // is this possible?
    // poormans IntoIter HERE trying to drill down "_path" into "s" to find the desired nested St
    &mut *s
}

// avoid having to stick the unit tests for st.rs in src/st/test_st.rs
#[path = "./test_st.rs"]
#[cfg(test)]
mod test_st;
