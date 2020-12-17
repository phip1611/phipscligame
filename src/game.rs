use crate::state::GameState;
use crate::field::{GameField, FieldType};
use crate::component::{RefreshableComponent};
use std::io::stdout;
use crossterm::ExecutableCommand;
use crossterm::terminal::{Clear, ClearType};

pub const COIN_COUNT: usize = 1;

#[derive(Debug)]
pub struct Game {
    state: GameState,
    field: GameField,
    is_initial_refresh: bool,
}

impl Game {

    pub fn new(cols: u8, rows: u8) -> Self {
        let mut gf = GameField::new(cols, rows);
        stdout().execute(Clear(ClearType::All)).unwrap();
        gf.init();
        Self {
            state: GameState::new(cols),
            field: gf,
            is_initial_refresh: true,
        }
    }

    pub fn refresh(&mut self) {
        if self.is_initial_refresh {
            self.is_initial_refresh = false;
        } else {
            self.state.reset_cursor();
            self.field.reset_cursor();
        }
        self.state.print();
        self.field.print();
    }

    pub fn move_player_up(&mut self) {
        let next_pos = (self.field.player_pos().0 - 1, self.field.player_pos().1);
        let res = self.field.move_player_to(next_pos);
        self.inc_if_necessary(res);
    }

    pub fn move_player_down(&mut self) {
        let next_pos = (self.field.player_pos().0 + 1, self.field.player_pos().1);
        let res = self.field.move_player_to(next_pos);
        self.inc_if_necessary(res);
    }

    pub fn move_player_left(&mut self) {
        let next_pos = (self.field.player_pos().0, self.field.player_pos().1 - 1);
        let res = self.field.move_player_to(next_pos);
        self.inc_if_necessary(res);
    }

    pub fn move_player_right(&mut self) {
        let next_pos = (self.field.player_pos().0, self.field.player_pos().1 + 1);
        let res = self.field.move_player_to(next_pos);
        self.inc_if_necessary(res);
    }

    pub fn won(&self) -> bool {
        self.state.won()
    }

    fn inc_if_necessary(&mut self, res: Result<FieldType, FieldType>) {
        match res {
            Ok(field) => {
                if field == FieldType::Coin {
                    self.state.coins_inc()
                }
            }
            Err(_) => {}
        }
    }
}

