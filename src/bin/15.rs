advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let _ = input;
    (0..=100)
        .flat_map(move |a| {
            (0u32..=100 - a).flat_map(move |b| {
                (0..=100 - a - b).map(move |c| {
                    let s = 100 - a - b - c;
                    (2 * s)
                        * ((5 * b).saturating_sub(a))
                        * ((5 * c).saturating_sub(2 * s).saturating_sub(3 * b))
                        * ((5 * a).saturating_sub(c))
                })
            })
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let _ = input;
    (0..=100)
        .flat_map(move |a| {
            (0u32..=100 - a).flat_map(move |b| {
                (0..=100 - a - b).filter_map(move |c| {
                    let s = 100 - a - b - c;
                    if (s + b) * 3 + (a + c) * 8 == 500 {
                        Some(
                            (2 * s)
                                * ((5 * b).saturating_sub(a))
                                * ((5 * c).saturating_sub(2 * s).saturating_sub(3 * b))
                                * ((5 * a).saturating_sub(c)),
                        )
                    } else {
                        None
                    }
                })
            })
        })
        .max()
}
