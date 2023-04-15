struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        // [0][0] is ambiguous, it may refer to kill zero'th row or kill 0'th col
        // so we disambiguate this by deliberately assigning it the meaning of
        // killing the zero'th row.

        let mut kill_zeroth_col = false; // This is what u must use if u wanna indicate killilng the zero'th col.

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;

                    if j != 0 {
                        matrix[0][j] = 0;
                    } else {
                        kill_zeroth_col = true;
                    }
                }
            }
        }

        for i in 1..m {
            let flag = matrix[i][0];
            if flag == 0 {
                matrix[i].fill(0);
            }
        }

        for j in 1..n {
            let flag = matrix[0][j];
            if flag == 0 {
                for i in 0..m {
                    matrix[i][j] = 0;
                }
            }
        }

        if matrix[0][0] == 0 {
            matrix[0].fill(0);
        }

        if kill_zeroth_col {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
