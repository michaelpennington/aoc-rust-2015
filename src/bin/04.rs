use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    thread,
};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut bs: [u8; 64] = [0; 64];
    for (ib, b) in input.trim().bytes().zip(bs.iter_mut()) {
        *b = ib;
    }

    let done = Arc::new(AtomicU32::new(u32::MAX));
    let input_len = input.len();
    let jhs: Vec<_> = (1..=16)
        .map(|j| {
            let d = Arc::clone(&done);
            thread::spawn(move || {
                let mut bs: [u8; 64] = bs;
                for i in (0..).map(|i| i * 16 + j) {
                    if d.load(Ordering::Relaxed) < i {
                        return;
                    }
                    let i_string = i.to_string();
                    let len = input_len + i_string.len();
                    for (is, b) in i_string.bytes().zip(bs[input_len - 1..].iter_mut()) {
                        *b = is;
                    }
                    let res = md5::compute(&bs[..len - 1]);
                    if res.0[0] == 0 && res.0[1] == 0 && res.0[2] < 16 {
                        d.store(i, Ordering::Relaxed);
                        return;
                    }
                }
            })
        })
        .collect();

    for jh in jhs {
        jh.join().unwrap();
    }

    Some(done.load(Ordering::Relaxed))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bs: [u8; 64] = [0; 64];
    for (ib, b) in input.trim().bytes().zip(bs.iter_mut()) {
        *b = ib;
    }

    let done = Arc::new(AtomicU32::new(u32::MAX));
    let input_len = input.len();
    let jhs: Vec<_> = (1..=16)
        .map(|j| {
            let d = Arc::clone(&done);
            thread::spawn(move || {
                let mut bs: [u8; 64] = bs;
                for i in (0..).map(|i| i * 16 + j) {
                    if d.load(Ordering::Relaxed) < i {
                        return;
                    }
                    let i_string = i.to_string();
                    let len = input_len + i_string.len();
                    for (is, b) in i_string.bytes().zip(bs[input_len - 1..].iter_mut()) {
                        *b = is;
                    }
                    let res = md5::compute(&bs[..len - 1]);
                    if res.0[0] == 0 && res.0[1] == 0 && res.0[2] == 0 {
                        d.store(i, Ordering::Relaxed);
                        return;
                    }
                }
            })
        })
        .collect();

    for jh in jhs {
        jh.join().unwrap();
    }

    Some(done.load(Ordering::Relaxed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(609043));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1048970));
    }
}
