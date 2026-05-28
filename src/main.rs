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

enum IpAddrType {
    V4(String),
    V6(String),
}

fn main(){
    let home = IpAddrType::V4(String::from("127.0.0.1"));
    let loopback = IpAddrType::V6(String::from("::1"));

    let some_number = Some(5); // options automatically assumes type based on value inside some
    let some_char = Some('e');

    //some_number cannot be added with a i32 cuz rn ts Option<i32>, it should be 
    //convered to i32 before adding.

    let absent_number: Option<i32> = None; // have to specify type for none.
}