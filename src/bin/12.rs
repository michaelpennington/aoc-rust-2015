use serde_json::Value;

advent_of_code::solution!(12);

fn process(val: &Value) -> i64 {
    match val {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(a) => a.iter().map(process).sum(),
        Value::Object(map) => map.values().map(process).sum(),
    }
}

fn processv2(val: &Value) -> i64 {
    match val {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(a) => a.iter().map(processv2).sum(),
        Value::Object(map) => {
            if map.values().any(|v| v == "red") {
                0
            } else {
                map.values().map(processv2).sum()
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let json = serde_json::from_str(input).ok()?;
    Some(process(&json) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let json = serde_json::from_str(input).ok()?;
    Some(processv2(&json) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
