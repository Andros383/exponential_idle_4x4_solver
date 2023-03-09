use std::{collections::HashSet, iter::Map};

use crate::state::State;
mod state;
const BOARD_SIZE: usize = 5;
fn _show(&board: &[[i32; BOARD_SIZE]; BOARD_SIZE]) {
    for row in board {
        for i in row {
            print!("{}\t", i);
        }
        println!();
    }
}
fn get_zero_board() -> [[i32; BOARD_SIZE]; BOARD_SIZE] {
    // match BOARD_SIZE {
    //     3 => [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
    //     4 => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    //     5 => [
    //         [0, 0, 0, 0, 0],
    //         [0, 0, 0, 0, 0],
    //         [0, 0, 0, 0, 0],
    //         [0, 0, 0, 0, 0],
    //         [0, 0, 0, 0, 0],
    //     ],
    //     _ => panic!("NA ilyen méretet nem tudok."),
    // } Nem is megy lol

    // return [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    // return [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    return [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
}
fn read_in() -> [[i32; BOARD_SIZE]; BOARD_SIZE] {
    let mut board: [[i32; BOARD_SIZE]; BOARD_SIZE] = get_zero_board();
    // std::io::stdin().read_line(&mut input).unwrap();
    let input = std::fs::read_to_string("teszt_input.txt").unwrap();
    let vec: Vec<i32> = input
        .split(" ")
        .map(|x| {
            x.trim()
                .parse::<i32>()
                .expect("Couldn't parse input to number.")
        })
        .collect();
    //Parse to array
    let mut counter = 0;
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            board[i][j] = *vec.get(counter).expect("Malformed input.");
            counter += 1;
        }
    }

    //Check validity
    for row in &board {
        for elem in row {
            //nem tudtam overflowolni
            if *elem < 0 || *elem as usize >= BOARD_SIZE * BOARD_SIZE {
                //Hibát kitalálni erre? Mert az usize nem lehet negatív, konvertáláskor elfogadhatja
                panic!(
                    "Unexpected integer in 0..{} range.",
                    BOARD_SIZE * BOARD_SIZE
                )
            }
        }
    }
    board
}

fn get_new_states(
    &board: &[[i32; BOARD_SIZE]; BOARD_SIZE],
) -> Vec<[[i32; BOARD_SIZE]; BOARD_SIZE]> {
    let (x, y) = get_position(&board, 0);
    let mut ki: Vec<[[i32; BOARD_SIZE]; BOARD_SIZE]> = Vec::new();
    let offsets = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for offset in offsets {
        let mut inner_board = board;
        //picit ronda
        let (ox, oy) = offset;
        let csere = match inner_board.get((x + ox) as usize) {
            Some(row) => match row.get((y + oy) as usize) {
                Some(x) => x,
                None => continue, //DEBUGOM VAGY MI
                                  //Ezek a continuek breakok voltak, így átugorja a maradék offseteket ha az első nem jó
            },
            None => continue,
        };
        //Innen már valid az ox és oy
        //Cserélem a 0-t és a részt amihez toltam
        inner_board[x as usize][y as usize] = *csere;
        inner_board[(x + ox) as usize][(y + oy) as usize] = 0;
        ki.push(inner_board);
    }
    ki
}
fn build_solved_board() -> [[i32; BOARD_SIZE]; BOARD_SIZE] {
    let mut solved_board: [[i32; BOARD_SIZE]; BOARD_SIZE] = get_zero_board();
    let mut counter = 1;
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            solved_board[i][j] = counter;
            counter += 1;
        }
    }
    solved_board[BOARD_SIZE - 1][BOARD_SIZE - 1] = 0;
    solved_board
}
/// Returns (x, y) where board\[x\]\[y\]=num
fn get_position(&board: &[[i32; BOARD_SIZE]; BOARD_SIZE], num: i32) -> (i32, i32) {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if board[i][j] == num {
                return (i as i32, j as i32);
            }
        }
    }
    panic!("Function get_position failed to find number in board.");
}
fn get_heuristic(&board: &[[i32; BOARD_SIZE]; BOARD_SIZE]) -> i32 {
    //Csak megmondja hogy minden egyes számot ha csak azt mozgatjuk mennyi idő benyomni a helyére
    let mut heuristic = 0;
    let solved_board = build_solved_board();
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let num = board[i][j];
            if num == 0 {
                continue;
            }
            let good_position = get_position(&solved_board, num);
            heuristic += (i as i32 - good_position.0).abs() + (j as i32 - good_position.1).abs()
        }
    }
    heuristic
}
fn print_id(&board: &[[i32; BOARD_SIZE]; BOARD_SIZE]) {
    //csak gyorsabb kiírásra
    for row in board {
        for j in row {
            print!("{}", j);
        }
    }
    println!();
}
//TODO gyakorlásnak kiszedni a segítő függvényeket külön fájlba
//Minta input: 3 4 2 5 0 6 8 7 1
//1 6 7 3 0 8 4 5 2
//Lehet elcsesztem az egészet?
//mert ez csak egy távolság, vagy mi a fene.
//De szerintem nem
//Mert itt nincsen "adott" távolság, amit lehetne számolni
//Talán az eddig megtett lépések száma?
//Nem tudom

//FIXME 5-re már lassú, a done_boards meg a visszamenési lista helyett lehetne egy hashmap
//Egy boardhoz azt rendelem hozzá, ahogy oda eljutottam (Lehet meg lehet oldani egy egész vektor nélkül?)
fn main() {
    // let n = 3;
    let board = read_in();
    let sv = build_solved_board();
    let start_state = State {
        board,
        heuristic: get_heuristic(&board),
        path: Vec::new(), //Ez lehet sok memória lesz
    };

    let mut progress = 0;
    let mut bakancslista = Vec::new();
    bakancslista.push(start_state);
    let mut done_boards: Vec<[[i32; BOARD_SIZE]; BOARD_SIZE]> = Vec::new();
    let mut current_state;
    loop {
        println!("{}", progress);
        progress += 1;
        // print!("{:?}\n\n\n{:?}", bakancslista, done_boards);
        current_state = match bakancslista
            .iter()
            .filter(|x| !done_boards.contains(&x.board)) //Ez lehet hogy lassú
            .min()
        {
            Some(s) => s,
            None => {
                // let mut bakancs_map: HashSet<[[i32; BOARD_SIZE]; BOARD_SIZE]> = HashSet::new();
                // let mut done_map: HashSet<[[i32; BOARD_SIZE]; BOARD_SIZE]> = HashSet::new();
                // for elem in &bakancslista {
                //     bakancs_map.insert(elem.board);
                // }
                // for elem in &done_boards {
                //     done_map.insert(*elem);
                // }

                // println!(
                //     "Bakancslista size: {}, Done_boards size: {}",
                //     bakancs_map.len(),
                //     done_map.len()
                // );

                // println!("Done boards");
                // for b in done_boards {
                //     println!("{:?}", b);
                // }
                // println!("Bakancslista");
                // for s in bakancslista {
                //     println!("{:?}", s);
                // }

                panic!("No solution exists!");
            }
        };

        done_boards.push(current_state.board);
        if current_state.board == sv {
            break;
        }

        // // print!("Current board: {:?}", &current_state.board);

        let mut to_push_vec = Vec::new();
        for new_state in get_new_states(&current_state.board) {
            let change = get_position(&current_state.board, 0);
            let changed_num = new_state[change.0 as usize][change.1 as usize];
            let mut new_vec = current_state.path.to_owned();

            // new_vec.push(changed_num);
            //PINPADOS OUTPUT
            //Csak kiiratom az új boardon a 0 helyét
            let nulla_helye = get_position(&new_state, 0);
            new_vec.push(sv[nulla_helye.0 as usize][nulla_helye.1 as usize]);
            to_push_vec.push(State {
                board: new_state,
                heuristic: get_heuristic(&new_state),
                path: new_vec,
            });
        }
        // print!("\n\n\n");

        // println!("Newly added boards:");
        // for b in &to_push_vec {
        //     println!("{:?}", b.board);
        // }
        // println!("Done boards");
        // for b in &done_boards {
        //     println!("{:?}", b);
        // }
        // // println!("Bakancslista");
        // // for s in &bakancslista {
        // //     println!("{:?}", s);
        // // }
        // print!("\n\n\n");
        bakancslista.append(&mut to_push_vec);
    }
    println!(
        "Done! Explored states: {} Moves used: {}\n Solution PINPAD MODE:\n",
        done_boards.len(),
        current_state.path.len()
    );
    let mut c = 0;
    for i in &current_state.path {
        print!("{}, ", i);
        c += 1;
        if c % 5 == 0 {
            println!();
        }
    }
}
