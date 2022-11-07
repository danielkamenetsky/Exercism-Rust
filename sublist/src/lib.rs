// Required for to_string method use
use std::fmt::Display;
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// Add is Display trait bound
pub fn sublist<T: PartialEq + Display>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // Turn list into iterator, map onto new list and convert to string then collect it
    let a_string = _first_list.iter().map(| x| x.to_string()).collect::<String>();
    let b_string = _second_list.iter().map(| x| x.to_string()).collect::<String>();

    if _first_list == _second_list {
        return Comparison::Equal
    }
    else if a_string.contains(&b_string) {
        return Comparison::Superlist
    }
    else if b_string.contains(&a_string) {
        return Comparison::Sublist
    }    
    else {

        Comparison::Unequal
    }
}