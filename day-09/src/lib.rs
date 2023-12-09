use itertools::Itertools;

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
    let l = v.len();

    if v.iter().all(|x| *x == 0) {
        return v[l - 1];
    }

    diff_fold_vec(v.iter().tuple_windows().map(|(y, x)| x - y).collect()) + v[l - 1]
}

fn diff_fold_vec_start(v: Vec<i64>) -> i64 {
    if v.iter().all(|x| *x == 0) {
        return v[0];
    }

    v[0] - diff_fold_vec_start(v.iter().tuple_windows().map(|(y, x)| x - y).collect())
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
