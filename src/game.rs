use crate::state::GameState;
use crate::field::{GameField, FieldType};
use crate::component::{RefreshableComponent};
use std::io::stdout;
use crossterm::ExecutableCommand;
use crossterm::terminal::{Clear, ClearType};
use gilrs::{GamepadId, Gilrs};
use gilrs::ff::{EffectBuilder, BaseEffect, BaseEffectType, Replay, Ticks};
use std::thread::{sleep, spawn};
use std::time::Duration;
use std::sync::{Mutex, Arc};

pub const COIN_COUNT: usize = 17;

#[derive(Debug)]
pub struct Game {
    state: GameState,
    field: GameField,
    is_initial_refresh: bool,
    gamepad_id: GamepadId,
    gilrs: Arc<Mutex<Gilrs>>,
}

impl Game {

    pub fn new(cols: u8, rows: u8, gilrs: Arc<Mutex<Gilrs>>, gamepad_id: GamepadId) -> Self {
        let mut gf = GameField::new(cols, rows);
        stdout().execute(Clear(ClearType::All)).unwrap();
        gf.init();
        Self {
            state: GameState::new(cols),
            field: gf,
            is_initial_refresh: true,
            gamepad_id,
            gilrs,
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

    fn gamepad_do_coin_ff_effect(&mut self) {
        let id = self.gamepad_id;
        let gilrs = self.gilrs.clone();
        spawn(move || {
            // fast release lock
            let effect = {
                let mut gilrs = gilrs.lock().unwrap();
                let duration = Ticks::from_ms(250);
                EffectBuilder::new()
                    .add_effect(BaseEffect {
                        kind: BaseEffectType::Weak { magnitude: u16::MAX },
                        scheduling: Replay { play_for: duration * 1, with_delay: duration * 0, ..Default::default() },
                        envelope: Default::default(),
                    })
                    .gamepads(&vec![id])
                    .finish(&mut gilrs).unwrap()
            };
            effect.play().unwrap();
            sleep(Duration::from_millis(250));
            effect.stop().unwrap();
        });
    }

    fn gamepad_do_barrier_ff_effect(&mut self) {
        let id = self.gamepad_id;
        let gilrs = self.gilrs.clone();
        spawn(move || {
            // fast release lock
            let effect = {
                let mut gilrs = gilrs.lock().unwrap();
                let duration = Ticks::from_ms(350);
                EffectBuilder::new()
                    // xbox controller has two vibration motors
                    // one left and one right; trigger both for maximum force feedback
                    .add_effect(BaseEffect {
                        kind: BaseEffectType::Strong { magnitude: u16::MAX },
                        scheduling: Replay { play_for: duration * 1, with_delay: duration * 0, ..Default::default() },
                        envelope: Default::default(),
                    })
                    .add_effect(BaseEffect {
                        kind: BaseEffectType::Weak { magnitude: u16::MAX },
                        scheduling: Replay { play_for: duration * 1, with_delay: duration * 0, ..Default::default() },
                        envelope: Default::default(),
                    })
                    .gamepads(&vec![id])
                    .finish(&mut gilrs).unwrap()
            };

            effect.play().unwrap();
            sleep(Duration::from_millis(500));
            effect.stop().unwrap();
        });
    }

    fn inc_if_necessary(&mut self, res: Result<FieldType, FieldType>) {
        match res {
            Ok(field) => {
                if field == FieldType::Coin {
                    self.state.coins_inc();
                    self.gamepad_do_coin_ff_effect();
                }
            }
            Err(field) => {
                match field {
                    FieldType::Space => {}
                    FieldType::Player => {}
                    FieldType::Barrier => {
                        self.gamepad_do_barrier_ff_effect()
                    }
                    FieldType::Coin => {}
                }
            }
        }
    }

    /// Getter for [`cols`].
    pub fn cols(&self) -> usize {
        self.field.cols()
    }

    /*/// Getter for [`rows`].
    pub fn rows(&self) -> usize {
        self.field.rows()
    }*/
}

