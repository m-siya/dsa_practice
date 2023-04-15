mod main_optimisation;

use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: u8,
    y: u8,
}
struct Solution;
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row_elems = vec![HashSet::<u8>::new(); 9];
        let mut col_elems = vec![HashSet::<u8>::new(); 9];
        let mut block_elems = vec![HashSet::<u8>::new(); 9];

        let mut unfilled = Vec::with_capacity(9 * 9);

        let mut num_board = board
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, c)| {
                        c.to_digit(10)
                            .map(|num| {
                                row_elems[i].insert(num as u8);
                                col_elems[j].insert(num as u8);
                                block_elems[(i / 3 * 3 + j / 3) as usize].insert(num as u8);
                                num as u8
                            })
                            .unwrap_or_else(|| {
                                // must've been a '.', nibba be unfilled.
                                unfilled.push(Coord {
                                    x: i as u8,
                                    y: j as u8,
                                });
                                0
                            })
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        assert_eq!(num_board.len(), 9);
        assert!(num_board.iter().all(|row| row.len() == 9));
        assert_eq!(row_elems.len(), 9);
        assert_eq!(col_elems.len(), 9);
        assert_eq!(block_elems.len(), 9);
        assert!(unfilled.len() <= 9 * 9);

        let solved = Self::_solve_sudoku(
            &mut num_board,
            &mut row_elems,
            &mut col_elems,
            &mut block_elems,
            &mut unfilled,
        );

        assert_eq!(solved, true);

        for (i, row) in num_board.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                board[i][j] = std::char::from_digit(*num as u32, 10).unwrap();
            }
        }
    }

    fn _solve_sudoku(
        board: &mut Vec<Vec<u8>>,
        row_elems: &mut Vec<HashSet<u8>>,
        col_elems: &mut Vec<HashSet<u8>>,
        block_elems: &mut Vec<HashSet<u8>>,
        unfilled: &mut Vec<Coord>,
    ) -> bool {
        let coord = match unfilled.pop() {
            Some(coord) => coord,
            None => return true,
        };

        for k in 1..=9 {
            if !row_elems[coord.x as usize].contains(&k)
                && !col_elems[coord.y as usize].contains(&k)
                && !block_elems[(coord.x / 3 * 3 + coord.y / 3) as usize].contains(&k)
            {
                row_elems[coord.x as usize].insert(k);
                col_elems[coord.y as usize].insert(k);
                block_elems[(coord.x / 3 * 3 + coord.y / 3) as usize].insert(k);

                board[coord.x as usize][coord.y as usize] = k;

                let solve_further =
                    Self::_solve_sudoku(board, row_elems, col_elems, block_elems, unfilled);
                if solve_further {
                    return true;
                } else {
                    row_elems[coord.x as usize].remove(&k);
                    col_elems[coord.y as usize].remove(&k);
                    block_elems[(coord.x / 3 * 3 + coord.y / 3) as usize].remove(&k);

                    board[coord.x as usize][coord.y as usize] = 0;
                }
            }
        }

        unfilled.push(coord);
        false
    }
}

fn main() {
    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    main_optimisation::Solution::solve_sudoku(&mut board);

    for row in board {
        println!("{:?}", row);
    }
}
