use std::collections::HashSet;


struct Solution;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut col_elems = vec![HashSet::<u8>::new(); 9];
        let mut block_elems = vec![HashSet::<u8>::new(); 9];

        for (i, row) in board.iter().enumerate() {
            let mut row_elems = HashSet::<u8>::new();
            for (j, cell) in row.iter().enumerate() {
                
                if *cell == '.' {continue;}
                let num = (*cell as u32 - '0' as u32) as u8;
                if row_elems.contains(&num) || col_elems[j].contains(&num) || block_elems[(i/3)*3 + j/3].contains(&num) {
                    return false;
                }

                row_elems.insert(num);
                col_elems[j].insert(num);
                block_elems[(i/3)*3 + j/3].insert(num);
            }
        }


        true
    }
}

fn main() {
    println!("Hello, world!");
}
