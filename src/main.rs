//Class 200
fn is_item_in_stock(item_is_in_system: bool,item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_system {
        Option::Some(false)
    } else {
        Option::None
    } 
}
        
fn main() { //Class 200
   
    let availability = is_item_in_stock(true, true);

    match availability {
        Option::Some(true) => println!("Item is abailable!"),
        Option::Some(false) => println!("Item is not available!"),
        Option::None => println!("Item is not in the system!"),
    }

}

/*
 //Class 199
    let musical_instrument = [
        String::from("Guitar"),
        String::from("Piano"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = musical_instrument.get(2);
    play(bass);
    println!("{:?}", bass);

    let invalid_instrument = musical_instrument.get(100); // Class 198
    play(invalid_instrument);
}
fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => println!("Playing the {}!", instrument),
        Option::None => println!("Singing with my voice!"),
    }
*/
/*
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
 */
/*
// Class 196

    let a = Option::Some(10);
    let b = Option::Some("Hello");
    let c = Option::Some(true);

    let a: Option<i8> = Option::Some(10);
    let a = Option::<i16>::Some(10);

    let d = Option<&str> = Option::None;
*/
