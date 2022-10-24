
//Need to import the two traits mentioned in README.md in order to use their methods
// using IntoEnumIterator trait from enum_iterator crate
use enum_iterator::IntoEnumIterator;
// using IntEnum trait from int_enum crate. 
use int_enum::IntEnum;
#[repr(usize)]
#[derive(Debug, PartialEq, Copy, Clone, IntEnum, IntoEnumIterator, Eq, Ord, PartialOrd)]

// Enum variants for resistor colors
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}
//Look up resistor color value using an IntEnum method
pub fn color_to_value(_color: ResistorColor) -> usize {
    ResistorColor::int_value(_color)
}
    

// Convert into a string representation of the color
pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => format!("{:?}", color),
        _ => String::from("value out of range"),
    }
}
pub fn colors() -> Vec<ResistorColor> {
    //collect uses the turbofish syntax here
    let mut collected = ResistorColor::into_enum_iter().collect::<Vec<ResistorColor>>();
    collected.sort();
    collected
}
