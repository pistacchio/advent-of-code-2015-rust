use crypto::md5::Md5;
use crypto::digest::Digest;

const OK_SIGNAL: &str = "00000";
const OK_SIGNAL_PT2: &str = "000000";

struct Miner(usize, String);

impl Miner {
    fn new(key: &str) -> Self {
        Self(0, key.to_string())
    }
}

impl Iterator for Miner {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        Some(format!("{}{}", self.1, self.0))
    }
}

fn mine_adventcoins(key: &str, signal: &str) -> usize {
    let mut miner = Miner::new(key);
    let mut md5 = Md5::new();

    loop {
        let m = miner.next().unwrap();

        md5.reset();
        md5.input_str(&m);
        if md5.result_str().starts_with(signal) {
            return miner.0;
        }
    }
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
