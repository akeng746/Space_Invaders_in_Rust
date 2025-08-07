use crate::frame::{Drawable, Frame};
// use crate::NUM_COLS;
use std::time::Duration;
use rusty_time::Timer;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub prev_y: usize,
    pub exploding: bool,
    timer: Timer,

}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            prev_y: 0,
            exploding: false,
            timer: Timer::new(Duration::from_millis(50)),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.finished() && ! self.exploding {
            if self.y > 0 { // remember y-axis is inverted here
                self.prev_y = self.y; // update prev_y record as shot moves
                self.y -= 1 ;
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::new(Duration::from_millis(250));
    }

    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.finished()) || (self.y == 0) // an example of a tail expression
        // no actual code, just evaluation
    }
}

impl Drawable for Shot { 
    fn draw(&self, frame: &mut Frame) {

        frame[self.x][self.y] = if self.exploding {" * "} else { " | "};

        //clear the old
        if self.prev_y != self.y {
            frame[self.x][self.prev_y] = "  ";
        }

        if self.y <=1 { 
            frame[self.x][self.y] = " ";
        }

        
    }
}
