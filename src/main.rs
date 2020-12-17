use crate::field::GameField;
use crate::game::Game;
use crossterm::cursor::{MoveDown, MoveLeft, MoveRight, MoveUp};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::ExecutableCommand;
use device_query::{DeviceQuery, DeviceState, Keycode};
use gilrs::ev::AxisOrBtn::Btn;
use gilrs::ff::{BaseEffect, BaseEffectType, EffectBuilder, Replay, Ticks};
use gilrs::{Button, Event, Gamepad, GamepadId, Gilrs, EventType};
use std::io::{stdout, Cursor, Write};
use std::path::Component::CurDir;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};
use gilrs::ev::EventType::ButtonReleased;

mod component;
mod field;
mod game;
mod state;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let gamepads = gilrs
        .gamepads()
        .filter(|(id, gamepad)| gamepad.name().to_lowercase().contains("xbox"))
        .map(|(id, _)| id)
        .collect::<Vec<GamepadId>>();
    if gamepads.len() != 1 {
        panic!("Please connect exactly one XBox Controller Gamepad");
    }
    let gamepad_id = gamepads[0];
    let gilrs = Arc::new(Mutex::new(gilrs));
    let mut game = Game::new(45, 25, gilrs.clone(), gamepad_id);

    /*loop {
        let mut gilrs = gilrs.lock().unwrap();
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            let gamepad = gilrs.gamepad(id);
            println!(
                "{:#?}",
                gamepad.state()
            );
        }
    }*/

    while !game.won() {
        game.refresh();

        // prevent nested lock
        let mut up_must_be_released = false;
        let mut left_must_be_released = false;
        let mut down_must_be_released = false;
        let mut right_must_be_released = false;
        let is_up;
        let is_left;
        let is_down;
        let is_right;
        // release lock afterwards
        {
            let mut gilrs = gilrs.lock().unwrap();
            let event = next_gamepad_button_event_blocking(&mut gilrs);

            if let EventType::ButtonReleased(btn, _) = event.event {
                if btn == Button::North || btn == Button::DPadUp {
                    up_must_be_released = false;
                }
                else if btn == Button::West || btn == Button::DPadLeft {
                    left_must_be_released = false;
                }
                else if btn == Button::South || btn == Button::DPadDown {
                    down_must_be_released = false;
                }
                else if btn == Button::East || btn == Button::DPadRight {
                    right_must_be_released = false;
                }
            }

            let gamepad = gilrs.gamepad(event.id);
            is_up = !up_must_be_released && (gamepad.is_pressed(Button::North) || gamepad.is_pressed(Button::DPadUp));
            is_left = !left_must_be_released && (gamepad.is_pressed(Button::West) || gamepad.is_pressed(Button::DPadLeft));
            is_down = !down_must_be_released && (gamepad.is_pressed(Button::South) || gamepad.is_pressed(Button::DPadDown));
            is_right = !right_must_be_released && (gamepad.is_pressed(Button::East) || gamepad.is_pressed(Button::DPadRight));

            if is_up { up_must_be_released = true; }
            if is_left { left_must_be_released = true; }
            if is_down { down_must_be_released = true; }
            if is_right { right_must_be_released = true; }
        };

        if is_up {
            game.move_player_up();
        } else if is_left {
            game.move_player_left();
        } else if is_down {
            game.move_player_down();
        } else if is_right {
            game.move_player_right();
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

fn next_gamepad_button_event_blocking(gilrs: &mut Gilrs) -> gilrs::Event {
    loop {
        if let Some(evt) = gilrs.next_event() {
            return evt;
        }
    }
}
