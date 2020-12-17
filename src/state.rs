use crate::component::{RefreshableComponent};
use crate::game::COIN_COUNT;
use std::cmp::min;

#[derive(Debug)]
pub struct GameState {
    coins: usize,
    is_init_refresh: bool,
    cols: u8,
}

impl GameState {

    pub fn new(cols: u8) -> Self {
        Self {
            cols,
            coins: 0,
            is_init_refresh: true,
        }
    }

    fn get_bar_string(&self) -> String {
        format!("Phips-Game: {:>3}/{} Coins", self.coins(), COIN_COUNT)
    }

    fn get_gamestate_string(&self) -> String {
        let stat_bar = self.get_bar_string();
        let len = self.width();
        let mut str = String::new();
        str.push_str(
            &"#".repeat(len)
        );
        str.push('\n');
        str.push_str(&stat_bar);
        str.push('\n');
        str.push_str(
            &"#".repeat(len)
        );
        str
    }

    fn width(&self) -> usize {
        let len = min(self.get_bar_string().len(), self.cols as usize);
        min(len, self.cols as usize)
    }

    pub fn coins_inc(&mut self) {
        self.coins += 1
    }

    fn coins(&self) -> usize {
        self.coins
    }

    pub fn won(&self) -> bool {
        self.coins == COIN_COUNT
    }

}

impl RefreshableComponent for GameState {

    fn after_print_cols_offset(&self) -> u16 {
        self.width() as u16
    }

    fn after_print_rows_offset(&self) -> u16 {
        3
    }

    fn print(&self) {
        println!("{}", self.get_gamestate_string());
    }
}
