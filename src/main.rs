fn main() {
    // Class 197
    let musical_instrument = [
        String::from("Guitar"),
        String::from("Piano"),
        String::from("Drums"),
    ];

    let drums: Option<&String> = musical_instrument.get(2);
    println!("Drums: {:?}", drums);

    let invalid_instrument: Option<&String> = musical_instrument.get(3);
    println!("Invalid Instrument: {:?}", invalid_instrument);
}
/*
// Class 196

    let a = Option::Some(10);
    let b = Option::Some("Hello");
    let c = Option::Some(true);

    let a: Option<i8> = Option::Some(10);
    let a = Option::<i16>::Some(10);

    let d = Option<&str> = Option::None;
*/
