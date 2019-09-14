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
    let miner = Miner::new(key);

    miner.enumerate()
        .find(|(_, m)| {
            let hash = format!("{:x}", md5::compute(m));

            hash.starts_with(signal)
        })
        .unwrap().0 + 1
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
