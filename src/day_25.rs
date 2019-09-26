fn code_at(row: i128, col: i128) -> i128 {
    let mut diagonal = 2;
    let mut last = 20_151_125;

    loop {
        let mut r = diagonal;
        let mut c = 1;

        while c <= diagonal {
            last = (last * 252_533) % 33_554_393;

            if r == 2981 && c == 3075 {
                return last;
            }

            r -= 1;
            c += 1;
        }
        diagonal += 1;
    }
}

pub fn run() -> String {
    code_at(2981, 3075).to_string()
}
