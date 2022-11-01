// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

// Calculating the quotient and remainder of a division
pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let two_element = (dividend/divisor, dividend%divisor);
    two_element  
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    return iter.enumerate().filter(|(i,val)| i % 2 == 0).map(|(i, val)| val)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        return self.0.abs() + self.1.abs()
    }
}
