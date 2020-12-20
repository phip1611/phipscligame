use crate::game::Game;
use gilrs::{Button, GamepadId, Gilrs, EventType, Gamepad};
use std::sync::{Arc, Mutex};
use std::io::{stdin, stdout, Write};
use std::collections::HashMap;

mod component;
mod field;
mod game;
mod state;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let gamepad_id = get_gamepad_id(&mut gilrs);
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

    // Refresh one final time in the end when the player won
    game.refresh();

    // print won text
    let won_txt = "YOU WON!";
    let times = game.cols() / 2 - won_txt.len() / 2;
    println!("{}YOU WON!", " ".repeat(times));
}

/// Gets a valid gamepad or panics
fn get_gamepad_id(gilrs: &mut Gilrs) -> GamepadId {
    let pads = gilrs
        .gamepads()
        .map(|(_id, gamepad)| gamepad)
        .filter(|gamepad| gamepad.is_connected())
        .filter(|gamepad| gamepad.is_ff_supported())
        .collect::<Vec<Gamepad>>();

    if pads.is_empty() {
        panic!("There are no connected gamepads that support force feedback!");
    }

    // if there is exactly one gamepad we just use this one
    if pads.len() == 1 {
        return pads[0].id();
    }

    // if there is only

    println!("Found the following connected gamepads with force feedback support:");
    for pad in &pads {
        println!("    {}: {} ({:?})", pad.id(), pad.name(), pad.power_info());
    }

    let mut pad_map: HashMap<usize, &Gamepad> = HashMap::new();
    pads.iter().for_each(|pad| {
        pad_map.insert(pad.id().into(), pad);
    });

    let gamepad_id;

    // Ask for gamepad id until valid
    loop {
        print!("Continue? (enter gamepad id) ");
        stdout().flush().unwrap(); // print stdout
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        // removes "\n" from input; on windows also \r
        input = input.replace('\r', "");
        input = input.replace('\n', "");
        let id = input.parse::<usize>();
        if id.is_err() {
            continue;
        }

        let id = id.unwrap();

        // check if gamepad id is valid
        let gamepad = pad_map.get(&id);
        if gamepad.is_none() {
            // Ask again
            continue;
        } else {
            // found valid gamepad
            gamepad_id = gamepad.unwrap().id();
            break;
        }
    }

    gamepad_id
}

fn next_gamepad_button_event_blocking(gilrs: &mut Gilrs) -> gilrs::Event {
    loop {
        if let Some(evt) = gilrs.next_event() {
            return evt;
        }
    }
}
