use std::fmt::Display;
use std::io;
use std::vec;

fn main() {
    let mut state = State {
        board: vec![vec![Box::NotChecked; 3]; 3],
        ended: false,
        current_player: Player::One,
    };
    while !state.ended {
        println!("Player : {:}", state.current_player);
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("expected an number input");
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if number < 1 || number > 9 {
            println!("input a number between 1 and 9");
        }
        let result = state.mark(number as usize);
        if !result {
            println!("It is already marked");
        }
        state.print();
    }
    println!("Winner is Player {:}", state.current_player);
}

//Player enum denoting which player
#[derive(Copy, Clone, PartialEq, Eq)]
enum Box {
    PlayerOne,
    PlayerTwo,
    NotChecked,
}

impl Box {
    fn player(&self) -> Option<Player> {
        match self {
            Box::PlayerOne => Some(Player::One),
            Box::PlayerTwo => Some(Player::Two),
            _ => None,
        }
    }
}

impl Display for Box {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let res = match self {
            Box::PlayerOne => "x",
            Box::PlayerTwo => "o",
            _ => " ",
        };
        write!(fmt, "{}", res)
    }
}

enum Player {
    One = 1,
    Two,
}

impl Display for Player {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let res = match self {
            Player::One => "x",
            Player::Two => "o",
        };
        write!(fmt, "{}", res)
    }
}

struct State {
    board: Vec<Vec<Box>>,
    ended: bool,
    current_player: Player,
}

impl Player {
    fn player(&self) -> Box {
        match self {
            Player::One => Box::PlayerOne,
            Player::Two => Box::PlayerTwo,
        }
    }
}

impl State {
    fn print(&self) {
        for row in 0..self.board.len() {
            for col in 0..self.board[row].len() {
                print!(" {:} ", self.board[row][col]);
                if col != 2 {
                    print!("|");
                }
            }
            if row != 2 {
                print!("\n -   -   - \n")
            }
        }
        print!("\n");
    }

    fn mark(&mut self, index: usize) -> bool {
        let y = match index {
            i if i > 6 => 2,
            i if i > 3 => 1,
            _ => 0,
        };
        let x = match index {
            i if i % 3 == 0 => 2,
            i if i % 3 == 2 => 1,
            _ => 0,
        };
        if self.board[y][x] != Box::NotChecked {
            return false;
        }
        self.board[y][x] = self.current_player.player();
        self.check_winner();
        true
    }

    fn check_winner(&mut self) {
        for i in 0..3 {
            let res = self.check_vertical(i as usize);
            if !res.is_none() {
                self.ended = true;
                return;
            }
            let res = self.check_horizontal(i as usize);
            if !res.is_none() {
                self.ended = true;
                return;
            }
        }

        let res = self.check_dialognal();
        if !res.is_none() {
            self.ended = true;
            return;
        }
        self.current_player = match self.current_player {
            Player::One => Player::Two,
            Player::Two => Player::One,
        };
    }

    fn check_vertical(&self, index: usize) -> Option<Player> {
        if self.board[0][index] == self.board[1][index]
            && self.board[0][index] == self.board[2][index]
        {
            return self.board[0][index].player();
        }
        None
    }

    fn check_horizontal(&self, index: usize) -> Option<Player> {
        if self.board[index][0] == self.board[index][1]
            && self.board[index][0] == self.board[index][2]
        {
            return self.board[index][0].player();
        }
        None
    }

    fn check_dialognal(&self) -> Option<Player> {
        if self.board[0][0] == self.board[1][1] && self.board[0][0] == self.board[2][2] {
            return self.board[0][0].player();
        } else if self.board[0][2] == self.board[1][1] && self.board[0][2] == self.board[2][0] {
            return self.board[0][2].player();
        }
        None
    }
}
