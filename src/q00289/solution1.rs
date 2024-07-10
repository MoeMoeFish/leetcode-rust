impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        if m == 0 {
            return
        }

        let n = board[0].len();

        for i in 0..m {
            for j in 0..n {
                let mut nbs: Vec<i32> = vec![];

                if i > 0 {
                    if j > 0 {
                        nbs.push(board[i-1][j-1])
                    }
                    nbs.push(board[i-1][j]);

                    if j < n - 1 {
                        nbs.push(board[i-1][j+1]);
                    }
                }
                {
                    if j > 0 {
                        nbs.push(board[i][j-1])
                    }

                    if j < n - 1 {
                        nbs.push(board[i][j+1]);
                    }
                }

                if i < m - 1 {
                    if j > 0 {
                        nbs.push(board[i+1][j-1])
                    }
                    nbs.push(board[i+1][j]);

                    if j < n - 1 {
                        nbs.push(board[i+1][j+1]);
                    }
                }

                let mut live_num = 0;
                nbs.iter().for_each(|x| if *x == 1 || *x == 3 { live_num += 1});

                if board[i][j] == 0 {
                    if live_num == 3 {
                        board[i][j] = 2;
                    }
                } else {
                    if live_num < 2 || live_num > 3 {
                        board[i][j] = 3;
                    }
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 2 {
                    board[i][j] = 1;
                }

                if board[i][j] == 3 {
                    board[i][j] = 1;
                }
            }
        }
        
    }
}

pub(crate) struct  Solution;