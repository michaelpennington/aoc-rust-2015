advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut max_distance = 0;
    const TIME: u32 = 2503;
    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<_>>();
        let speed: u32 = words[3].parse().unwrap();
        let time_a: u32 = words[6].parse().unwrap();
        let time_b: u32 = words[13].parse().unwrap();
        let period = time_a + time_b;
        max_distance =
            max_distance.max(((TIME / period) * time_a + time_a.min(TIME % period)) * speed);
    }
    Some(max_distance)
}

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    time_a: u32,
    time_b: u32,
    score: u32,
}

impl Reindeer {
    fn distance(&self, time: u32) -> u32 {
        let period = self.time_a + self.time_b;
        ((time / period) * self.time_a + self.time_a.min(time % period)) * self.speed
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    const TIME: u32 = 2503;
    let mut reindeers = input
        .lines()
        .map(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            let speed: u32 = words[3].parse().unwrap();
            let time_a: u32 = words[6].parse().unwrap();
            let time_b: u32 = words[13].parse().unwrap();
            Reindeer {
                speed,
                time_a,
                time_b,
                score: 0,
            }
        })
        .collect::<Vec<_>>();
    for t in 1..=TIME {
        let max_d = reindeers.iter().map(|r| r.distance(t)).max().unwrap();
        for rein in &mut reindeers {
            if rein.distance(t) == max_d {
                rein.score += 1;
            }
        }
    }
    reindeers.iter().map(|r| r.score).max()
}
