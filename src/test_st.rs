use super::*;
#[cfg(test)]
mod st_tests {
    // import crate::st
    use super::*;
    use crate::st::mut_ref_to_child;
    use once_cell::sync::Lazy;

    // maintain my nice tidy test structs
    #[rustfmt::skip]
      //static TEST_S: St = St { v: None, c: Some(vec![
      static TEST_S: Lazy<St> = Lazy::new(|| St { v: None, c: Some(vec![
              St { v: Some((1, 1)), c: Some(vec![St { v: Some((2, 1)), c: None, }]), },
              St { v: Some((1, 2)), c: Some(vec![
                      St { v: Some((2, 1)), c: Some(vec![
                              St { v: Some((3, 2)), c: None, },
                              St { v: Some((3, 2)), c: None, },
                          ]),
                      },
                      St { v: Some((2, 3)), c: Some(vec![
                              St { v: Some((3, 0)), c: None, },
                              St { v: Some((3, 2)), c: None, },
                          ]),
                      },
                  ]),
              },
              St { v: Some((1, 3)), c: Some(vec![St { v: Some((2, 3)), c: None, }]), },
          ]),
      });
    //};

    #[rustfmt::skip]
      static TEST_T: Lazy<St> = Lazy::new(|| St { v: Some((199, 2)), c: Some(vec![
              St { v: Some((71, 1)), c: Some(vec![St { v: Some((7, 1)), c: None, }]), },
              St { v: Some((71, 2)), c: Some(vec![
                      St { v: Some((72, 2)), c: Some(vec![
                              St { v: Some((73, 2)), c: None, },
                              St { v: Some((73, 2)), c: None, },
                          ]),
                      },
                      St { v: Some((72, 3)), c: Some(vec![
                              St { v: Some((73, 1)), c: Some(vec![St { v: Some((777, 3)), c: None, }]), },
                              St { v: Some((73, 2)), c: None, },
                          ]),
                      },
                  ]),
              },
              St { v: Some((7, 3)), c: Some(vec![St { v: Some((7, 3)), c: None, }]), },
          ]),
      });

    #[rustfmt::skip]
      static TEST_U: Lazy<St> = Lazy::new(|| St { v: None, c: Some(vec![
              St { v: Some((1, 1)), c: Some(vec![St { v: Some((2, 1)), c: None, }]), },
              St { v: Some((1, 2)), c: Some(vec![
                      St { v: Some((2, 1)), c: Some(vec![
                              St { v: Some((3, 1)), c: None, },
                              St { v: Some((3, 2)), c: None, },
                          ]),
                      },
                      St { v: Some((2, 3)), c: Some(vec![
                              St { v: Some((3, 1)), c: None, },
                              St { v: Some((3, 2)), c: None, },
                          ]),
                      },
                  ]),
              },
              St { v: Some((1, 3)), c: Some(vec![St { v: Some((2, 3)), c: None, }]), },
          ]),
      });

    #[rustfmt::skip]
      static TEST_V: Lazy<St> = Lazy::new(|| St { v: None, c: Some(vec![
              St { v: Some((1, 1)), c: Some(vec![St { v: Some((2, 1)), c: None, }]), },
              St { v: Some((1, 2)), c: Some(vec![
                      St { v: Some((2, 1)), c: Some(vec![
                              St { v: Some((3, 1)), c: None, },
                              St { v: Some((3, 2)), c: None, },
                          ]),
                      },
                      St { v: Some((2, 3)), c: Some(vec![
                              St { v: Some((3, 1)), c: Some(vec![St { v: Some((999, 9)), c: None, }]), },
                              St { v: Some((3, 2)), c: None, },
                          ]),
                      },
                  ]),
              },
              St { v: Some((1, 3)), c: Some(vec![St { v: Some((2, 3)), c: None, }]), },
          ]),
      });
    // the "P" class of static test St structs are for PRUNED branched
    // BECAUSE any node with a rotation that != [1,2,3,4] is skipped as either marked for pruning or as being occupied by another thread!
    //static TEST_SP: St = St { v: None, c: Some(vec![

    /*

    #[rustfmt::skip]
      static TEST_SP: Lazy<St> = Lazy::new(|| St { v: None, c: Some(vec![
              St { v: Some((1, 1)), c: Some(vec![St { v: Some((2, 11)), c: None, }]), },
              St { v: Some((1, 2)), c: Some(vec![
                      St { v: Some((2, 21)), c: Some(vec![
                              St { v: Some((3, 211)), c: None, },
                              St { v: Some((3, 212)), c: None, },
                          ]),
                      },
                      St { v: Some((2, 31)), c: Some(vec![
                              St { v: Some((3, 011)), c: None, },
                              St { v: Some((3, 012)), c: None, },
                          ]),
                      },
                  ]),
              },
              St { v: Some((1, 3)), c: Some(vec![St { v: Some((2, 3)), c: None, }]), },
          ]),
      });
    //};

    #[rustfmt::skip]
      static TEST_TP: Lazy<St> = Lazy::new(|| St { v: Some((199, 2)), c: Some(vec![
              St { v: Some((71, 71)), c: Some(vec![St { v: Some((7, 11)), c: None, }]), },
              St { v: Some((71, 72)), c: Some(vec![
                      St { v: Some((72, 21)), c: Some(vec![
                              St { v: Some((73, 211)), c: None, },
                              St { v: Some((73, 212)), c: None, },
                          ]),
                      },
                      St { v: Some((72, 31)), c: Some(vec![
                              St { v: Some((73, 011)), c: Some(vec![St { v: Some((777, 7)), c: None, }]), },
                              St { v: Some((73, 012)), c: None, },
                          ]),
                      },
                  ]),
              },
              St { v: Some((7, 3)), c: Some(vec![St { v: Some((7, 3)), c: None, }]), },
          ]),
      });

    #[rustfmt::skip]
      static TEST_UP: Lazy<St> = Lazy::new(|| St { v: None, c: Some(vec![
              St { v: Some((1, 1)), c: Some(vec![St { v: Some((2, 11)), c: None, }]), },
              St { v: Some((1, 2)), c: Some(vec![
                      St { v: Some((2, 21)), c: Some(vec![
                              St { v: Some((3, 211)), c: None, },
                              St { v: Some((3, 212)), c: None, },
                          ]),
                      },
                      St { v: Some((2, 31)), c: Some(vec![
                              St { v: Some((3, 011)), c: None, },
                              St { v: Some((3, 012)), c: None, },
                          ]),
                      },
                  ]),
              },
              St { v: Some((1, 3)), c: Some(vec![St { v: Some((2, 3)), c: None, }]), },
          ]),
      });

    #[rustfmt::skip]
      static TEST_VP: Lazy<St> = Lazy::new(|| St { v: None, c: Some(vec![
              St { v: Some((1, 1)), c: Some(vec![St { v: Some((2, 11)), c: None, }]), },
              St { v: Some((1, 2)), c: Some(vec![
                      St { v: Some((2, 21)), c: Some(vec![
                              St { v: Some((3, 211)), c: None, },
                              St { v: Some((3, 212)), c: None, },
                          ]),
                      },
                      St { v: Some((2, 31)), c: Some(vec![
                              St { v: Some((3, 011)), c: Some(vec![St { v: Some((999, 9)), c: None, }]), },
                              St { v: Some((3, 012)), c: None, },
                          ]),
                      },
                  ]),
              },
              St { v: Some((1, 3)), c: Some(vec![St { v: Some((2, 3)), c: None, }]), },
          ]),
      });

      */

    /*
        START of the actual unit tests
    */

    #[test]
    pub fn create_ne() {
        let s: St = St::new();
        assert_eq!(s, St { v: None, c: None });
    }

    #[test]
    fn new_st_from_tiles() {
        let next_tiles = [(0, 1), (2, 4), (6, 2), (8, 3)].to_vec();
        let s = st_from_tiles(next_tiles);
        //s.print_as_yaml();
        #[rustfmt::skip]
        assert_eq!( s,
            St { v: None, c: Some(vec![
                    St { v: Some((0, 1)), c: None },
                    St { v: Some((2, 4)), c: None },
                    St { v: Some((6, 2)), c: None },
                    St { v: Some((8, 3)), c: None },
                ])
            }
        );
    }

    #[rustfmt::skip]
    #[test]
    fn graft_branch_to_root() {
        let mut s: St = St::new();
        let _ = graft( &mut s, [0].to_vec(), St { v: Some((999, 9)), c: None, },);
        assert_eq!(s, St { v: None, c: Some(vec![St { v: Some((999, 9)), c: None }]) }
        );
    }

    #[rustfmt::skip]
    #[test] // here we have found the first 4 corners and have added them to the tree.. and now we test adding a new edge
    fn graft_branch_to_corners() {
        let next_tiles = [(0, 1), (2, 4), (6, 2), (8, 3)].to_vec();
        let mut s = st_from_tiles(next_tiles);
        let _ = graft( &mut s, [0].to_vec(), St { v: Some((999, 9)), c: None, },);
        s.dump();
        assert_eq!( s,
            St { v: None, c: Some(vec![
                    St { v: Some((0, 1)), c: Some(vec![St { v: Some((999, 9)), c: None }]) },
                    St { v: Some((2, 4)), c: None },
                    St { v: Some((6, 2)), c: None },
                    St { v: Some((8, 3)), c: None }
                ])
            }
        );
    }

    #[rustfmt::skip]
    #[test] // here we some of the tiles for the second square already on the tree, now we want to add some new 3rd level tiles!
    fn graft_branch_to_0_() {
        let mut s: St = St { v: None, c: Some(vec![
                St { v: Some((7, 3)), c: Some(vec![St { v: Some((3, 1)), c: None, }]), },
                St { v: Some((7, 1)), c: Some(vec![St { v: Some((3, 3)), c: None, }]), },
                St { v: Some((7, 2)), c: Some(vec![St { v: Some((3, 1)), c: None, }]), },
            ]),
        };
        let t: St = St { v: None, c: Some(vec![
                St { v: Some((7, 3)), c: Some(vec![
                        St { v: Some((3, 1)), c: None, },
                        St { v: Some((999, 9)), c: None, },
                    ]),
                },
                St { v: Some((7, 1)), c: Some(vec![St { v: Some((3, 3)), c: None, }]), },
                St { v: Some((7, 2)), c: Some(vec![St { v: Some((3, 1)), c: None, }]), },
            ]),
        };
        /* let next_tiles= [(0, 1), (2, 4), (6, 2), (8, 3)].to_vec();
        let mut s = st_from_tiles(next_tiles);
        let c1: St = St { v: Some((2,1)), c: Some(vec![St{ v: Some((8,3)), c: None}]) };
        s.add_child(c1);
        */
        let _ = graft(
            &mut s,
            [0].to_vec(),
            St {
                v: Some((999, 9)),
                c: None,
            },
        );
        eprintln!("----------------");
        s.dump();
        println!("---^s--_0_--------");
        t.dump();
        eprintln!("---^t-------------");

        assert_eq!(s, t);
    }

    #[rustfmt::skip]
    #[test] // here we some of the tiles for the second square already on the tree, now we want to add some new 3rd level tiles!
    fn graft_branch_to_0_0_() {
        let mut s: St = St { v: None, c: Some(vec![
                St { v: Some((7, 3)), c: Some(vec![St { v: Some((3, 1)), c: None, }]), },
                St { v: Some((7, 1)), c: Some(vec![St { v: Some((3, 3)), c: None, }]), },
                St { v: Some((7, 2)), c: Some(vec![St { v: Some((3, 1)), c: None, }]), },
            ]),
        };
        let t: St = St { v: None, c: Some(vec![
                St { v: Some((7, 3)), c: Some(vec![St { v: Some((3, 1)), c: Some(vec![St { v: Some((999, 9)), c: None, }]), }]), },
                St { v: Some((7, 1)), c: Some(vec![St { v: Some((3, 3)), c: None, }]), },
                St { v: Some((7, 2)), c: Some(vec![St { v: Some((3, 1)), c: None, }]), },
            ]),
        };
        /* let next_tiles= [(0, 1), (2, 4), (6, 2), (8, 3)].to_vec();
        let mut s = st_from_tiles(next_tiles);
        let c1: St = St { v: Some((2,1)), c: Some(vec![St{ v: Some((8,3)), c: None}]) };
        s.add_child(c1);
        */
        let _ = graft( &mut s, [0, 0].to_vec(), St { v: Some((999, 9)), c: None, },);
        eprintln!("----------------");
        s.dump();
        println!("---^s--_0_0_--------");
        t.dump();
        eprintln!("---^t-------------");

        assert_eq!(s, t);
    }

    #[test] // here we have some of the tiles for the second square already on the tree, now we want to add some new 3rd level tiles!
    fn graft_branch_to_1_0_() {
        let mut s = TEST_U.clone();
        let g = TEST_U.clone();
        let t = TEST_U.clone();

        /*
        let next_tiles= [(0, 1), (2, 4), (6, 2), (8, 3)].to_vec();
        let mut s = st_from_tiles(next_tiles);
        let c1: St = St { v: Some((2,1)), c: Some(vec![St{ v: Some((8,3)), c: None}]) };
        s.add_child(c1);
        */

        // this is a DELIBERATE typo (that we are testing) <should have been [1,0]>
        //let _ = graft(&mut s, [1,1].to_vec(), St { v: Some((999,9)), c: None });
        let _ = graft(&mut s, [1, 1].to_vec(), g);

        s.dump();
        eprintln!("--------^s---Vt----------");
        t.dump();

        //assert_eq!(s,t);
        assert_ne!(s, t);
    }

    #[test] // here, some of the tiles for the second square already on the tree, now we want to add some new 3rd level tiles!
    fn graft_branch_to_1_1_0() {
        let mut s = TEST_U.clone();
        //let t: St = St{ ..St::default() };
        let t = TEST_V.clone();

        #[rustfmt::skip]
        let _ = graft( &mut s, [1, 1, 0].to_vec(), St { v: Some((999, 9)), c: None, },);
        /* DEBUG
        eprintln!("----------------");
        s.dump();
        eprintln!("---^s-------------");
        t.dump();
        eprintln!("---^t-------------");
        */

        assert_eq!(s, t);
    }

    #[test]
    fn walk_branch() {
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

    #[test]
    fn graft_and_walk() {
        // here we want to take a large tree, graft on another large tree and then walk the left-path
        let mut s: St = TEST_S.clone();
        let t: St = TEST_T.clone();
        /*
        let mut s: St = St::new();
        s.generate_test_st();
        */

        //let t: St = St{ ..St::default() };

        // NTS if we graft an St with a root {v: None} then it stalls left_walk
        // we need to deal with this (either reject the graft as invalid, or replace it with the Node-to-which-we-are-grafting)

        // N.B. do not confure the graft_site array with the new_path array!

        //let graft_site = [0,0]; // works
        //let graft_site = [2,0]; //works nb:= [(1, 1), (2, 11)]; np: = [0,0]
        //let graft_site = [1,1]; //works
        //let graft_site = [0,1]; // "incorrectly" grafts to [0,0] because there is NO [0,1]
        //let graft_site = []; // [] === [0]
        //let graft_site = [2].to_vec();

        //let graft_site = [0,0,0]; let path_test = [0,0]; let board_test = [(1, 1), (2, 11)]; //Ok
        //let graft_site = [2,0]; let path_test = [0,0]; let board_test = [(1, 1), (2, 11)]; //Ok
        //let graft_site = [1,1]; let path_test = [0,0]; let board_test = [(1, 1), (2, 11)]; //Ok
        //let graft_site = [0,0]; let path_test = [0,0,0,0,0]; let board_test = [(1, 1), (2, 11), (199, 2), (71, 71), (7, 11)];// Ok
        //let graft_site = [0]; let path_test = [0,0]; let board_test = [(1_u16, 1_u8), (2_u16, 11_u8)];// Ok
        let graft_site = [0; 0];
        let path_test = [0, 0];
        let board_test = [(1, 1), (2, 1)]; // Bad graft! (but GOOD left_walk!)

        //let graft_site = []; let path_test = [0,0]; let board_test = [(1, 2), (2, 11)];// Bad graft! (but GOOD left_walk!)
        //let graft_site = []; let path_test = [1,0]; let board_test = [(1, 1), (2, 11)];// Bad graft! (but GOOD left_walk!)
        //let graft_site = [-1]; let path_test = [0,0]; let board_test = [(1, 1), (2, 11)];// Err("the trait `std::ops::Neg` is not implemented for `u16`")

        let graft_site_recipt = graft_site.clone();
        let _ = graft(&mut s, graft_site.to_vec(), t);
        //let _ = graft(&mut s, [0,0,0].to_vec(), t);
        let LhpResult(new_tiles, new_path) =
            s.left_hand_path().unwrap_or(LhpResult(vec![], vec![]));
        s.dump();
        // NTS currently this shows that left-hand-path is only going 2 deep!!!! YOU ARE HERE (guess we need to look at fn graft()! )
        eprintln!("Graft site: {:?}", graft_site_recipt);
        eprintln!("New Board (after graft: {:?}", new_tiles);
        assert_eq!(path_test.to_vec(), new_path);
        //assert_eq!(vec![1_usize], new_path);
        //assert_eq!(graft_site_recipt, new_path);
        //assert_eq!(vec![(0_u16,0_u8)], new_tiles);
        assert_eq!(board_test.to_vec(), new_tiles);
    }

    #[test] // YOU NEED TO FIX left_hand_path!!!
    fn walk_past_pruned_nodes() {
        #[rustfmt::skip]
        let s: St = St { v: None, c: Some(vec![
                St { v: Some((0, 1)), c: Some(vec![St { v: Some((1, 1)), c: Some(vec![
                            St { v: Some((2, 1)), c: Some(vec![
                                    St { v: Some((5, 1)), c: Some(vec![
                                            St { v: Some((6, 4)), c: Some(vec![
                                                    St { v: Some((3, 0)), c: None, },
                                                    St { v: Some((74, 1)), c: None, },
                                                ]),
                                            },
                                            St { v: Some((8, 1)), c: None, },
                                        ]),
                                    },
                                    St { v: Some((7, 4)), c: None, },
                                ]),
                            },
                            St { v: Some((6, 3)), c: Some(vec![
                                    St { v: Some((3, 0)), c: None, },
                                    St { v: Some((73, 1)), c: None, },
                                ]),
                            },
                            St { v: Some((8, 4)), c: None, },
                        ]),
                    }]),
                },
                St { v: Some((2, 4)), c: None, },
                St { v: Some((6, 2)), c: None, },
                St { v: Some((8, 3)), c: None, },
            ]),
        };

        //let LhpResult(new_tiles, new_path) = s.left_hand_path().unwrap_or((vec![], vec![]));
        let (new_tiles, _stn_path) = match s.left_hand_path() {
            Ok(LhpResult(b, p)) => {
                info(&"LHP found a path");
                (b, p)
            }
            Err(e) => match e {
                "Zero size SearchTree" => (vec![], vec![]), // to be expected: if the St is blank then the LHP is blank
                _ => {
                    // uh-oh! looks like an error
                    err(format!("LHP FAILED!!!: {:?}", &e));
                    (vec![], vec![])
                }
            },
        };
        assert_eq!(
            new_tiles,
            vec![(0, 1), (1, 1), (2, 1), (5, 1), (6, 4), (74, 1)]
        );
    }

    #[test]
    fn walk_then_graft() {
        // walk the tree, and then use the path as a graft index
        let mut s: St = TEST_S.clone();
        let t: St = TEST_T.clone();
        //let t: St = St{ ..St::default() };
        //let LhpResult(new_tiles, new_path) = s.left_hand_path().unwrap_or((vec![], vec![]));
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
        let new_path_recipt = new_path.clone();
        let mut graft_site: Vec<u16> = vec![];
        for p in new_path {
            graft_site.push(p as u16);
        }
        let graft_site_recipt = graft_site.clone();
        let _ = graft(&mut s, graft_site, t);
        s.dump();
        eprintln!("Graft site: {:?}", graft_site_recipt);
        eprintln!("New path: {:?}", new_path_recipt);
        eprintln!("New Board (after graft): {:?}", new_tiles);
        assert_eq!(path_test.to_vec(), graft_site_recipt);
        assert_eq!(board_test.to_vec(), new_tiles);
    }

    #[test]
    fn multiple_mutable_branches() {
        // create our struct
        #[rustfmt::skip]
        let mut s: St = St { v: None, c: Some(vec![
                St { v: Some((7, 3)), c: Some(vec![St { v: Some((4, 1)), c: None, }]), },
                St { v: Some((5, 1)), c: Some(vec![St { v: Some((8, 3)), c: None, }]), },
                St { v: Some((3, 2)), c: Some(vec![St { v: Some((9, 1)), c: None, }]), },
            ]),
        };
        //let mut t = TEST_U.clone();
        let mut t = s.clone(); // take a copy of the St for testing
                               // >>>>> we want to be able to pass branches to different threads so <<<<<
                               // get an example handle to one child-of-a-child
        let path_a = vec![1, 1];
        let twin_path_a = vec![0, 1];
        let twin_a = mut_ref_to_child(&mut t, &twin_path_a);
        #[rustfmt::skip]
        let a_new_branch: St = St { v: Some((2, 1)), c: None, };
        twin_a.add_st(a_new_branch);

        /* FIRST mutable_ref_to_child */

        let child_a = mut_ref_to_child(&mut s, &path_a);

        // mutate child_a
        let z: St = St {
            v: Some((2, 1)),
            c: None,
        };
        child_a.add_st(z);

        assert_eq!(*child_a, *twin_a);
        //assert_eq!(s, t);

        /* SECOND mutable_ref_to_child */

        //assert_eq!(*child_a, t);
        //assert_eq!(*child_a, twin_a);
        /*
        assert_eq!(*child_a,
            St { v: None, c: Some(vec![St { v: Some((7, 3)), c: Some(vec![St { v: Some((4, 1)), c: None }]) }, St { v: Some((5, 1)), c: Some(vec![St { v: Some((8, 3)), c: None }]) }, St { v: Some((3, 2)), c: Some(vec![St { v: Some((9, 1)), c: None }]) }, St { v: Some((2, 1)), c: None }]) }
        );
        */

        // get a handle to a different child-of-a-child on another branch
        //let path_b = vec![2,1];
        let path_b = vec![1];
        // #[trust_me_borrow_checker(this is a different part of the tree) // TODO: VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV
        let child_b = mut_ref_to_child(&mut s, &path_b); // THIS IS NOT WORKING! (just gets a ref to the s; so child_b.add_st(y) cats "y" onto the end)
                                                         //   TODO ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

        child_b.dump();

        #[rustfmt::skip]
        // mutate child_b
        let y: St = St { v: Some((1, 1)), c: None, };
        //child_b.add_st(y.clone());
        child_b.add_child([y.clone()].to_vec());
        t.add_st(y);
        #[rustfmt::skip]
        let twin_b: St = St { v: None, c: Some(vec![
                St { v: Some((7, 3)), c: Some(vec![St { v: Some((4, 1)), c: None, }]), },
                St { v: Some((5, 1)), c: Some(vec![St { v: Some((8, 3)), c: None, }]), },
                St { v: Some((3, 2)), c: Some(vec![St { v: Some((9, 1)), c: None, }]), },
                St { v: Some((2, 1)), c: None, },
                St { v: Some((1, 1)), c: None, },
            ]),
        };

        assert_eq!(*child_b, twin_b);

        /* check that we end up at with the same trees with both methods */

        /*
        s.dump();
        t.dump();
        */
        //assert_eq!(child_a, child_b);
        assert_eq!(s, t);
        //assert_ne!(s, t);
    }
    #[test]
    fn walk_path_and_spawn_children_then_graft_on_the_return_from_the_children() {
        // so rather than giving the children pointers to location on the tree
        // we cerate NEW trees for them to populate (and return)
        // and then we grapht on what they give us!!!
        //
        // this means that we can populate the new child trees with the path that we are giving them
        // and so do not have to give them full trees.
    }
    /*
        gardener(tree_path, board_path, board, trih, control) -> (Vec<tree_path>, St) {
            if child_s.last_vec().collect() == Vec(Some((_,0))) // then all of the children have been pruned {
                parent.v = Some(Sum_of_(this,_) from each child if Sum_of(this) < usize }else{ (usize,0) }
                // that is how we recess/recoil from a dead branch
            elsif child_s.last_vec().collect().has_any_child( c: None ) {
                then crate::solve::puzzle(this node's children)
            }
            return (tree_path, child_ST)
        }
      let finished: St = loop {
        let (candiate_tree_path, board) = iter St and output (Vec(tree_path{which is unique!}), board)
        if candidate_tree_path.len() == 0 {
            save_ST_to_disk()
            break St //return St to $finished
        }

        (child_id, child_ST) =gardener(tree_path, board_path, board, trih, Enum{ control bit where {depth(usize), limits depth,}})
        graft_child_ST(tree_path, child_ST)
        save_ST_to_disk()

      };

    */
}
