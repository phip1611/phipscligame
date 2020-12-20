use crate::game::Game;
use gilrs::{Button, GamepadId, Gilrs, EventType};
use std::sync::{Arc, Mutex};

mod component;
mod field;
mod game;
mod state;

fn main() {
    let gilrs = Gilrs::new().unwrap();
    let gamepads = gilrs
        .gamepads()
        .filter(|(_id, gamepad)| gamepad.name().to_lowercase().contains("xbox"))
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

        // reset after each input so that a new button pressed event is required
        let mut is_up = false;
        let mut is_left= false;
        let mut is_down= false;
        let mut is_right= false;

        // prevent nested lock
        {
            let mut gilrs = gilrs.lock().unwrap();
            let event = next_gamepad_button_event_blocking(&mut gilrs);

            if let EventType::ButtonPressed(btn, _) = event.event {
                if btn == Button::North || btn == Button::DPadUp {
                    is_up = true;
                }
                else if btn == Button::West || btn == Button::DPadLeft {
                    is_left = true;
                }
                else if btn == Button::South || btn == Button::DPadDown {
                    is_down = true;
                }
                else if btn == Button::East || btn == Button::DPadRight {
                    is_right = true;
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

fn next_gamepad_button_event_blocking(gilrs: &mut Gilrs) -> gilrs::Event {
    loop {
        if let Some(evt) = gilrs.next_event() {
            return evt;
        }
    }
}
