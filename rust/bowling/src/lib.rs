#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Copy, Clone, PartialEq, Debug)]
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
        if self.game.len() == 10 {
            self.game
                .push(Frame::Open(self.roll[0], *self.roll.get(1).unwrap_or(&0)))
        } else if self.roll[0] == 10 {
            self.game.push(Frame::Strike);
        } else if self.roll.iter().sum::<u16>() == 10 {
            self.game.push(Frame::Spare(self.roll[0], self.roll[1]));
        } else {
            self.game.push(Frame::Open(self.roll[0], self.roll[1]));
        }

        self.roll.clear();
    }

    fn game_over(&self) -> bool {
        if self.game.len() >= 11 {
            return true;
        }

        if self.game.len() >= 10 {
            let last_frame = self.game.last().unwrap();
            return match last_frame {
                Frame::Strike => false,
                Frame::Spare(_, _) => false,
                Frame::Open(_, _) => true,
            };
        }

        false
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.game_over() {
            return Err(Error::GameComplete);
        }

        let played_frames = self.game.len();
        let last_frame = self.game.last();

        if (played_frames != 10 && self.roll.iter().sum::<u16>() + pins > 10)
            || (played_frames == 10
                && last_frame == Some(&Frame::Strike)
                && !self.roll.is_empty()
                && self.roll[0] != 10
                && self.roll.iter().sum::<u16>() + pins > 10)
        {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.roll.push(pins);

        if played_frames == 10 {
            if last_frame == Some(&Frame::Strike) {
                if self.roll.len() == 2 {
                    self.next_frame();
                }
            } else {
                self.next_frame();
            }
        } else if pins == 10 || self.roll.len() == 2 {
            self.next_frame()
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
        } else {
            padded_game.extend([Frame::Open(0, 0); 1].iter().copied());
        }

        Some(padded_game.windows(3).fold(0, |score, frame| -> u16 {
            let temp = score
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
                };
            temp
        }))
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
