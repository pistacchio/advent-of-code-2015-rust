use itertools::Itertools;

fn look_and_say(n: &str) -> String {
    let num: String = n.to_string()
        .chars()
        .group_by(|c| *c)
        .into_iter()
        .map(|(key, val)| format!("{}{}", val.count(), key))
        .collect();

    num
}

pub fn run() -> String {
    let mut res = "3113322113".to_string();

    for _ in 0..40 {
        res = look_and_say(&res);
    }

    res.to_string().chars().count().to_string()
}

pub fn run_pt2() -> String {
    let mut res = "3113322113".to_string();

    for n in 0..50 {
        println!("{}", n);
        res = look_and_say(&res);
    }

    res.to_string().chars().count().to_string()
}

#[test]
fn test_run() {
    assert_eq!(look_and_say("1"), "11");
    assert_eq!(look_and_say("11"), "21");
    assert_eq!(look_and_say("21"), "1211");
    assert_eq!(look_and_say("111221"), "312211");
}
