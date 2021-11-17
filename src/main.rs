extern crate getopts;

pub mod util;

use getopts::Options;
use std::{env, time};
use util::board::Board;
use util::ui::UI;
use std::thread;
use std::thread::JoinHandle;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

const N: usize = 13;

fn solve(mut board: Board, column: usize, mut count: &mut i64, mut ui: &mut UI) {
    let mut is_main_thread = false;
    if column == 0 {
       is_main_thread = true;
    }

    if column == N {
        *count += 1;
        ui.plot(board);
        return;
    }

    //let (tx, rx) = channel();
    //let mut count_threads = 0;
    //let allowed_threads = 13;

    let mut threads: Vec<JoinHandle<i64>> = vec![];

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
            ui.plot(board);

            if is_main_thread {
                //count_threads += 1;
                //let tx = tx.clone();
                threads.push(thread::spawn(move || {
                    let mut ui = UI::disabled();
                    let mut count: i64 = 0;
                    solve(board, column + 1, &mut count, &mut ui);
                    //tx.send(count).unwrap();
                    count
                }));
            } else {
                solve(board, column + 1, &mut count, &mut ui);
            }
            board.set(column, y, false);
        }
    }
    if is_main_thread {
        // drop(tx);
        // while let Ok(msg) = rx.recv() {
        //     *count += msg;
        // }
        //*count += threads.iter().map(|t| t.join().unwrap() as i64).sum::<i64>();

        for thread in threads {
            *count += thread.join().unwrap();
        }
    }

    // if is_main_thread {
    //     println!("{}", count_threads);
    // }
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
    let board = Board::new();
    let now = time::Instant::now();
    let mut count: i64 = 0;
    solve(board, 0, &mut count, &mut ui);
    println!("{}\n{}", now.elapsed().as_nanos(), count);
}
