extern crate getopts;

pub mod util;

use getopts::Options;
use std::{env, time};
use util::board::Board;
use util::ui::UI;
use std::thread;
use std::thread::JoinHandle;
use thread_priority::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

const N: usize = 13;

fn solve(mut board: Board, column: usize, mut count: &mut i64) {
    if column == 0 {
        let mut threads: Vec<JoinHandle<i64>> = vec![];
        for y in 0..N {
            // if y % 2 != 0 { continue; }
            //board.set(0, y, true);

            // let mut count1: i64 = 0;
            // let mut count2: i64 = 0;

            // rayon::join(|| solve(board, 1, &mut count1),
            //             || solve(board, 1, &mut count2));

            threads.push(thread::spawn(move || {
                board.set(0, y, true);
                //set_current_thread_priority(ThreadPriority::Max);
                let mut count: i64 = 0;
                solve(board, 1, &mut count);
                board.set(0, y, false);
                count
            }));
        }
        for thread in threads {
           *count += thread.join().unwrap();
            //break;
        }
    } else {
        if column == N {
            *count += 1;
            return;
        }
        for y in 0..N {
            let mut ok: bool = true;
            for x in 0..column {
                if board.get(x, y).is_occupied()
                    || y + x >= column && board.get(x, y + x - column).is_occupied()
                    || y + column < N + x && board.get(x, y + column - x).is_occupied()
                {
                    ok = false;
                    break;
                }
                if !ok {
                    break;
                }
            }
            if ok {
                board.set(column, y, true);
                solve(board, column + 1, &mut count);
                board.set(column, y, false);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("g", "graphical", "use graphical output");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let mut ui = match matches.opt_present("g") {
        true => UI::new(),
        false => UI::disabled(),
    };
    set_current_thread_priority(ThreadPriority::Specific(75));
    let board = Board::new();
    let now = time::Instant::now();
    let mut count: i64 = 0;
    solve(board, 0, &mut count);
    println!("{}\n{}", now.elapsed().as_nanos(), count);
}