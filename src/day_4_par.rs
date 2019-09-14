use std::sync::atomic::{AtomicUsize, Ordering, AtomicBool};
use std::thread::spawn;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc};

const OK_SIGNAL: &str = "00000";
const OK_SIGNAL_PT2: &str = "000000";

fn mine_adventcoins(key: &str, signal: &str) -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let found = Arc::new(AtomicBool::new(false));

    let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();

    for _ in 0..8 {
        let thread_found = found.clone();
        let thread_counter = counter.clone();
        let thread_tx = tx.clone();
        let thread_key = key.to_string().clone();
        let thread_signal = signal.to_string().clone();

        spawn(move || {
            loop {
                if thread_found.load(Ordering::Relaxed) {
                    break;
                }

                let n = thread_counter.fetch_add(1, Ordering::SeqCst);

                let hash = format!("{:x}", md5::compute(format!("{}{}", thread_key, n)));

                if hash.starts_with(&thread_signal) {
                    thread_tx.send(n).unwrap();
                    thread_found.store(true, Ordering::Relaxed);
                    break;
                }
            }
        });
    }

    rx.recv().unwrap()
}

pub fn run() -> String {
    mine_adventcoins("iwrupvqb", OK_SIGNAL).to_string()
}

pub fn run_pt2() -> String {
    mine_adventcoins("iwrupvqb", OK_SIGNAL_PT2).to_string()
}

#[test]
fn test_run() {
    assert_eq!(mine_adventcoins("abcdef", OK_SIGNAL), 609_043);
    assert_eq!(mine_adventcoins("pqrstuv", OK_SIGNAL), 1_048_970);
}
