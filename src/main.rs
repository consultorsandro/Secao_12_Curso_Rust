fn main() {
      let musical_instrument = [
        String::from("Guitar"),
        String::from("Piano"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = musical_instrument.get(2); 
    println!("Bass: {:?}", bass);
    let valid_instrument = bass.unwrap(); // Class 198
    println!("{}", valid_instrument);

    let invalid_instrument: Option<&String> = musical_instrument.get(3);
    println!("Invalid Instrument: {:?}", invalid_instrument);
    invalid_instrument.expect("This instrument does not exist in the list!"); // Class 198

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
