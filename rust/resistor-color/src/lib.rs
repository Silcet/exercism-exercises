use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;
use std::string::ToString;
use std::fmt;

#[repr(usize)]
#[derive(Debug, PartialEq, Clone, Copy, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ResistorColor::Black => "Black",
            ResistorColor::Brown => "Brown",
            ResistorColor::Red => "Red",
            ResistorColor::Orange => "Orange",
            ResistorColor::Yellow => "Yellow",
            ResistorColor::Green => "Green",
            ResistorColor::Blue => "Blue",
            ResistorColor::Violet => "Violet",
            ResistorColor::Grey => "Grey",
            ResistorColor::White => "White"
        })
    }
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(x) => x.to_string(),
        Err(_) => "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
