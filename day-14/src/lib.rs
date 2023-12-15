use std::collections::BTreeMap;

pub fn part_one(input: &str) -> u32 {
    let line_len = input.lines().next().unwrap().len() as u32;
    let rolled_field = parse(input, line_len);

    let n_rows = rolled_field.values().map(|f| f.y).max().unwrap() + 1;

    rolled_field
        .values()
        .map(|cell| {
            if cell.kind == CellType::Ball {
                n_rows - cell.y
            } else {
                0
            }
        })
        .sum()
}

fn _print_thing(field: &BTreeMap<u32, Cell>) {
    for row_num in 0..field.values().map(|f| f.y).max().unwrap() {
        let new_row = field
            .values()
            .filter(|cell| cell.y == row_num)
            .map(|cell| match cell.kind {
                CellType::Ball => '0',
                CellType::Dirt => '.',
                CellType::Wall => '#',
            })
            .collect::<String>();
        println!("{new_row}");
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Ball,
    Wall,
    Dirt,
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    x: u32,
    y: u32,
    kind: CellType,
}

fn parse(input: &str, line_len: u32) -> BTreeMap<u32, Cell> {
    let mut rolled_field = BTreeMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, kind)| {
            let kind = match kind {
                '#' => CellType::Wall,
                'O' => CellType::Ball,
                '.' => CellType::Dirt,
                _ => panic!("Didn't get a valid input"),
            };

            let cell = Cell {
                x: x as u32,
                y: y as u32,
                kind,
            };

            get_northern_position(&mut rolled_field, cell, line_len);
        });
    });

    rolled_field
}

fn get_northern_position(rolled_field: &mut BTreeMap<u32, Cell>, cell: Cell, line_len: u32) {
    if cell.kind != CellType::Ball || cell.y == 0 {
        rolled_field.insert(line_len * cell.y + cell.x, cell);
        return;
    }

    let col_indices: Vec<(u32, CellType)> = rolled_field
        .values()
        .filter(|col_cell| col_cell.x == cell.x)
        .rev()
        .enumerate()
        .map(|(idx, col_cell)| (idx as u32, col_cell.kind.clone()))
        .collect();

    for (col_idx, col_kind) in col_indices {
        if col_kind == CellType::Dirt {
            rolled_field.insert(
                cell.y * line_len + cell.x,
                Cell {
                    x: cell.x,
                    y: cell.y,
                    kind: CellType::Dirt,
                },
            );

            if cell.y - col_idx - 1 == 0 {
                rolled_field.insert(
                    cell.x,
                    Cell {
                        x: cell.x,
                        y: 0,
                        kind: CellType::Ball,
                    },
                );
                break;
            }

            continue;
        }

        let new_y = cell.y - col_idx;
        rolled_field.insert(
            new_y * line_len + cell.x,
            Cell {
                x: cell.x,
                y: new_y,
                kind: cell.kind,
            },
        );

        break;
    }
}

pub fn part_two(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part_one(&input), 136)
    }

    // #[test]
    // fn test_part_two() {
    //     let input = "";
    //     assert_eq!(part_one(&input), 0)
    // }
}
