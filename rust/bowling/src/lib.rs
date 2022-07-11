#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Frame {
    Strike,
    Spare(u16),
    Open(u16, u16),
}

pub struct BowlingGame {
    game: [Frame; 11],
    frame: usize,
    pins: u16,
    roll: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            game: [Frame::Open(0, 0); 11],
            frame: 0,
            pins: 10,
            roll: 0,
        }
    }

    fn next_frame(&mut self) {
        self.frame += 1;
        self.pins = 10;
        self.roll = 0;
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.roll += 1;

        if pins > 10 || pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }
        if (self.frame > 10 && self.game[10] != Frame::Strike) || self.frame > 11 {
            return Err(Error::GameComplete);
        }

        if pins == 10 {
            self.game[self.frame] = Frame::Strike;
            self.next_frame();
            return Ok(());
        } else if self.roll == 2 {
            self.game[self.frame] = if self.pins + pins == 0 {
                Frame::Spare(10 - self.pins)
            } else {
                Frame::Open(10 - self.pins, pins)
            };
        } else {
            self.pins -= pins;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame >= 10 {
            Some(0)
        } else {
            None
        }
    }
}
