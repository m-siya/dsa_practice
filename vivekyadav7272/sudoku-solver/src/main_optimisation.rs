#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: u8,
    y: u8,
}

pub struct Solution;
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row_elems = vec![vec![false; 9]; 9];
        let mut col_elems = vec![vec![false; 9]; 9];
        let mut block_elems = vec![vec![false; 9]; 9];

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
                                row_elems[i][num as usize - 1] = true;
                                col_elems[j][num as usize - 1] = true;
                                block_elems[(i / 3 * 3 + j / 3) as usize][num as usize - 1] = true;
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
        row_elems: &mut Vec<Vec<bool>>,
        col_elems: &mut Vec<Vec<bool>>,
        block_elems: &mut Vec<Vec<bool>>,
        unfilled: &mut Vec<Coord>,
    ) -> bool {
        let coord = match unfilled.pop() {
            Some(coord) => coord,
            None => return true,
        };
        assert!(coord.x < 9);
        assert!(coord.y < 9);

        for k in 1..=9 {
            if !row_elems[coord.x as usize][k - 1]
                && !col_elems[coord.y as usize][k - 1]
                && !block_elems[(coord.x / 3 * 3 + coord.y / 3) as usize][k - 1]
            {
                row_elems[coord.x as usize][k - 1] = true;
                col_elems[coord.y as usize][k - 1] = true;
                block_elems[(coord.x / 3 * 3 + coord.y / 3) as usize][k - 1] = true;

                board[coord.x as usize][coord.y as usize] = k as u8;

                let solve_further =
                    Self::_solve_sudoku(board, row_elems, col_elems, block_elems, unfilled);
                if solve_further {
                    return true;
                } else {
                    row_elems[coord.x as usize][k - 1] = false;
                    col_elems[coord.y as usize][k - 1] = false;
                    block_elems[(coord.x / 3 * 3 + coord.y / 3) as usize][k - 1] = false;

                    board[coord.x as usize][coord.y as usize] = 0;
                }
            }
        }

        unfilled.push(coord);
        false
    }
}
