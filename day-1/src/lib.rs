pub fn part_a(input: &str) -> i64 {
    let result: Vec<i64> = input
        .trim()
        .split("\n")
        .map(|value| {
            let parsed_value: i64 = value
                .parse()
                .unwrap();
            parsed_value
        })
        .collect();

    let mut increases = 0;
    for i in 1..result.len() {
        if result[i] > result[i - 1] {
            increases += 1;
        }
    }

    increases
}

pub fn part_b(input: &str) -> i64 {
    let n = 3; // window size

    let values: Vec<i64> = input
        .trim()
        .split("\n")
        .map(|v| v.parse::<i64>().unwrap())
        .collect();

    let mut increases = 0;
    let mut sum = 0;
    for i in 0..n {
        sum += values[i];
    }

    for i in n..values.len() {
        let new_sum = sum - values[i - n] + values[i];
        if new_sum > sum {
            increases += 1;
        }

        sum = new_sum;
    }

    increases
}

mod tests {

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("test.txt")), 7);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("test.txt")), 5);
    }
}
