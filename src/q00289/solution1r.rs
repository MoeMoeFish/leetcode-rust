impl Solution {
    /**
     * elem = 2 if live -> dead
     * elem = 3 if dead -> live
     */
    // 
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        if board.len() == 0 || board[0].len() == 0 {
            return
        }

        let (n, m) = (board.len(), board[0].len());

        for j in 0..n {
            for i in 0..m  {
                let mut live_num = 0;

                if i > 0 {
                    live_num = Solution::add_live(live_num, board[j][i-1]);

                    if j > 0 {
                        live_num = Solution::add_live(live_num, board[j-1][i-1]);
                    }

                    if j < n - 1 {
                        live_num = Solution::add_live(live_num, board[j+1][i-1]);
                    }
                }

                if j > 0 {
                    live_num = Solution::add_live(live_num, board[j-1][i]);
                }

                if j < n - 1 {
                    live_num = Solution::add_live(live_num, board[j+1][i]);
                }

                if i < m - 1 {
                    live_num = Solution::add_live(live_num, board[j][i+1]);

                    if j > 0 {
                        live_num = Solution::add_live(live_num, board[j-1][i+1]);
                    }
                    if j < n - 1 {
                        live_num = Solution::add_live(live_num, board[j+1][i+1]);
                    }
                }

                if board[j][i] == 1 && (live_num < 2 || live_num > 3) {
                    board[j][i] = 2;
                }

                if board[j][i] == 0 && live_num == 3 { 
                    board[j][i] = 3;
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if board[j][i] == 2 {
                    board[j][i] = 0;
                }
                if board[j][i] == 3 {
                    board[j][i] = 1;
                }
            }
        }

    }

    fn add_live(live_num: i32, num: i32) -> i32 {
        if num == 1 || num == 2 {
            return live_num + 1;
        }
        live_num
    }
}

pub(crate) struct Solution;