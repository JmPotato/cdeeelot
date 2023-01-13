// 79.[word-search](https://leetcode.cn/problems/word-search)

use crate::Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if word.len() == 0 {
            return true;
        }
        let m = board.len();
        if m == 0 {
            return false;
        }
        let n = board[0].len();
        if n == 0 {
            return false;
        }
        let mut mark_board = vec![vec![false; n]; m];
        let alpha = word.chars().nth(0).unwrap();
        for (x, line) in board.iter().enumerate() {
            for (y, row) in line.iter().enumerate() {
                if *row != alpha {
                    continue;
                }
                mark_board[x][y] = true;
                if Self::backtrace(x, y, board.clone(), mark_board.clone(), &word[1..]) {
                    return true;
                }
                mark_board[x][y] = false;
            }
        }
        false
    }

    fn backtrace(
        x: usize,
        y: usize,
        board: Vec<Vec<char>>,
        mut mark_board: Vec<Vec<bool>>,
        word: &str,
    ) -> bool {
        if word.len() == 0 {
            return true;
        }
        let alpha = word.chars().nth(0).unwrap();
        let (m, n) = (board.len(), board[0].len());
        for direct in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_x = x as i64 + direct.0;
            let next_y = y as i64 + direct.1;
            if !(0 <= next_x && next_x < m as i64 && 0 <= next_y && next_y < n as i64) {
                continue;
            }
            let (next_x, next_y) = (next_x as usize, next_y as usize);
            if board[next_x][next_y] != alpha {
                continue;
            }
            if mark_board[next_x][next_y] {
                continue;
            }
            mark_board[next_x][next_y] = true;
            if Self::backtrace(
                next_x,
                next_y,
                board.clone(),
                mark_board.clone(),
                &word[1..],
            ) {
                return true;
            }
            mark_board[next_x][next_y] = false;
        }
        false
    }
}

mod tests {
    use crate::Solution;

    #[test]
    fn test_exist() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word: String = "ABCCED".into();
        assert!(Solution::exist(board, word));
    }
}
