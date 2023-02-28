use std::mem::size_of;

use smart_string::SmartString;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn keep_size_of() {
    assert_eq!(size_of::<SmartString>(), size_of::<String>())
}