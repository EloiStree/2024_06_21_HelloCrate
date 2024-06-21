

//! # Eloi Crate
//! Hello guys,
//!  this is a simple crate that I created to test the creation of a crate in Rust.
//! This is my first one.
//! [![Hello Rust Bend Drone XR](https://raw.githubusercontent.com/EloiStree/HelloRustBending/main/Icon/RustDroneXR.png)](https://github.com/EloiStree/HelloRustBending)   
//! [https://github.com/EloiStree/HelloRustBending](https://github.com/EloiStree/HelloRustBending)    
//!  
//! May the code be with you :)

/// Test  in console debug the methode of gamepad to integer
pub fn test_gamepad_to_integer(){
    println!("Hello, world for Eloi Crate!");
    for _ in 0..10{
        let  input = gamepad_to_integer(
            19,
            get_random_float(),
            get_random_float(),
            get_random_float(),
            get_random_float()
        );
        println!("Input: {}", input);
    }

    println!("T:{}", gamepad_to_integer(19,0.0,0.0,0.0,0.0));
    println!("T:{}", gamepad_to_integer(19,1.0,0.0,0.0,0.0));
    println!("T:{}", gamepad_to_integer(19,0.0,1.0,0.0,0.0));
    println!("T:{}", gamepad_to_integer(19,0.0,0.0,1.0,0.0));
    println!("T:{}", gamepad_to_integer(19,0.0,0.0,0.0,1.0));
    println!("T:{}", gamepad_to_integer(19,1.0,1.0,1.0,1.0));
    println!("T:{}", gamepad_to_integer(19,1.0,1.0,0.0,0.0));
    println!("T:{}", gamepad_to_integer(19,0.0,0.0,1.0,1.0));
    
    println!("T:{}", gamepad_to_integer(-19,0.0,0.0,0.0,0.0));
    println!("T:{}", gamepad_to_integer(-19,1.0,0.0,0.0,0.0));
    println!("T:{}", gamepad_to_integer(-19,0.0,1.0,0.0,0.0));
    println!("T:{}", gamepad_to_integer(-19,0.0,0.0,1.0,0.0));
    println!("T:{}", gamepad_to_integer(-19,0.0,0.0,0.0,1.0));
    println!("T:{}", gamepad_to_integer(-19,1.0,1.0,1.0,1.0));
    println!("T:{}", gamepad_to_integer(-19,1.0,1.0,0.0,0.0));
    println!("T:{}", gamepad_to_integer(-19,0.0,0.0,1.0,1.0));
}


/// Returns a random float between -1.0 and 1.0
pub fn get_random_float() -> f32 {
    return (rand::random::<f32>() - 0.5) * 2.0;
}


/// Returns a integer that represents the gamepad input with front it,
/// left stick x, left stick y, right stick x, right stick y
///  (Right Rotation), (Down Up), (Left, Right), (Back, Front)
pub fn gamepad_to_integer(front_id:i8,lx:f32, ly:f32, rx:f32, ry:f32 )->i32
{

    let mut result:i32 =0;

    result+= parse_percent11_to_integer99(lx)*1000000;
    result+= parse_percent11_to_integer99(ly)*10000;
    result+= parse_percent11_to_integer99(rx)*100;
    result+= parse_percent11_to_integer99(ry)*1;

    if front_id<0{
        result += (-front_id as i32) * 100000000;
        result = result *-1;
    }
    else {
        result += (front_id as i32) * 100000000;
    }
    return result;
}


/// Converts a float between -1.0 and 1.0 to a integer between 0 and 99 digits
/// Quick Example
/// ```
/// let percent11_negative:i32 = be_eloistree_hellocrate::parse_percent11_to_integer99(-1.0);
/// assert_eq!(percent11_negative, 1);
/// let percent11_positive :i32= be_eloistree_hellocrate::parse_percent11_to_integer99(1.0);
/// assert_eq!(percent11_positive, 99);
/// let percent11_zero :i32= be_eloistree_hellocrate::parse_percent11_to_integer99(0.0);
/// assert_eq!(percent11_zero, 0);
/// ```
/// Source of this test: [https://youtu.be/4TI153PIEDQ?t=346](https://youtu.be/4TI153PIEDQ?t=346)
/// cargo test
/// cargo doc --open
pub fn parse_percent11_to_integer99(percent11:f32) -> i32{
    if percent11==0.00{
        return 0;
    }
    return 1+ (( ( (percent11+1.0)*0.5  ) * 98.0 ) ) as i32;
}


// Doc commentary
// https://youtu.be/4TI153PIEDQ?t=243



// cargo doc --open
// cargo test
// cargo login ciobelF5EbtvD0...Ei



pub use self::utils::random_println;

pub mod utils{
    use crate::gamepad_to_integer;
    use crate::get_random_float;


    pub fn random_println(){

        println!("Hello, world for Eloi Crate!");
        for _ in 0..10{
            let  input = gamepad_to_integer(
                19,
                get_random_float(),
                get_random_float(),
                get_random_float(),
                get_random_float()
            );
            println!("Input: {}", input);
        }

    }


}