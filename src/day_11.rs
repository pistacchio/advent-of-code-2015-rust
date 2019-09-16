use itertools::Itertools;

fn rotate_password(password: &str) -> String {
    let mut new_password = String::new();
    let mut rotating = true;

    for c in password.chars().rev() {
        let mut new_char = c;

        if rotating {
            new_char = if c == 'z' { 'a' } else { ((c as u8) + 1) as char };

            if new_char != 'a' {
                rotating = false;
            }
        }

        new_password = format!("{}{}", new_char, new_password);
    }

    new_password
}

fn is_password_ok(password: &str) -> bool {
    if password.contains('i') || password.contains('o') || password.contains('l') {
        return false;
    }

    if password.chars()
        .group_by(|c| *c)
        .into_iter()
        .map(|(key, vals)| (key, vals.count()))
        .filter(|(_key, vals_count)| *vals_count > 1)
        .unique()
        .count() < 2 {
        return false;
    }

    let consecutive_triplets = password.chars().zip(
        password.chars().skip(1).zip(
            password.chars().skip(2)
        )
    ).filter(|c| ((c.0 as u8) + 1 == (c.1).0 as u8) && ((((c.1).0 as u8) + 1) == (c.1).1 as u8))
        .collect::<Vec<_>>();

    if consecutive_triplets.is_empty() {
        return false
    }

    true
}

pub fn run() -> String {
    let mut new_password = rotate_password("cqjxjnds");

    while !is_password_ok(&new_password) {
        new_password = rotate_password(&new_password);
    }

    new_password
}

pub fn run_pt2() -> String {
    let mut new_password = rotate_password(&run());

    while !is_password_ok(&new_password) {
        new_password = rotate_password(&new_password);
    }

    new_password
}

#[test]
fn test_run() {
    assert!(!is_password_ok("hijklmmn"));
    assert!(!is_password_ok("abbceffg"));
    assert!(!is_password_ok("abbcegjk"));
}

