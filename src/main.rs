use std::env;
use std::str::FromStr;
use std::num::ParseIntError;
//unused for now


struct Roll {
   sides: u8,
   dice: u8,
   modifier: u8,
}

impl FromStr for Roll {
    type Err = ParseIntError;

    // Turns a query in the form of xxxdyyy+zzz into an
    // instance of 'Roll'

    fn from_str(query: &str) -> Result<Self, Self::Err> {
        let qualities: Vec<&str> = query.split(|p| p == 'd' || p == '+').collect();

        let sides_fromstr = qualities[0].parse::<u8>()?;
        let dice_fromstr = qualities[1].parse::<u8>()?;
        let modifier_fromstr = qualities[2].parse::<u8>()?;

        Ok(Roll{sides:sides_fromstr,dice:dice_fromstr,modifier:modifier_fromstr})
    }
}

fn roll_dice(sides:u8, quantity:u8, query:&str) {
    let mut result = 0;
    for _i in 0..quantity {
        result = result + rand::random::<u8>() % sides;
    }
    println!("{} is {}.", query, result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let query:&str = &args[1];
    let roll = Roll::from_str(query);

    println!("{}", roll.dice)
}
