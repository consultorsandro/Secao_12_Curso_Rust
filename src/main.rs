fn divide(numerator: f64, denominator: f64) -> Result<f64, String> { // Class 206
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero!".to_string()))
    } else {
        Ok(numerator / denominator)
    }
}
fn main() {  // Class 207
    let result = divide(10.0, 0.0);
 // println!("{}", result.unwrap()); // This will panic if the result is an error. Ok from divide function.
 // println!("{}", result.unwrap_or(0.0)); // This will return 0.0 if the result is an error.
 // println!("{}", result.expect("Unable to parse calculation!")); 
 // println!("{}", result.is_ok()); // This will return false if the result is an error.
    println!("{}", result.is_err()); // This will return true if the result is an error.
    
}


/*
// Class 205
let text = "50";
let text_as_number = text.parse::<i32>();
println!("text_as_number: {:?}", text_as_number);  

let text = "Alabama";
let text_as_number = text.parse::<i32>();
println!("text_as_number: {:?}", text_as_number); // Class 205
  
*/
/*
    let ok: Result<i32, &str> = Ok(100); // Class 204
    println!("ok: {:?}", ok);
    let disaster: Result<i32, &str> = Err("Something went wrong!"); // Class 204
    println!("disaster: {:?}", disaster);
*/
/*
#[derive(Debug, Copy, Clone)] // Class 203

enum MyOption {
    Some(i32),
    None,
}
impl MyOption {
    fn unwrap(self) -> i32 {// Class 203
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh oh"),
        }
    }

    fn unwrap_or(self, fallback_value: i32) -> i32 { // Class 203
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

fn main() { 
    let some_option: MyOption = MyOption::Some(100); 
    println!("{}", some_option.unwrap_or(13)); // Responde melhor que o unwrap, pois não causa panic se for None

    let none_option: MyOption = MyOption::None; 
    println!("{}", none_option.unwrap_or(13));
}
*/
/*
{ //Class 201
   let present_value = Some(10);
   let missing_value: Option<i32> = None; // poderia ser qualquer tipo

   println!("{:?}", present_value.unwrap_or(0)); // Prints 10
   println!("{:?}", missing_value.unwrap_or(0)); // Prints 0
}
*/

/*
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

} */
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
