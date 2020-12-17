use crate::field::GameField;
use crossterm::style::{SetForegroundColor, SetBackgroundColor, Print, ResetColor, Color};
use crossterm::ExecutableCommand;
use std::io::{Cursor, stdout, Write};
use std::path::Component::CurDir;
use crossterm::cursor::{MoveUp, MoveLeft, MoveDown, MoveRight};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::game::Game;

mod field;
mod game;
mod state;
mod component;

fn main() {
    let mut game = Game::new(45, 25);

    let device_state = DeviceState::new();
    while !game.won() {
        const MAX_ACTIONS_PER_SECOND: u64 = 5;
        sleep(Duration::from_millis(1000 / MAX_ACTIONS_PER_SECOND));
        game.refresh();
        let key = get_next_key(&device_state);
        match key {
            Keycode::W => { game.move_player_up(); }
            Keycode::S => { game.move_player_down(); }
            Keycode::A => { game.move_player_left(); }
            Keycode::D => { game.move_player_right(); }
            _ => {}
        }
    }

}

fn get_next_key(device_state: &DeviceState) -> Keycode {
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.is_empty() {
            continue;
        } else {
            return keys[0].clone();
        }
    }

}
