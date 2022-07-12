#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Frame {
    Strike,
    Spare(u16, u16),
    Open(u16, u16),
}

pub struct BowlingGame {
    game: Vec<Frame>,
    roll: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            game: Vec::with_capacity(12),
            roll: Vec::with_capacity(2),
        }
    }

    fn next_frame(&mut self) {
        if self.roll[0] == 10 {
            self.game.push(Frame::Strike);
        } else if self.roll.iter().sum::<u16>() == 10 {
            self.game.push(Frame::Spare(self.roll[0], self.roll[1]));
        } else {
            self.game.push(Frame::Open(self.roll[0], self.roll[1]))
        }

        self.roll.clear();
    }

    fn game_over(&self) -> bool {
        if self.game.len() > 10 {
            return true;
        }

        if self.game.len() > 9 {
            let last_frame = self.game.last().unwrap();
            if matches!(last_frame, Frame::Open(..)) {
                return true;
            }
        }

        false
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.game_over() {
            return Err(Error::GameComplete);
        }

        if pins > 10 || self.roll.iter().sum::<u16>() + pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.roll.push(pins);
        if self.roll.len() == 2
            || pins == 10
            || (self.game.len() == 10 && self.game.last().unwrap() != &Frame::Strike)
        {
            self.next_frame();
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.game_over() {
            return None;
        }

        let mut padded_game = self.game.clone();
        if self.game.len() == 10 {
            padded_game.extend([Frame::Open(0, 0); 2].iter().copied());
        } else if self.game.len() == 11 {
            padded_game.extend([Frame::Open(0, 0); 1].iter().copied());
        }

        Some(padded_game.windows(3).fold(0, |score, frame| -> u16 {
            score
                + match frame[0] {
                    Frame::Strike => {
                        10 + match frame[1] {
                            Frame::Spare(r1, r2) | Frame::Open(r1, r2) => r1 + r2,
                            Frame::Strike => {
                                10 + match frame[2] {
                                    Frame::Spare(r1, _) | Frame::Open(r1, _) => r1,
                                    Frame::Strike => 10,
                                }
                            }
                        }
                    }
                    Frame::Spare(r1, r2) => {
                        r1 + r2
                            + match frame[1] {
                                Frame::Spare(r1, _) | Frame::Open(r1, _) => r1,
                                Frame::Strike => 10,
                            }
                    }
                    Frame::Open(r1, r2) => r1 + r2,
                }
        }))
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
