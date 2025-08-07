use rusty_time::Timer;
use crate::{NUM_ROWS, NUM_COLS};
use crate::frame::{Drawable, Frame};
use std::time::Duration;
use std::cmp::max;

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if (x > 1)
                    && (x < NUM_COLS -2)
                    && (y > 0) 
                    && (y < 9)
                    && (x % 2 == 0) // spread them out so they don't take up every slot
                    && (y % 2 == 0) 
                    && (x == 2) { // for testing
                        army.push(Invader {x, y});
                    }
            }
        }
        Self {
            army,
            move_timer: Timer::new(Duration::from_millis(2000)),
            direction:1,
        }
    
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.tick(delta);
        if self.move_timer.finished() {
            self.move_timer.reset();
            let mut downwards: bool = false;
            if self.direction == -1 { // if going left
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else { // if going right
                    let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                    if max_x == NUM_COLS -1 {
                        self.direction = -1; //change direction
                        downwards = true;
                    }
                }
            if downwards { // either moving down or moving to the side. Here, invaders are moving down and increasing speed.
                let new_duration = max(self.move_timer.duration().as_millis() - 250, 250);
                self.move_timer //= Timer::from_millis(new_duration as u64);
                    .set_duration(Duration::from_millis(new_duration as u64));
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else { // Here invaders are moving laterally, not downwards 
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize; // cast uing "as"
                }
            }
            true
        } else {
            false
        }
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }
    pub fn kill_invader_at(&mut self, x:usize, y: usize) -> bool {
        if let Some(idx) = self 
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y)) {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.remaining().as_secs_f32() / self.move_timer.duration().as_secs_f32()) > 0.5 { // updated .time_left to .remaining()
                "x"
            }else {
                "+"
            };
            
        }
    }
}
