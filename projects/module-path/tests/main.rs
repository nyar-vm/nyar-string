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

#[test]
fn test() {
    let m1 = SmartString::try_managed("managed".to_string());
    let m2 = SmartString::try_managed("managed".to_string());
    println!("{:?}", m1);
    println!("{:?}", m2);

    unsafe {
        let i1 = SmartString::inlined("inlined string");
        let i2 = SmartString::inlined("inlined string");
        println!("{:?}", i1);
        println!("{:?}", i2);
    }

    unsafe {
        let h1 = SmartString::heap("heap string");
        let h2 = SmartString::heap("heap string");
        println!("{:?}", h1);
        println!("{:?}", h2);
    }
}
