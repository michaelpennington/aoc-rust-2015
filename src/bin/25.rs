advent_of_code::solution!(25);

fn f(n: u64) -> u64 {
    let mut ans = 20151125;
    for _ in 1..n {
        ans = (ans * 252533) % 33554393;
    }
    ans
}

fn g(x: u64, y: u64) -> u64 {
    let n = ((x + y - 1) * (x + y)) / 2 - y + 1;
    // println!("({x}, {y}) -> {n}");
    f(n)
}

pub fn part_one(_input: &str) -> Option<u64> {
    // println!("g(4, 3) = {}", g(4, 3));
    Some(g(3083, 2978))
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(0)
}
