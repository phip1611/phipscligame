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

        let mut up_must_be_released = false;
        let mut left_must_be_released = false;
        let mut down_must_be_released = false;
        let mut right_must_be_released = false;
        let mut is_up = false;
        let mut is_left= false;
        let mut is_down= false;
        let mut is_right= false;

        // prevent nested lock
        {
            let mut gilrs = gilrs.lock().unwrap();
            let event = next_gamepad_button_event_blocking(&mut gilrs);

            if let EventType::ButtonPressed(btn, _) = event.event {
                // println!("Button {:#?} pressed!", btn);
                if btn == Button::North || btn == Button::DPadUp {
                    if !up_must_be_released {
                        is_up = true;
                        up_must_be_released = true;
                    }
                }
                else if btn == Button::West || btn == Button::DPadLeft {
                    if !left_must_be_released {
                        is_left = true;
                        left_must_be_released = true;
                    }
                }
                else if btn == Button::South || btn == Button::DPadDown {
                    if !down_must_be_released {
                        is_down = true;
                        down_must_be_released = true;
                    }
                }
                else if btn == Button::East || btn == Button::DPadRight {
                    if !right_must_be_released {
                        is_right = true;
                        right_must_be_released = true;
                    }
                }
            }
            else if let EventType::ButtonReleased(btn, _) = event.event {
                // println!("Button {:#?} released!", btn);
                if btn == Button::North || btn == Button::DPadUp {
                    up_must_be_released = false;
                    is_up = false;
                }
                else if btn == Button::West || btn == Button::DPadLeft {
                    left_must_be_released = false;
                    is_left = false;
                }
                else if btn == Button::South || btn == Button::DPadDown {
                    down_must_be_released = false;
                    is_down = false;
                }
                else if btn == Button::East || btn == Button::DPadRight {
                    right_must_be_released = false;
                    is_right = false;
                }
            }
        };
        // release lock afterwards

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
