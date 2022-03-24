#![allow(dead_code)]
extern crate e2_rust;

#[cfg(test)]
#[path = "../src/st.rs"] mod st;
#[path = "../src/log.rs"] mod log;
#[path = "../src/fsio.rs"] mod fsio;
mod i_tests {

    use super::*;
    use log::{err, warn};
    use st::{St, LhpResult};
    //use st::{c_vec_from_tiles, graft, mut_ref_to_child, node_children, update_node, St, LhpResult};
    use once_cell::sync::Lazy;
    //mod common;

    //static TEST_S: St = St { v: None, c: Some(vec![
    static TEST_S: Lazy<St> = Lazy::new(|| St {
        v: None,
        c: Some(vec![
            St {
                v: Some((1, 1)),
                c: Some(vec![St {
                    v: Some((2, 1)),
                    c: None,
                }]),
            },
            St {
                v: Some((1, 2)),
                c: Some(vec![
                    St {
                        v: Some((2, 1)),
                        c: Some(vec![
                            St {
                                v: Some((3, 2)),
                                c: None,
                            },
                            St {
                                v: Some((3, 2)),
                                c: None,
                            },
                        ]),
                    },
                    St {
                        v: Some((2, 3)),
                        c: Some(vec![
                            St {
                                v: Some((3, 0)),
                                c: None,
                            },
                            St {
                                v: Some((3, 2)),
                                c: None,
                            },
                        ]),
                    },
                ]),
            },
            St {
                v: Some((1, 3)),
                c: Some(vec![St {
                    v: Some((2, 3)),
                    c: None,
                }]),
            },
        ]),
    });
    //};
    static TEST_T: Lazy<St> = Lazy::new(|| St {
        v: Some((199, 2)),
        c: Some(vec![
            St {
                v: Some((71, 1)),
                c: Some(vec![St {
                    v: Some((7, 1)),
                    c: None,
                }]),
            },
            St {
                v: Some((71, 2)),
                c: Some(vec![
                    St {
                        v: Some((72, 2)),
                        c: Some(vec![
                            St {
                                v: Some((73, 2)),
                                c: None,
                            },
                            St {
                                v: Some((73, 2)),
                                c: None,
                            },
                        ]),
                    },
                    St {
                        v: Some((72, 3)),
                        c: Some(vec![
                            St {
                                v: Some((73, 1)),
                                c: Some(vec![St {
                                    v: Some((777, 3)),
                                    c: None,
                                }]),
                            },
                            St {
                                v: Some((73, 2)),
                                c: None,
                            },
                        ]),
                    },
                ]),
            },
            St {
                v: Some((7, 3)),
                c: Some(vec![St {
                    v: Some((7, 3)),
                    c: None,
                }]),
            },
        ]),
    });

    #[test]
    fn it_walk_branch() {
        //common::setup();
        // walk the tree
        let s: St = TEST_S.clone();
        let (new_tiles, new_path) = match s.left_hand_path() {
            Ok(LhpResult(b, p)) => {
                err(&"LHP found a path");
                (b, p)
            }
            Err(e) => match e {
                "Zero size SearchTree" if s.c == None => {
                    warn(format!("Zero size SearchTree: {:?}", s));
                    (vec![], vec![])
                } // to be expected: if the St is blank then the LHP is blank
                "Zero size SearchTree" => {
                    err(format!("left_hand_path failed HARD: {:?}", s));
                    (vec![], vec![])
                } // to be expected: if the St is blank then the LHP is blank
                _ => {
                    // uh-oh! looks like an error
                    err(format!("LHP FAILED!!!: {:?}", &e));
                    (vec![], vec![])
                }
            },
        };

        //let path_test = [1,0]; let board_test = [(1, 1), (2, 11)];// Bad graft! (but GOOD left_walk!) [force err]
        let path_test = [0, 0];
        let board_test = [(1, 1), (2, 1)]; // Bad graft! (but GOOD left_walk!)
        s.dump();
        assert_eq!(board_test.to_vec(), new_tiles);
        assert_eq!(path_test.to_vec(), new_path);
    }
}
