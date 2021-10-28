use rand::Rng;
use std::sync::mpsc::channel;
use std::{thread, time};
fn main() {
    let (sender, receiver) = channel();
    let mut handles = Vec::new();
    for n in 0..3 {
        let s = sender.clone();
        let builder_sender = thread::Builder::new().name(format!("sender thread {}", n).into());
        handles.push(
            builder_sender
                .spawn(move || {
                    let mut rng = rand::thread_rng();
                    for i in 0..10 {
                        let time_ms = time::Duration::from_millis(rng.gen_range(100, 550));
                        thread::sleep(time_ms);
                        s.send(format!("thread: {} round: {}", n, i)).unwrap();
                    }
                })
                .unwrap(),
        )
    }
    let builder_receiver = thread::Builder::new().name("receiver thread".into());
    let handle_receiver = builder_receiver
        .spawn(move || {
            for recv in receiver {
                let time_ms = time::Duration::from_millis(10);
                thread::sleep(time_ms);
                println!("received: {}", recv);
            }
        })
        .unwrap();

    for h in handles {
        h.join().unwrap();
    }
    handle_receiver.join().unwrap();
}
