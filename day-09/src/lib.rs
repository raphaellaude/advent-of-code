pub fn part_one(input: &str) -> i64 {
    parse(input, diff_fold_vec)
}

pub fn part_two(input: &str) -> i64 {
    parse(input, diff_fold_vec_start)
}

fn parse(input: &str, f: fn(Vec<i64>) -> i64) -> i64 {
    input
        .lines()
        .map(|v| {
            f(v.split(' ')
                .into_iter()
                .map(|t| t.parse::<i64>().unwrap())
                .collect())
        })
        .sum()
}

fn diff_fold_vec(v: Vec<i64>) -> i64 {
    let mut total: i64 = 0;

    let l = v.len();

    if v.iter().all(|x| *x == 0) {
        return total + v[l - 1];
    }

    total += diff_fold_vec(
        v[1..]
            .iter()
            .enumerate()
            .map(|(idx, x)| x - v[idx])
            .collect(),
    );

    total + v[l - 1]
}

fn diff_fold_vec_start(v: Vec<i64>) -> i64 {
    let mut total: i64 = 0;

    if v.iter().all(|x| *x == 0) {
        return v[0] - total;
    }

    total += diff_fold_vec_start(
        v[1..]
            .iter()
            .enumerate()
            .map(|(idx, x)| x - v[idx])
            .collect(),
    );

    v[0] - total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part_one(input), 114)
    }

    #[test]
    fn test_part_two() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part_two(input), 2)
    }
}
