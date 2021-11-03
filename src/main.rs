use std::sync::mpsc::channel;
use std::{thread};
pub mod performance;
pub mod util;
use performance::analyze::*;
use performance::mesure::*;
fn main() {
    let (sender, receiver) = channel();
    let anna = Analyzer::new(receiver);
    for _ in 0..5 {
        let s = sender.clone();
        let builder = thread::Builder::new().name("slow thread".into());
        builder
            .spawn(move || {
                mesure! {s,{
                        let mut b = 0;
                        for _ in 0..12 {
                            b *= 4;
                            b /= 4;
                        }
                        b == 10
                }}
            })
            .unwrap();
    }
    mesure! {sender,{
    let mut a: u128 = 64;
    for _ in 0..20{
        mesure!(sender, {
            a += 1
        });
        mesure!(sender, {
            a *= 4;
            a /=4;
        });

    }
    println!("{}",a);}}
    drop(sender);
    //println!("{}", anna.join().unwrap().print());
    anna.join().unwrap().plot();
}
