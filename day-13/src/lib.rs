use std::collections::BTreeMap;

pub fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .into_iter()
        .map(parse_chunk)
        .map(summarize_reflection)
        .sum()
}

struct Chunk {
    horiz: BTreeMap<u32, String>,
    vert: BTreeMap<u32, String>,
}

fn parse_chunk(input: &str) -> Chunk {
    let mut horiz = BTreeMap::new();
    input.lines().enumerate().for_each(|(idx, l)| {
        horiz.insert(idx as u32, l.to_string());
    });

    let mut vert = BTreeMap::new();
    (0..input.lines().next().unwrap().len())
        .into_iter()
        .for_each(|idx| {
            let col = horiz
                .values()
                .map(|col| col.chars().nth(idx).unwrap())
                .collect::<String>();
            vert.insert(idx as u32, col);
        });

    Chunk { horiz, vert }
}

fn summarize_reflection(chunk: Chunk) -> usize {
    println!("GETTING VERT REFLECTIONS");
    let vert_reflections = n_reflections(chunk.vert);

    println!("GETTING HORIZ REFLECTIONS");
    let horiz_reflections = n_reflections(chunk.horiz);

    dbg!(&vert_reflections, &horiz_reflections);

    let vert_max_ids = get_max_by_index(&vert_reflections);
    let horiz_max_ids = get_max_by_index(&horiz_reflections);

    dbg!(vert_max_ids, horiz_max_ids);

    let v_max = vert_reflections.iter().max();
    let h_max = horiz_reflections.iter().max();

    if v_max >= h_max {
        vert_max_ids.unwrap()
    } else {
        horiz_max_ids.unwrap() * 100
    }
}

fn n_reflections(btree: BTreeMap<u32, String>) -> Vec<usize> {
    let n = btree.len();
    (0..n)
        .into_iter()
        .map(|c| {
            let count = (0..c)
                .into_iter()
                .filter(|idx| {
                    btree.get(&((c - (idx + 1)) as u32)) == btree.get(&((c + idx) as u32))
                })
                .count();
            if n - count == c || count >= c {
                count
            } else {
                0
            }
        })
        .collect()
}

fn get_max_by_index(reflection_counts: &Vec<usize>) -> Option<usize> {
    reflection_counts
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
}

pub fn part_two(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        405
    )]
    #[case(
        ".####..
###..#.
..#.###
#.####.
#.####.
..#.###
###..#.
.####..
.....#.
...#.#.
.####..
###..#.
..#.###",
        400
    )]
    #[case(
        "###.#.###.#.#.###
.#.........####.#
...#.#####.#.####
###.....##.##...#
#.######...##.#.#
.#.#.#.......##..
.#.#.#.......##..
#.######...##.#.#
###.....##.##...#
...#.#####.#.####
.#....#....####.#
###.#.###.#.#.###
###.#.###.#.#.###",
        1200
    )]
    fn test_part_one(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part_one(input), expected)
    }

    // #[test]
    // fn test_part_two() {
    //     let input = "";
    //     assert_eq!(part_one(&input), 0)
    // }
}
