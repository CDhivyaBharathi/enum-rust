//enum is used to enummerate all options possible for that variable.
// enum ipAddrType {
//     V4,
//     V6,
// }//this doesn't store the address (yet), so we use a struct.

// struct ipAddr {
//     kind: ipAddrType,
//     address: String,
// }

// fn main() {
//  let home = ipAddr {
//     kind: ipAddrType::V4, //namespaced using the enum name
//     address: String::from("127.0.0.1"),
//  };

//  let loopback = ipAddr {
//     kind: ipAddrType::V6,
//     address: String::from("::1"),
//  };
// }

//refactoring using just enum

// enum IpAddrType {
//     V4(String),
//     V6(String),
// }

// fn main(){
//     let home = IpAddrType::V4(String::from("127.0.0.1"));
//     let loopback = IpAddrType::V6(String::from("::1"));

//     let some_number = Some(5); // options automatically assumes type based on value inside some
//     let some_char = Some('e');

//     //some_number cannot be added with a i32 cuz rn ts Option<i32>, it should be 
//     //convered to i32 before adding.

//     //in order to use option<T>, we require code to handle some(T), that can use T 
//     // and code to handle when its None.

//     let absent_number: Option<i32> = None; // have to specify type for none.
// }

//match - Control flow construct

#[derive(Debug)]
enum us_state {
    Alabama,
    Alaska

}

impl us_state {
    fn existed_in(&self, year: u16) -> bool {
        match self{
            us_state::Alabama => year >= 1819,
            us_state::Alaska => year >= 1959,
        }
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(us_state), //only specific variants can have additional parameters.
}

fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Its a lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from state {state:?} ");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    //match must cover all possible posibilities or else it would become an error.
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main(){
    let coin = Coin::Quarter(us_state::Alabama);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other) //use _ => () [unit value] when you don't wanna use the value i.e. other.
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // the above can be refactored as the below using if let - need not handle the 
    // extra cases, just what you require
    if let Some(max) = config_max{
        println!("The maximum is configured to be {max}");
    }

    //using if let and else.
    let mut count = 0;
    if let Coin::Quarter(us_state) = coin {
        println!("Lucky Quarter");
    } else {
        count += 1;
    }

    let coin1 = Coin::Quarter(us_state::Alabama);
    let str: Option<String> = desc_state_qrtr(coin1);
    

    

}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

//using let..else
fn desc_state_qrtr(coin: Coin) -> Option<String>{
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900){
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new. "))
    }
}