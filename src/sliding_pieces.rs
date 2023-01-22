pub fn sout_attacks(mut pieces: u64, empty: u64) -> u64 {
    for _ in 0..7 {
        pieces |= (pieces >> 8) & empty;
    }
    pieces >> 8
}
pub fn nort_attacks(mut pieces: u64, empty: u64) -> u64 {
    for _ in 0..7 {
        pieces |= (pieces << 8) & empty;
    }
    pieces << 8
}
pub fn east_attacks(mut pieces: u64, mut empty: u64) -> u64 {
    const NOT_A: u64 = 0xfefefefefefefefe;
    empty &= NOT_A;
    for _ in 0..7 {
        pieces |= (pieces << 1) & empty;
    }
    (pieces << 1) & NOT_A
}
pub fn west_attacks(mut pieces: u64, mut empty: u64) -> u64 {
    const NOT_H: u64 = 0x7f7f7f7f7f7f7f7f;
    empty &= NOT_H;
    for _ in 0..7 {
        pieces |= (pieces >> 1) & empty;
    }
    (pieces >> 1) & NOT_H
}
pub fn noea_attacks(mut pieces: u64, mut empty: u64) -> u64 {
    const NOT_A: u64 = 0xfefefefefefefefe;
    empty &= NOT_A;
    for _ in 0..7 {
        pieces |= (pieces << 9) & empty;
    }
    (pieces << 9) & NOT_A
}
pub fn nowe_attacks(mut pieces: u64, mut empty: u64) -> u64 {
    const NOT_H: u64 = 0x7f7f7f7f7f7f7f7f;
    empty &= NOT_H;
    for _ in 0..7 {
        pieces |= (pieces << 7) & empty;
    }
    (pieces << 7) & NOT_H
}
pub fn soea_attacks(mut pieces: u64, mut empty: u64) -> u64 {
    const NOT_A: u64 = 0xfefefefefefefefe;
    empty &= NOT_A;
    for _ in 0..7 {
        pieces |= (pieces >> 7) & empty;
    }
    (pieces >> 7) & NOT_A
}
pub fn sowe_attacks(mut pieces: u64, mut empty: u64) -> u64 {
    const NOT_H: u64 = 0x7f7f7f7f7f7f7f7f;
    empty &= NOT_H;
    for _ in 0..7 {
        pieces |= (pieces >> 9) & empty;
    }
    (pieces >> 9) & NOT_H
}
