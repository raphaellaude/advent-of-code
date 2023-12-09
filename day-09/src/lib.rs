pub fn part_one<'a>(input: &'a str) -> i64 {
    let vals: Vec<Vec<i64>> = input
        .lines()
        .map(|v| {
            v.split(' ')
                .into_iter()
                .map(|t| t.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    vals.into_iter().map(diff_fold_vec).sum()
}

fn diff_fold_vec(v: Vec<i64>) -> i64 {
    let mut total: i64 = 0;
    // dbg!(&v);

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
}
