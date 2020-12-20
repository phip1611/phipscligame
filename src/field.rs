use rand::{thread_rng, Rng};
use std::ops::Index;
use std::io::stdout;
use crossterm::ExecutableCommand;
use crossterm::style::{SetAttribute, Attribute, Print, ResetColor, SetForegroundColor, Color, SetBackgroundColor};
use crate::component::{RefreshableComponent};
use crate::game::COIN_COUNT;

type GameFieldDataType = Box<[Box<[FieldType]>]>;



#[derive(Debug)]
pub struct GameField {
    cols: usize,
    rows: usize,
    gamefield: GameFieldDataType,
    player_pos: (usize, usize),
}

impl GameField {
    pub fn new(cols: u8, rows: u8) -> Self {
        let cols = cols as usize;
        let rows = rows as usize;
        assert!(cols > 3, "There must be more than three cols!");
        assert!(rows > 3, "There must be more than three rows!");
        let mut gamefield = vec![];
        for _ in 0..rows {
            let mut row = vec![];
            for _ in 0..cols {
                row.push(FieldType::Space)
            }
            gamefield.push(row);
        }

        let gamefield = gamefield.into_iter()
            .map(|vec| vec.into_boxed_slice())
            .collect::<Vec<Box<[FieldType]>>>()
            .into_boxed_slice();

        GameField {
            cols,
            rows,
            gamefield,
            player_pos: (0, 0),
        }
    }

    pub fn move_player_to(&mut self, (row, col): (usize, usize)) -> Result<FieldType, FieldType> {
        let next_field = self.gamefield[row][col];
        if next_field != FieldType::Barrier {
            self.gamefield[self.player_pos.0][self.player_pos.1] = FieldType::Space;
            self.player_pos = (row, col);
            self.gamefield[row][col] = FieldType::Player;
            Ok(next_field)
        } else {
            Err(next_field)
        }
    }

    /// Getter for [`rows`]
    fn rows(&self) -> usize {
        self.rows
    }

    /// Getter for [`cols`]
    fn cols(&self) -> usize {
        self.cols
    }

    /// Getter for [`player_pos`]
    pub fn player_pos(&self) -> (usize, usize) {
        self.player_pos
    }

    /// Inits a new game field.
    pub fn init(&mut self) {
        &self.init_game_borders();
        &self.init_random_player_pos();
        &self.init_random_barriers();
        &self.init_random_coins();
    }

    /// Inits the game borders/boundaries.
    fn init_game_borders(&mut self) {
        // print horizontal walls
        for col_i in 0..self.cols() {
            self.gamefield[0][col_i] = FieldType::Barrier;
            self.gamefield[self.rows() - 1][col_i] = FieldType::Barrier;
        }

        // print vertical walls
        for row_i in 0..self.rows() {
            self.gamefield[row_i][0] = FieldType::Barrier;
            self.gamefield[row_i][self.cols() - 1] = FieldType::Barrier;
        }
    }

    fn init_random_barriers(&mut self) {
        const BARRIERS_FACTOR: f64 = 0.25_f64;
        // minus borders left/right and top/bottom minus one for player
        let usable_fields = ((self.cols() - 2)*(self.rows() - 2) - 1) as f64;
        // calculate a good amount of barriers
        let barrier_count = (usable_fields * BARRIERS_FACTOR) as usize;

        let mut actual_barrier_count = 0;
        // iterate through the complete field but without the borders
        while actual_barrier_count != barrier_count {
            let col = thread_rng().gen_range(1, self.cols() - 1);
            let row = thread_rng().gen_range(1, self.rows() - 1);
            if let FieldType::Space = self.gamefield[row][col] {
                self.gamefield[row][col] = FieldType::Barrier;
                actual_barrier_count += 1;
            }
        }
    }

    fn init_random_coins(&mut self) {

        let mut actual_coin_count = 0;
        // iterate through the complete field but without the borders
        while actual_coin_count != COIN_COUNT {
            let col = thread_rng().gen_range(1, self.cols() - 1);
            let row = thread_rng().gen_range(1, self.rows() - 1);
            if let FieldType::Space = self.gamefield[row][col] {
                self.gamefield[row][col] = FieldType::Coin;
                actual_coin_count += 1;
            }
        }
    }

    fn init_random_player_pos(&mut self) {
        let col = thread_rng().gen_range(1, self.cols() - 1);
        let row = thread_rng().gen_range(1, self.rows() - 1);
        self.gamefield[row][col] = FieldType::Player;
        self.player_pos = (row, col);
    }
}

impl Index<(usize, usize)> for GameField {
    type Output = FieldType;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.gamefield[index.0][index.1]
    }
}

impl RefreshableComponent for GameField {

    fn after_print_cols_offset(&self) -> u16 {
        self.cols() as u16
    }

    fn after_print_rows_offset(&self) -> u16 {
        self.rows() as u16
    }

    fn print(&self) {
        for row_i in 0..self.rows() {
            for col_i in 0..self.cols() {
                self.gamefield[row_i][col_i].print();
            }
            println!(); // newline
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FieldType {
    Space,
    Player,
    Barrier,
    Coin,
}

impl FieldType {
    fn symbol(self) -> char {
        match self {
            FieldType::Space => { ' ' }
            FieldType::Player => { 'X' }
            FieldType::Barrier => { '#' }
            FieldType::Coin => { 'C' }
        }
    }

    /// Prints the symbol using ANSI colors.
    fn print(self) {
        match self {
            FieldType::Space => {
                print!("{}", self.symbol());
            }
            FieldType::Player => {
                stdout()
                    .execute(SetForegroundColor(Color::Red)).unwrap()
                    .execute(Print(self.symbol())).unwrap()
                    .execute(ResetColor).unwrap();
            }
            FieldType::Barrier => {
                stdout()
                    .execute(SetBackgroundColor(Color::White)).unwrap()
                    .execute(SetAttribute(Attribute::Bold)).unwrap()
                    .execute(Print(self.symbol())).unwrap()
                    .execute(SetAttribute(Attribute::Reset)).unwrap();
            }
            FieldType::Coin => {
                stdout()
                    .execute(SetAttribute(Attribute::Bold)).unwrap()
                    .execute(SetForegroundColor(Color::Yellow)).unwrap()
                    .execute(Print(self.symbol())).unwrap()
                    .execute(ResetColor).unwrap()
                    .execute(SetAttribute(Attribute::Reset)).unwrap();
            }
        }
    }
}
